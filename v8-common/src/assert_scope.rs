// 由 tool/src/common.js 从 v8/src/common/assert-scope.h/.cc 生成; 修订就地改, 复跑覆盖
//
// 修订 (Developer): PerThreadAssertScope 落成泛型 RAII (const ALLOW/TYPES/ACTIVE),
// 线程本地位集进出配对; per-isolate scope 以 trait 留 L3 接入点; 23 个 using
// 别名全量可用; assert-scope.cc 的 static_assert 落编译期断言。
//
// 修订 (审查环 round1): D3 — NoGarbageCollectionMutexGuard 字段序对齐原版
// 成员逆序析构 (no_gc 先于 guard); D4 — 删 4 个虚构 per-isolate DebugOnly
// 符号 (PER_ISOLATE_CHECK_TYPE 不生成 DebugOnly, assert-scope.h:167-168)。
//
// 修订 (审查环 round2 findings): F6 — NoGarbageCollectionMutexGuard 原版
// 构造为加锁构造 (assert-scope.h:302-305 guard_(mutex)), 补 new(mutex);
// from_guard 降级为 Rust 扩展 (接管已持锁 guard, 登记)。
//
// 修订 (审查环 round5): F10 — NoGarbageCollectionMutexGuard 补 mutex 引用
// 字段 (对齐原版 mutex_ 成员, assert-scope.h:318), Lock() 改无参重锁同一把
// 锁 (h:311-314); 此前 lock(mutex) 要求调用方重传, 允许解锁 A 后重锁 B,
// 违反原版结构保证。from_guard 经 parking_lot MutexGuard::mutex() 取回
// 'a 生命周期父锁引用。

use core::{cell::Cell, marker::PhantomData};

use parking_lot::{Mutex, MutexGuard};

use crate::{
  globals::config::DEBUG_BOOL,
  internal::{K_DISALLOW_GARBAGE_COLLECTION_ALIGN, K_DISALLOW_GARBAGE_COLLECTION_SIZE},
};

/// 原版 enum PerThreadAssertType (assert-scope.h:22-39)
/// 位集下标 = 变体声明顺序
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PerThreadAssertType {
  /// 标记位: 恒置位表示数据有效, scope 不得直接读写 (assert-scope.h:23-27)
  AssertTypeIsValidMarker,
  SafepointsAssert,
  HeapAllocationAssert,
  HandleAllocationAssert,
  HandleDereferenceAssert,
  HandleUsageOnAllThreadsAssert,
  CodeDependencyChangeAssert,
  CodeAllocationAssert,
  /// 标记位: 关闭 GC mole
  GCMole,
  PositionInfoSlowAssert,
}

/// 单类型位掩码 (1 << 下标)
const fn bit(t: PerThreadAssertType) -> u32 {
  1 << t as u32
}

/// 原版 kInitialValue (assert-scope.cc:17-18) — 除
/// HANDLE_USAGE_ON_ALL_THREADS_ASSERT 外全允许, 标记位恒置
pub const K_INITIAL_VALUE: u32 = !(bit(PerThreadAssertType::HandleUsageOnAllThreadsAssert));
/// 原版 kClearedValue (assert-scope.cc:23) — 标记位清除态
pub const K_CLEARED_VALUE: u32 = 0;

// 原版 static_assert (assert-scope.cc:19, 24): K_INITIAL_VALUE 含标记位;
// K_CLEARED_VALUE = 0 由定义即不含标记位 (原版断言在 Rust 为恒真式, 不落位)
const _: () = assert!(K_INITIAL_VALUE & bit(PerThreadAssertType::AssertTypeIsValidMarker) != 0);

// 原版 thread_local PerThreadAsserts current_per_thread_assert_data
// (assert-scope.cc:27)
thread_local! {
  static CURRENT_PER_THREAD_ASSERT_DATA: Cell<u32> = const { Cell::new(K_INITIAL_VALUE) };
}

/// 线程本位断言数据快照 (contains_all 判定)
#[inline]
pub fn per_thread_asserts_allow(types: u32) -> bool {
  CURRENT_PER_THREAD_ASSERT_DATA.with(|d| d.get() & types == types)
}

/// 原版 PerThreadAsserts = base::EnumSet<PerThreadAssertType, uint32_t>
/// (assert-scope.h:41) — Rust 以裸位集 u32 承载, 组合用 `|`
pub type PerThreadAsserts = u32;

