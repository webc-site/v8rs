//! v8/src/base/lazy-instance.h 的 1:1 转写 — 首次访问构造的单实例懒管理
//! 原版 LazyInstance<Type, Traits> → Rust LazyInstance<T>:
//! - 静态分配模式 (StaticallyAllocatedInstanceTrait): 值内联于结构 (UnsafeCell<Option<T>>)
//! - 动态分配模式 (DynamicallyAllocatedInstanceTrait): LazyDynamicInstance<T>=懒持 Box<T>
//! - ThreadSafeInitOnceTrait: 复用 crate::once 的双检锁状态机 (OnceFlag 语义)
//! - SingleThreadInitOnceTrait: SingleThreadLazyInstance (非原子, 单线程专用)
//! - CreateTrait (Construct/Create): Rust 侧以闭包 get_with / T: Default 表达
//! - DestroyTrait/LeakyInstanceTrait: Rust RAII 自动析构; 原版故意不析构的
//!   LeakyObject 以 leak 变体保留语义 (差异登记)
//!
//! 注: 本快照无 LazyListening/依赖注册/invalidation 回调与 ThreadLocal 变体
//! (上游已移除/位于其他文件), 如上层迁移需要再登记补建

use std::cell::UnsafeCell;

use crate::once::{
  ONCE_STATE_DONE, ONCE_STATE_UNINITIALIZED, OnceType, call_once, call_once_impl, declare_once,
};

/// LAZY_STATIC_INSTANCE_INITIALIZER / LAZY_INSTANCE_INITIALIZER:
/// 静态懒实例的 const 初始化表达式 (原版 POD 初始化)
///
/// 存储布局: once 状态机 + 已分配未构造的值槽。
/// 快路径 (DONE 之后) 直接读 storage 是数据安全的:
/// 对 once 的 Acquire load 与初始化线程的 Release store(DONE) 同步,
/// happens-before 覆盖 storage 的写入 (与原版 placement-new 同款推理)。
pub struct LazyInstance<T> {
  once: OnceType,
  storage: UnsafeCell<Option<T>>,
}

// 原版 LazyInstanceImpl 线程安全 → Rust 显式允许跨线程共享
// SAFETY 前提: 所有 storage 访问都经 once 状态机同步 (见下方方法);
// get() 会向多线程分发 &T 共享引用, 故 Sync 除 T: Send (构造数据跨线程
// 可见) 外还要求 T: Sync (共享 &T 本身跨线程安全), 二者缺一不可
unsafe impl<T: Send + Sync> Sync for LazyInstance<T> {}
unsafe impl<T: Send> Send for LazyInstance<T> {}

impl<T> LazyInstance<T> {
  /// 对应 LAZY_INSTANCE_INITIALIZER (静态模式)
  #[inline]
  pub const fn new() -> Self {
    Self {
      once: declare_once(),
      storage: UnsafeCell::new(None),
    }
  }

  /// InitInstance 慢路径: CallOnce 保证 construct 至多执行一次
  fn init(&self, construct: impl FnOnce() -> T) {
    let slot = self.storage.get();
    call_once_impl(&self.once, move || {
      // SAFETY: CallOnceImpl 状态机保证此闭包全进程仅执行一次,
      // 且执行期间无其他读者 (快路径要求 DONE 状态, Release/Acquire 配对)
      unsafe { *slot = Some(construct()) };
    });
  }

  /// const T& Get() + DefaultConstructTrait: 默认构造路径
  pub fn get(&self) -> &T
  where
    T: Default,
  {
    if !self.once() {
      self.init(T::default);
    }
    // SAFETY: 快路径经 once==DONE 的 Acquire 读与写入端 Release 同步,
    // 值已构造且此后不再变更; 以共享引用暴露 (原版 const T&)
    unsafe { slot_ref(self.storage.get()) }
  }

  /// const T& Get() + 自定义 CreateTrait: 首次访问时用 construct 构造
  pub fn get_with(&self, construct: impl FnOnce() -> T) -> &T {
    self.init(construct);
    // SAFETY: 同上, init 返回即 DONE
    unsafe { slot_ref(self.storage.get()) }
  }

  /// T* Pointer(): 可变访问 (原版 Pointer() 非 const this, 但实例本身共享;
  /// Rust 侧要求 &mut 借用以保证引用独占)
  pub fn pointer(&mut self) -> &mut T
  where
    T: Default,
  {
    self.init(T::default);
    // SAFETY: &mut self 独占借用, 无其他访问者
    unsafe { slot_mut(self.storage.get()) }
  }

  /// 同 pointer, 自定义构造
  pub fn pointer_with(&mut self, construct: impl FnOnce() -> T) -> &mut T {
    self.init(construct);
    // SAFETY: 同上
    unsafe { slot_mut(self.storage.get()) }
  }

  /// 状态查询 (原版无公开接口, 供测试/上层 DCHECK 用)
  #[inline]
  pub fn is_initialized(&self) -> bool {
    self.once()
  }

  #[inline]
  fn once(&self) -> bool {
    use std::sync::atomic::Ordering;
    self.once.load(Ordering::Acquire) == ONCE_STATE_DONE
  }
}

impl<T> Default for LazyInstance<T> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

/// LazyStaticInstance<T, CreateTrait, ...>::type — Rust 即 LazyInstance<T>
/// (构造 trait 由 get_with 闭包替代)
pub type LazyStaticInstance<T> = LazyInstance<T>;

