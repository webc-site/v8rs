//! v8/src/base/functional/{bind-internal.h, function-ref.h} 的转写
//! 本快照 functional 目录仅含这两个头 (旧版 function-parameters.h/member.h/
//! maybe-const-args.h 已上游移除, 不在此迁移; Memoize 位于 objects/tagged-field.h,
//! 归 objects 层迁移, 本轮登记不实现)
//!
//! Rust 对应关系 (按 7.1 签名先行 + glossary):
//! - internal::TypeList<Args...>  → Rust 元组类型 (A, B, C) 直接承载参数列表
//! - ExtractArgs / ExtractReturnType (从函数签名抽取) → trait FunctionSignature
//! - FunctorTraits / DecayedFunctorTraits::Invoke → trait Invokable + 常量位
//!   (Rust 无重载 operator() 探测; HasNonOverloadedCallOp / HasOverloadedCallOp /
//!   IsComplete 是 C++ 模板重载决议机制, Rust 由 Fn/FnMut/FnOnce trait 天然覆盖,
//!   不转写运行时代码, 仅保留常量语义)
//! - base::FunctionRef<R(Args...)> → FunctionRef<'a, Args, R>:
//!   非 owning、无空状态、禁赋值 (Clone 允许), 零堆分配 (dyn 引用)

// ---------------------------------------------------------------------------
// ExtractArgs / ExtractReturnType (bind-internal.h)
// ---------------------------------------------------------------------------

use std::mem::size_of;

/// 从函数指针签名抽取返回类型与参数列表
/// 对应原版 `ExtractReturnType<R(Args...)>` 与 `ExtractArgs<R(Args...)>`
/// (Args 以元组承载, Rust 无变长模板参数)
pub trait FunctionSignature {
  type ReturnType;
  type Args;
}

macro_rules! impl_fn_signature {
  ($($name:ident),*) => {
    impl<R, $($name,)*> FunctionSignature for fn($($name),*) -> R {
      type ReturnType = R;
      type Args = ($($name,)*);
    }
  };
}

impl_fn_signature!(A);
impl_fn_signature!(A, B);
impl_fn_signature!(A, B, C);
impl_fn_signature!(A, B, C, D);

// ---------------------------------------------------------------------------
// FunctorTraits / DecayedFunctorTraits (bind-internal.h)
// ---------------------------------------------------------------------------

/// 原版 DecayedFunctorTraits 的四个常量位:
/// - is_method: Rust 成员方法指针不存在该形态 (闭包/self 引用直调), 恒 false
///   (原版 Methods 特化在 Rust 迁移中以 `&mut self` 方法表达)
/// - is_nullable: 函数指针可空性由 Option<fn> 类型显式表达
/// - is_callback: Callback 类型本轮未迁移
/// - is_stateless: 原版 std::is_empty_v<Functor> → 零大小类型判定
pub struct FunctorTraitsFlags;

impl FunctorTraitsFlags {
  /// is_stateless = std::is_empty_v<Functor>
  #[inline]
  pub const fn is_stateless<F>() -> bool {
    size_of::<F>() == 0
  }
  /// is_method (C++ 成员函数指针形态; Rust 侧上层不产生)
  pub const IS_METHOD: bool = false;
  /// is_nullable (仅函数指针 true; 由 Option<fn> 承载)
  pub const IS_NULLABLE: bool = false;
  /// is_callback
  pub const IS_CALLBACK: bool = false;
}

/// 原版 DecayedFunctorTraits<>::Invoke(forward(f), forward(args)...)
/// Rust 以 Fn trait 的单参 (元组) 形态统一入口
pub trait Invokable<A> {
  type ReturnType;
  fn invoke(&self, args: A) -> Self::ReturnType;
}

// Fn/A -> R 的 blanket 实现; 不与具体类型 impl 冲突 (sealed 无需)
impl<F: Fn(A) -> R, A, R> Invokable<A> for F {
  type ReturnType = R;
  #[inline]
  fn invoke(&self, args: A) -> R {
    self(args)
  }
}

// ---------------------------------------------------------------------------
// FunctionRef (function-ref.h)
// ---------------------------------------------------------------------------

/// 非 owning 的可调用对象引用 (原版 base::FunctionRef<R(Args...)>):
/// - 适合 ForEach/visitor 型同步调用, 不需要所有权
/// - 无空状态: 构造后即总可调用 (Rust 借用检查替代 LIFETIME_BOUND)
/// - 零堆分配: 内部仅一个 fat 引用; 原版禁止赋值 → 此处不提供 Copy,
///   Clone 语义 = 重新借用 (等价原版拷贝构造)
///
/// 原版禁止从其他 FunctionRef 构造: 类型参数即 dyn 引用, 天然无该路径
pub struct FunctionRef<'a, A, R> {
  f: &'a dyn Fn(A) -> R,
}

impl<'a, A, R> FunctionRef<'a, A, R> {
  /// 从任意兼容可调用对象借用构造
  /// (原版 template<Functor> requires kCompatibleFunctor 构造 +
  ///  function pointer 构造 — Rust 侧 &impl Fn 覆盖两者)
  #[inline]
  pub fn new<F: Fn(A) -> R + 'a>(functor: &'a F) -> Self {
    Self { f: functor }
  }

  /// 从 dyn Fn 引用直接承接 (协变场景)
  #[inline]
  pub fn from_ref(f: &'a dyn Fn(A) -> R) -> Self {
    Self { f }
  }

  /// R operator()(Args... args) const — #[must_use]: 原版返回值不可静默丢弃的
  /// 约束在 Rust 用 must_use 表达 (返回 void 场景用 () 仍触发 lint 提醒)
  #[inline]
  #[must_use]
  pub fn call(&self, args: A) -> R {
    (self.f)(args)
  }
}

impl<'a, A, R> Clone for FunctionRef<'a, A, R> {
  #[inline]
  fn clone(&self) -> Self {
    Self { f: self.f }
  }
}

impl<'a, A, R> Invokable<A> for FunctionRef<'a, A, R> {
  type ReturnType = R;
  #[inline]
  fn invoke(&self, args: A) -> R {
    self.call(args)
  }
}