/// 原版 PerThreadAssertScope (assert-scope.h:58-74, assert-scope.cc:31-61)
///
/// - `ALLOW = true`: 授予例外 (置位); `false`: 施加禁止 (清位)
/// - `TYPES`: 类型位集掩码 (多个类型用 `|` 组合)
/// - `ACTIVE`: false 时整个作用域零开销空转, 对应 C++
///   PerThreadAssertScopeDebugOnly 在 release 下的 PerThreadAssertScopeEmpty
///
/// 非 Copy 非 Clone — RAII 进出配对由所有权保证 (C++ 删除了拷贝构造)
///
/// 审查环 round3 (F7): 线程绑定 — 原版作用域删除拷贝构造且无移动构造
/// (assert-scope.h:65-66), 实体栈绑定, 不可跨线程迁移; Rust `Cell<u32>`
/// 载体会派生 Send, 作用域可被 move 至他线程, drop 时 old_data 回写当前
/// 线程 TLS 而构造线程永久残留作用域值。以 `PhantomData<*const ()>` 复现
/// 原版线程绑定 (!Send), 23 个别名随之收紧。
pub struct PerThreadAssertScope<const ALLOW: bool, const TYPES: u32, const ACTIVE: bool> {
  /// 原版 old_data_ (assert-scope.h:73)
  old_data: Cell<u32>,
  /// 线程绑定标记 — *const () 非 Send, 类型随之 !Send
  _thread_bound: PhantomData<*const ()>,
}

impl<const ALLOW: bool, const TYPES: u32, const ACTIVE: bool>
  PerThreadAssertScope<ALLOW, TYPES, ACTIVE>
{
  /// 原版 PerThreadAssertScope() 构造 (assert-scope.cc:31-43)
  #[inline]
  pub fn new() -> Self {
    // 原版构造内 static_assert (assert-scope.cc:34-36):
    // TYPES 不得含 ASSERT_TYPE_IS_VALID_MARKER
    const {
      assert!(
        TYPES & bit(PerThreadAssertType::AssertTypeIsValidMarker) == 0,
        "PerThreadAssertScope types should not include the ASSERT_TYPE_IS_VALID_MARKER",
      );
    }
    let scope = Self {
      old_data: Cell::new(K_CLEARED_VALUE),
      _thread_bound: PhantomData,
    };
    if ACTIVE {
      CURRENT_PER_THREAD_ASSERT_DATA.with(|d| {
        let old = d.get();
        debug_assert!(old & bit(PerThreadAssertType::AssertTypeIsValidMarker) != 0);
        d.set(if ALLOW { old | TYPES } else { old & !TYPES });
        scope.old_data.set(old);
      });
    }
    scope
  }

  /// 原版 IsAllowed() (assert-scope.cc:58-61) — 全部位均允许才真
  #[inline]
  pub fn is_allowed() -> bool {
    per_thread_asserts_allow(TYPES)
  }

  /// 原版 Release() (assert-scope.cc:50-55) — 手动恢复, 幂等
  #[inline]
  pub fn release(&self) {
    if ACTIVE && self.old_data.get() != K_CLEARED_VALUE {
      CURRENT_PER_THREAD_ASSERT_DATA.with(|d| d.set(self.old_data.get()));
      self.old_data.set(K_CLEARED_VALUE);
    }
  }
}