/// LazyDynamicInstance<T>::type — 动态分配模式: 懒构造并持有 Box<T>
/// 原版 DefaultCreateTrait (new T()) → 闭包 || Box::new(T::default())
pub type LazyDynamicInstance<T> = LazyInstance<Box<T>>;

// ---------------------------------------------------------------------------
// SingleThreadInitOnceTrait 对应物
// ---------------------------------------------------------------------------

/// 单线程懒实例 (原版 SingleThreadInitOnceTrait: 非原子 byte 状态机)
/// 不保证线程安全, 供明确单线程路径 (原 TLS 场景) 使用
pub struct SingleThreadLazyInstance<T> {
  // 原版 once_ 为普通 uint8 (非原子读取语义)
  once: UnsafeCell<u8>,
  storage: UnsafeCell<Option<T>>,
}

impl<T> SingleThreadLazyInstance<T> {
  #[inline]
  pub const fn new() -> Self {
    Self {
      once: UnsafeCell::new(ONCE_STATE_UNINITIALIZED),
      storage: UnsafeCell::new(None),
    }
  }

  /// if (*once == UNINITIALIZED) { function(storage); *once = DONE; }
  pub fn get_with(&self, construct: impl FnOnce() -> T) -> &T {
    // SAFETY: 类型契约 = 单线程使用, 无并发访问 (原版同款使用约束)
    unsafe {
      let once = &mut *self.once.get();
      if *once == ONCE_STATE_UNINITIALIZED {
        *self.storage.get() = Some(construct());
        *once = ONCE_STATE_DONE;
      }
      match (&*self.storage.get()).as_ref() {
        Some(v) => v,
        // 状态机保证不可达 (glossary: UNREACHABLE 白名单)
        None => unreachable!("SingleThreadLazyInstance slot empty after DONE"),
      }
    }
  }

  /// Get() 默认构造路径
  pub fn get(&self) -> &T
  where
    T: Default,
  {
    self.get_with(T::default)
  }
}
impl<T> Default for SingleThreadLazyInstance<T> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

// ---------------------------------------------------------------------------
// LeakyObject / DEFINE_LAZY_LEAKY_OBJECT_GETTER
// ---------------------------------------------------------------------------

/// 原版 LeakyObject<T>: 构造于栈内存储、故意不析构 (trivially destructible)
/// Rust 差异登记: 本类型随 owner 正常 Drop; 需要"C++ 式永不析构"
/// (静态变量场景) 时用 LeakyObject::leak 显式泄漏
pub struct LeakyObject<T> {
  value: T,
}

impl<T> LeakyObject<T>
where
  T: 'static,
{
  /// explicit LeakyObject(args...) — 构造即初始化
  #[inline]
  pub fn new(value: T) -> Self {
    Self { value }
  }

  /// T* get() — 可变访问 (原版返回裸指针; Rust 给引用)
  #[inline]
  pub fn get(&mut self) -> &mut T {
    &mut self.value
  }

  /// 原版 "静态局部 + 从不析构" 语义的显式实现:
  /// 泄漏分配得到 'static 可变引用 (调用点须保证仅初始化一次,
  /// 对应 C++11 magic statics 的线程安全初始化位置)
  pub fn leak(value: T) -> &'static mut T {
    Box::leak(Box::new(value))
  }
}

/// DEFINE_LAZY_LEAKY_OBJECT_GETTER(T, FunctionName, args...) 的 Rust 承接:
/// 上层以 `static X: LazyInstance<T> = LazyInstance::new();` 或
/// 一次性 `LeakyObject::leak(...)` 表达, 此处提供指针形态入口对应原版 T* 返回
///
/// # Safety
/// - 与原版一致: 返回指针指向的对象生命周期为进程级 (leak)
/// - 多线程下只应在单一初始化点调用 (原版 static 局部语义)
/// - T 须 'static: 泄漏出的引用存活期为进程级
pub unsafe fn leaky_object_getter<T: 'static>(value: T) -> *mut T {
  LeakyObject::leak(value) as *mut T
}

/// once.cc 状态机直连入口的封装重导出 (上层 LazyListening 类需求的前置)
#[inline]
pub fn lazy_call_once(once: &OnceType, init_func: impl FnOnce()) {
  call_once(once, init_func);
}

// ---------------------------------------------------------------------------
// 内部辅助
// ---------------------------------------------------------------------------

/// 读取已构造槽位 (调用方已保证 once==DONE 的 Acquire 同步或独占借用)
/// 生命周期 'a 由调用处推断收窄为实例借用期 (原版 Get() 返回 const T& 同款,
/// 引用有效期 = 懒实例存活期, 而非要求 T 本身 'static)
/// SAFETY: slot 必为 Some (CallOnceImpl 完成后写入)
#[inline]
unsafe fn slot_ref<'a, T>(slot: *mut Option<T>) -> &'a T {
  match unsafe { &*slot }.as_ref() {
    Some(v) => v,
    // 状态机保证不可达 (glossary: UNREACHABLE 白名单)
    None => unreachable!("LazyInstance slot empty after DONE"),
  }
}

/// 可变槽位访问 (调用方保证 &mut 独占借用)
/// SAFETY: slot 必为 Some; 生命周期同 slot_ref 由调用处收窄
#[inline]
unsafe fn slot_mut<'a, T>(slot: *mut Option<T>) -> &'a mut T {
  match unsafe { &mut *slot }.as_mut() {
    Some(v) => v,
    None => unreachable!("LazyInstance slot empty after DONE"),
  }
}
