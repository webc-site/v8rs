//! v8/src/base/contextual.h 的 1:1 转写
//! ContextualVariable (CRTP + thread_local Scope* top_) → VariableSpec trait +
//! contextual_variable! 宏机械展开 (对应 DECLARE_CONTEXTUAL_VARIABLE)
//! 原版 Get() 无 Scope 时 DCHECK; Rust 禁 panic → get() 返回 Option
//! ContextualVariableWithDefault → contextual_variable_with_default! (get_or_default)
//! ContextualClass<T> → contextual_class!

use std::{
  cell::{Cell, UnsafeCell},
  ptr::null_mut,
};

/// 作用域结点: 原版 Scope { VarType value_; Scope* previous_; }
struct ScopeNode<T> {
  value: UnsafeCell<T>,
  prev: Option<*mut ScopeNode<T>>,
}

/// 变量规格 (CRTP Derived 的 trait 化): 每个 contextual variable 由宏生成一个实现
pub trait VariableSpec {
  type VarT;
  /// 访问本变量专属的 thread_local 栈顶指针槽
  fn with_top<R>(f: impl FnOnce(&Cell<*mut ()>) -> R) -> R;
}

/// RAII 作用域 (原版 V8_NODISCARD Scope): 构造压栈, Drop 弹栈, 栈纪律
pub struct Scope<V: VariableSpec> {
  node: Box<ScopeNode<V::VarT>>,
}

impl<V: VariableSpec> Scope<V> {
  /// explicit Scope(args...) — 以值构造并压栈
  #[must_use]
  pub fn new(value: V::VarT) -> Self {
    let prev = V::with_top(|top| {
      let raw = top.get() as *mut ScopeNode<V::VarT>;
      if raw.is_null() { None } else { Some(raw) }
    });
    // mut: 取 &raw mut 指针登记栈顶 (raw 指针不转移所有权)
    let mut node = Box::new(ScopeNode {
      value: UnsafeCell::new(value),
      prev,
    });
    // 记录栈顶为本 node
    let ptr: *mut ScopeNode<V::VarT> = &raw mut *node;
    V::with_top(|top| top.set(ptr.cast()));
    Scope { node }
  }

  /// VarType& Value()
  #[inline]
  pub fn value(&mut self) -> &mut V::VarT {
    // SAFETY: &mut self 唯一借出该 UnsafeCell
    unsafe { &mut *self.node.value.get() }
  }
}

impl<V: VariableSpec> Drop for Scope<V> {
  fn drop(&mut self) {
    let own: *mut ScopeNode<V::VarT> = &raw mut *self.node;
    V::with_top(|top| {
      // Ensure stack discipline (原版 DCHECK_EQ(this, Top()))
      debug_assert_eq!(top.get(), own.cast::<()>(), "contextual scope 违反栈纪律");
      top.set(
        self
          .node
          .prev
          .map(|p| p.cast::<()>())
          .unwrap_or_else(null_mut),
      );
    });
  }
}

/// static VarType& Get(): 当前栈顶值 (无 Scope → None, 原版 DCHECK(HasScope()))
#[inline]
pub fn get<V: VariableSpec>() -> Option<&'static mut V::VarT> {
  V::with_top(|top| {
    let raw = top.get() as *mut ScopeNode<V::VarT>;
    if raw.is_null() {
      return None;
    }
    // SAFETY: 非空栈顶必指向本线程活动 Scope 的 node;
    // 'static 妥协与原版裸引用等价 —— 使用方须保证在对应 Scope 存活期内访问
    unsafe { Some(&mut *(*raw).value.get()) }
  })
}

/// static bool HasScope()
#[inline]
pub fn has_scope<V: VariableSpec>() -> bool {
  V::with_top(|top| !top.get().is_null())
}

/// 原版 DECLARE_CONTEXTUAL_VARIABLE(VarName, VarType)
#[macro_export]
macro_rules! contextual_variable {
  ($vis:vis $name:ident, $ty:ty) => {
    /// 上下文变量: thread_local, 仅限同线程使用 (原版注释约束)
    $vis struct $name {}
    impl $crate::contextual::VariableSpec for $name {
      type VarT = $ty;
      #[inline]
      fn with_top<R>(f: impl FnOnce(&std::cell::Cell<*mut ()>) -> R) -> R {
        thread_local! {
          static TOP: std::cell::Cell<*mut ()> =
            const { std::cell::Cell::new(std::ptr::null_mut()) };
        }
        TOP.with(f)
      }
    }
  };
}

/// 原版 DECLARE_CONTEXTUAL_VARIABLE_WITH_DEFAULT(VarName, VarType, Args...)
/// 额外生成 inherent 方法 get_or_default()
#[macro_export]
macro_rules! contextual_variable_with_default {
  ($vis:vis $name:ident, $ty:ty, $default:expr) => {
    $crate::contextual_variable!($vis $name, $ty);
    impl $name {
      /// 原版 ContextualVariableWithDefault::Get()
      #[inline]
      pub fn get_or_default() -> &'static mut $ty {
        if let Some(val) = $crate::contextual::get::<$name>() {
          val
        } else {
          thread_local! {
            static DEFAULT: std::cell::UnsafeCell<$ty> =
              const { std::cell::UnsafeCell::new($default) };
          }
          // SAFETY: thread_local 单线程访问; 默认值仅在无 Scope 时暴露,
          // 与 Scope 值互斥 (原版同款约束)
          unsafe { &mut *DEFAULT.with(|d| d.get()) }
        }
      }
    }
  };
}

/// 原版 ContextualClass<T>: 类自身作为自己的 contextual variable
#[macro_export]
macro_rules! contextual_class {
  ($vis:vis $name:ident) => {
    $crate::contextual_variable!($vis $name, $name);
  };
}