impl<const ALLOW: bool, const TYPES: u32, const ACTIVE: bool> Default
  for PerThreadAssertScope<ALLOW, TYPES, ACTIVE>
{
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

/// 原版析构 ~PerThreadAssertScope() (assert-scope.cc:45-48)
impl<const ALLOW: bool, const TYPES: u32, const ACTIVE: bool> Drop
  for PerThreadAssertScope<ALLOW, TYPES, ACTIVE>
{
  #[inline]
  fn drop(&mut self) {
    self.release();
  }
}

/// 原版 PerThreadAssertScopeEmpty (assert-scope.h:49-56) — release 空作用域
pub type PerThreadAssertScopeEmpty<const ALLOW: bool, const TYPES: u32> =
  PerThreadAssertScope<ALLOW, TYPES, false>;

/// 原版 PerThreadAssertScopeDebugOnly (assert-scope.h:170-177)
/// DEBUG_BOOL 开 → 真作用域, 关 → 空作用域
pub type PerThreadAssertScopeDebugOnly<const ALLOW: bool, const TYPES: u32> =
  PerThreadAssertScope<ALLOW, TYPES, { DEBUG_BOOL }>;

// ── 原版 using 别名 (assert-scope.h:179-288, 23 条全量) ────────────────────
// 位掩码与原版模板实参 1:1。

pub type DisallowHandleAllocation =
  PerThreadAssertScopeDebugOnly<false, { bit(PerThreadAssertType::HandleAllocationAssert) }>;
pub type AllowHandleAllocation =
  PerThreadAssertScopeDebugOnly<true, { bit(PerThreadAssertType::HandleAllocationAssert) }>;
pub type DisallowSafepoints =
  PerThreadAssertScopeDebugOnly<false, { bit(PerThreadAssertType::SafepointsAssert) }>;
pub type AllowSafepoints =
  PerThreadAssertScopeDebugOnly<true, { bit(PerThreadAssertType::SafepointsAssert) }>;
pub type DisallowHeapAllocation =
  PerThreadAssertScopeDebugOnly<false, { bit(PerThreadAssertType::HeapAllocationAssert) }>;
pub type AllowHeapAllocation =
  PerThreadAssertScopeDebugOnly<true, { bit(PerThreadAssertType::HeapAllocationAssert) }>;
/// 原版 AllowHeapAllocationInRelease — release 亦生效 (assert-scope.h:205)
pub type AllowHeapAllocationInRelease =
  PerThreadAssertScope<true, { bit(PerThreadAssertType::HeapAllocationAssert) }, true>;
pub type DisallowHandleDereference =
  PerThreadAssertScopeDebugOnly<false, { bit(PerThreadAssertType::HandleDereferenceAssert) }>;
pub type AllowHandleDereference =
  PerThreadAssertScopeDebugOnly<true, { bit(PerThreadAssertType::HandleDereferenceAssert) }>;
pub type AllowHandleUsageOnAllThreads =
  PerThreadAssertScopeDebugOnly<true, { bit(PerThreadAssertType::HandleUsageOnAllThreadsAssert) }>;
pub type DisallowCodeDependencyChange =
  PerThreadAssertScopeDebugOnly<false, { bit(PerThreadAssertType::CodeDependencyChangeAssert) }>;
pub type AllowCodeDependencyChange =
  PerThreadAssertScopeDebugOnly<true, { bit(PerThreadAssertType::CodeDependencyChangeAssert) }>;
pub type DisallowCodeAllocation =
  PerThreadAssertScopeDebugOnly<false, { bit(PerThreadAssertType::CodeAllocationAssert) }>;
pub type AllowCodeAllocation =
  PerThreadAssertScopeDebugOnly<true, { bit(PerThreadAssertType::CodeAllocationAssert) }>;
/// 原版 DisallowGarbageCollection — safepoints + heap allocation 双位
pub type DisallowGarbageCollection = PerThreadAssertScopeDebugOnly<
  false,
  { bit(PerThreadAssertType::SafepointsAssert) | bit(PerThreadAssertType::HeapAllocationAssert) },
>;
pub type DisallowGarbageCollectionInRelease = PerThreadAssertScope<
  false,
  { bit(PerThreadAssertType::SafepointsAssert) | bit(PerThreadAssertType::HeapAllocationAssert) },
  true,
>;
pub type DisableGCMole = PerThreadAssertScopeDebugOnly<false, { bit(PerThreadAssertType::GCMole) }>;
pub type DisallowPositionInfoSlow =
  PerThreadAssertScopeDebugOnly<false, { bit(PerThreadAssertType::PositionInfoSlowAssert) }>;
pub type AllowPositionInfoSlow =
  PerThreadAssertScopeDebugOnly<true, { bit(PerThreadAssertType::PositionInfoSlowAssert) }>;
pub type AllowGarbageCollection = PerThreadAssertScopeDebugOnly<
  true,
  { bit(PerThreadAssertType::SafepointsAssert) | bit(PerThreadAssertType::HeapAllocationAssert) },
>;
pub type AllowGarbageCollectionInRelease = PerThreadAssertScope<
  true,
  { bit(PerThreadAssertType::SafepointsAssert) | bit(PerThreadAssertType::HeapAllocationAssert) },
  true,
>;
pub type DisallowHeapAccess = PerThreadAssertScopeDebugOnly<
  false,
  {
    bit(PerThreadAssertType::CodeDependencyChangeAssert)
      | bit(PerThreadAssertType::HandleDereferenceAssert)
      | bit(PerThreadAssertType::HandleAllocationAssert)
      | bit(PerThreadAssertType::HeapAllocationAssert)
  },
>;
pub type AllowHeapAccess = PerThreadAssertScopeDebugOnly<
  true,
  {
    bit(PerThreadAssertType::CodeDependencyChangeAssert)
      | bit(PerThreadAssertType::HandleDereferenceAssert)
      | bit(PerThreadAssertType::HandleAllocationAssert)
      | bit(PerThreadAssertType::HeapAllocationAssert)
  },
>;

// 原版 assert-scope.cc:106-131 显式实例化清单 (21 条 per-thread) — Rust 单个
// 泛型实现即覆盖全部实例化, 无需逐条登记。

// 原版 static_assert (assert-scope.cc:133-136): Internals 对齐/尺寸约束
// (DisallowGarbageCollectionInRelease 持 u32 old_data)
const _: () = assert!(K_DISALLOW_GARBAGE_COLLECTION_ALIGN == 4);
const _: () = assert!(K_DISALLOW_GARBAGE_COLLECTION_SIZE == 4);
const _: () = assert!(
  size_of::<DisallowGarbageCollectionInRelease>() == K_DISALLOW_GARBAGE_COLLECTION_SIZE
    && align_of::<DisallowGarbageCollectionInRelease>() == K_DISALLOW_GARBAGE_COLLECTION_ALIGN
);

// ── per-isolate 断言作用域 (assert-scope.h:76-168, L3 接入点) ───────────────

/// 原版 PER_ISOLATE_DCHECK_TYPE / PER_ISOLATE_CHECK_TYPE 的六组三元组
/// (allow 类, disallow 类, isolate 字段) — 生成物表保留登记
pub const PER_ISOLATE_ASSERT_TYPES: [(&str, &str, &str); 6] = [
  (
    "AllowJavascriptExecution",
    "DisallowJavascriptExecution",
    "javascript_execution_assert",
  ),
  (
    "AllowDeoptimization",
    "DisallowDeoptimization",
    "deoptimization_assert",
  ),
  (
    "AllowCompilation",
    "DisallowCompilation",
    "compilation_assert",
  ),
  (
    "AllowExceptions",
    "DisallowExceptions",
    "no_exception_assert",
  ),
  (
    "NoThrowOnJavascriptExecution",
    "ThrowOnJavascriptExecution",
    "javascript_execution_throws",
  ),
  (
    "NoDumpOnJavascriptExecution",
    "DumpOnJavascriptExecution",
    "javascript_execution_dump",
  ),
];

/// per-isolate 断言类别 (assert-scope.cc:63-101 的 field 枚举化)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PerIsolateAssertType {
  JavascriptExecutionAssert,
  DeoptimizationAssert,
  CompilationAssert,
  NoExceptionAssert,
  JavascriptExecutionThrows,
  JavascriptExecutionDump,
}

/// L3 接入点 (原版 isolate->field() / isolate->set_field()):
/// isolate 断言字段的读写由上层 isolate 实现承载
pub trait PerIsolateAssertState {
  fn get(&self, ty: PerIsolateAssertType) -> bool;
  fn set(&mut self, ty: PerIsolateAssertType, enable: bool);
}

/// 原版 PER_ISOLATE_ASSERT_SCOPE_DECLARATION + _DEFINITION (assert-scope.h:104-123,
/// assert-scope.cc:63-88)
///
/// `ENABLE = true` → Allow*; `ENABLE = false` → Disallow*。
/// 构造保存旧值并写入 enable, 析构恢复 (RAII)。
pub struct PerIsolateAssertScope<'a, const ENABLE: bool> {
  isolate: &'a mut dyn PerIsolateAssertState,
  ty: PerIsolateAssertType,
  old_data: bool,
}

impl<'a, const ENABLE: bool> PerIsolateAssertScope<'a, ENABLE> {
  /// 原版 ScopeType(Isolate*) (assert-scope.cc:66-70)
  pub fn new(isolate: &'a mut dyn PerIsolateAssertState, ty: PerIsolateAssertType) -> Self {
    let old_data = isolate.get(ty);
    isolate.set(ty, ENABLE);
    Self {
      isolate,
      ty,
      old_data,
    }
  }

  /// 原版 IsAllowed(isolate) (assert-scope.cc:75)
  pub fn is_allowed(isolate: &dyn PerIsolateAssertState, ty: PerIsolateAssertType) -> bool {
    isolate.get(ty)
  }

  /// 原版 Open(isolate, &was_execution_allowed) (assert-scope.cc:78-83)
  pub fn open(isolate: &mut dyn PerIsolateAssertState, ty: PerIsolateAssertType) -> bool {
    let was = isolate.get(ty);
    isolate.set(ty, ENABLE);
    was
  }

  /// 原版 Close(isolate, was_execution_allowed) (assert-scope.cc:85-88)
  pub fn close(isolate: &mut dyn PerIsolateAssertState, ty: PerIsolateAssertType, was: bool) {
    isolate.set(ty, was);
  }
}

impl<const ENABLE: bool> Drop for PerIsolateAssertScope<'_, ENABLE> {
  /// 原版 ~ScopeType (assert-scope.cc:72)
  fn drop(&mut self) {
    self.isolate.set(self.ty, self.old_data);
  }
}

/// 原版 DisallowHeapAccessIf (assert-scope.h:290-298) — 条件启用
pub struct DisallowHeapAccessIf {
  pub maybe_disallow: Option<DisallowHeapAccess>,
}

impl DisallowHeapAccessIf {
  pub fn new(condition: bool) -> Self {
    Self {
      maybe_disallow: condition.then(DisallowHeapAccess::new),
    }
  }
}

/// 原版 NoGarbageCollectionMutexGuard (assert-scope.h:302-320) —
/// 持锁期间禁 GC; Unlock/Lock 支持中途临时放锁
///
/// 审查环 round1 (D3): C++ 成员逆序析构 — 声明序 guard_, mutex_, no_gc_
/// → 析构序 no_gc_ (恢复 GC 断言位) 先于 guard_ (解锁), 即恢复断言位时
/// 互斥锁仍持有; Rust 字段按声明序 drop, 故 no_gc 声明在前、guard 在后,
/// 1:1 对齐原版析构序
///
/// 审查环 round5 (F10): 原版以 mutex_ 成员记忆持锁目标 (h:318), Lock()
/// 无参重锁**同一把锁** (h:311-314); 此前 Rust 版 lock 要求调用方重传
/// mutex — 允许解锁 A 后重锁 B, 违反原版结构保证。mutex 引用落字段,
/// from_guard 经 parking_lot `MutexGuard::mutex()` 恢复 'a 生命周期引用
pub struct NoGarbageCollectionMutexGuard<'a, T> {
  no_gc: Option<DisallowGarbageCollection>,
  guard: Option<MutexGuard<'a, T>>,
  /// 原版 mutex_ (assert-scope.h:318) — Lock() 重锁目标
  mutex: &'a Mutex<T>,
}

impl<'a, T> NoGarbageCollectionMutexGuard<'a, T> {
  /// 原版构造 (assert-scope.h:302-305) — `guard_(mutex)` 即**加锁**构造:
  /// 锁 mutex 并进入禁 GC 断言 (round2-F6 注释纠偏: 此前误注 "锁已由
  /// 调用方持有"; 原版成员序 guard_ → mutex_ → no_gc_, 构造先取锁)
  pub fn new(mutex: &'a Mutex<T>) -> Self {
    let guard = mutex.lock();
    Self {
      no_gc: Some(DisallowGarbageCollection::new()),
      guard: Some(guard),
      mutex,
    }
  }

  /// Rust 扩展 (原版无对应, 登记): 接管调用方已持有的 guard — 供已持锁
  /// 路径免二次加锁; 原版构造 (assert-scope.h:302-305) 始终自行加锁,
  /// 加锁形态见 `new`。父锁引用经 `MutexGuard::mutex()` 取回 (lock_api
  /// 返回 'a 生命周期引用, 与 guard 消费无关)
  pub fn from_guard(guard: MutexGuard<'a, T>) -> Self {
    let mutex = MutexGuard::mutex(&guard);
    Self {
      no_gc: Some(DisallowGarbageCollection::new()),
      mutex,
      guard: Some(guard),
    }
  }

  /// 原版 Unlock() (assert-scope.h:307-310) — 先解锁再恢复 GC (序 1:1)
  pub fn unlock(&mut self) {
    self.guard = None;
    self.no_gc = None;
  }

  /// 原版 Lock() (assert-scope.h:311-314) — 无参重锁构造时锁定的**同一把锁**
  pub fn lock(&mut self) {
    self.guard = Some(self.mutex.lock());
    self.no_gc = Some(DisallowGarbageCollection::new());
  }
}

// ── 8 个 PER_ISOLATE_*_DebugOnly 别名 (assert-scope.h:125-169) ─────────────
// DEBUG 构建继承对应真作用域 (进入/析构切换 isolate 断言字段), release 构建
// 为空壳 (无状态无效果)。无 isolate 的默认构造形态依赖原版 gCurrentIsolate
// 线程局部 (assert-scope.cc:29-31), L3 isolate 接入前不落位 (登记)。
//
// 审查环 round1 (D4): 原版仅 PER_ISOLATE_DCHECK_TYPE 过 DebugOnly 生成宏
// (assert-scope.h:167-168) → 8 个 (4 Allow* + 4 Disallow*);
// PER_ISOLATE_CHECK_TYPE (NoThrow/Throw/NoDump/Dump) 不生成 DebugOnly —
// 早前转写的 4 个 *OnJavascriptExecutionDebugOnly / *DumpOnJavascriptExecution
// DebugOnly 系虚构符号, 已删。

macro_rules! per_isolate_debug_only_scope {
  ($name:ident, $enable:literal, $ty:expr, $doc:literal) => {
    #[doc = $doc]
    pub struct $name<'a> {
      /// DEBUG_BOOL 开 → 真作用域, 关 → None (release 空壳)
      pub inner: Option<PerIsolateAssertScope<'a, $enable>>,
    }

    impl<'a> $name<'a> {
      /// 原版 ScopeType##DebugOnly(Isolate*)
      #[inline]
      pub fn new(isolate: &'a mut dyn PerIsolateAssertState) -> Self {
        Self {
          inner: if DEBUG_BOOL {
            Some(PerIsolateAssertScope::new(isolate, $ty))
          } else {
            None
          },
        }
      }
    }
  };
}

per_isolate_debug_only_scope!(
  AllowJavascriptExecutionDebugOnly,
  true,
  PerIsolateAssertType::JavascriptExecutionAssert,
  "原版 AllowJavascriptExecutionDebugOnly (assert-scope.h:127-129)"
);
per_isolate_debug_only_scope!(
  DisallowJavascriptExecutionDebugOnly,
  false,
  PerIsolateAssertType::JavascriptExecutionAssert,
  "原版 DisallowJavascriptExecutionDebugOnly (assert-scope.h:142-144)"
);
per_isolate_debug_only_scope!(
  AllowDeoptimizationDebugOnly,
  true,
  PerIsolateAssertType::DeoptimizationAssert,
  "原版 AllowDeoptimizationDebugOnly (assert-scope.h:127-129 展开)"
);
per_isolate_debug_only_scope!(
  DisallowDeoptimizationDebugOnly,
  false,
  PerIsolateAssertType::DeoptimizationAssert,
  "原版 DisallowDeoptimizationDebugOnly (assert-scope.h:142-144 展开)"
);
per_isolate_debug_only_scope!(
  AllowCompilationDebugOnly,
  true,
  PerIsolateAssertType::CompilationAssert,
  "原版 AllowCompilationDebugOnly (assert-scope.h:127-129 展开)"
);
per_isolate_debug_only_scope!(
  DisallowCompilationDebugOnly,
  false,
  PerIsolateAssertType::CompilationAssert,
  "原版 DisallowCompilationDebugOnly (assert-scope.h:142-144 展开)"
);
per_isolate_debug_only_scope!(
  AllowExceptionsDebugOnly,
  true,
  PerIsolateAssertType::NoExceptionAssert,
  "原版 AllowExceptionsDebugOnly (assert-scope.h:127-129 展开)"
);
per_isolate_debug_only_scope!(
  DisallowExceptionsDebugOnly,
  false,
  PerIsolateAssertType::NoExceptionAssert,
  "原版 DisallowExceptionsDebugOnly (assert-scope.h:142-144 展开)"
);
