//! v8/src/base/template-meta-programming/{common,functional,list,string-literal}.h
//! 的转写。
//! 映射说明 (本快照 v8/src 内 tmp:: 仅有自身头文件间引用, 无其他消费方):
//! - C++ 类型级 list<Args...> → 值级 TypeList (TypeId 序列)。类型级操作
//!   (element/map 返回 *类型*) 在 stable Rust 无法以类型函数表达, 提供
//!   element_is/index_of/contains 等判定形态 (上层实际出现再按需求单态化)。
//! - list1 (一元模板参数序列)/equals1/instantiate/is_instantiation_of/
//!   call_foreach: 依赖 HKT (higher-kinded types), stable 无法表达, 推迟登记。
//! - tmp::equals<T,U>::value → type_equals::<T,U>() (TypeId 判等; 注意
//!   TypeId 会擦除 ref/cv 修饰, 与 C++ equals 语义在指针/引用场景有差异)。
//! - call_parameters<R(*)(Args...)> → list<Args...>: Rust fn 指针参数为
//!   单个元组类型且无法在 stable 展开 → 以 tmp_list![A,B,C] 由调用方显式
//!   列参数类型 (登记差异)。
//! - StringLiteral<N> (class-type NTTP) → &'static str 包装 (stable 无
//!   const-generic str 参数)。

use std::{any::TypeId, marker::PhantomData};

/// 对应 tmp::lazy_false<T>::value — 依赖型 false (静态断言两阶段查找用)。
/// Rust 泛型单态化不存在该问题, 保留同形标记供上层对照。
pub struct LazyFalse<T>(PhantomData<T>);
impl<T> LazyFalse<T> {
  pub const VALUE: bool = false;
}

/// 对应 tmp::lazy_true<T>::value
pub struct LazyTrue<T>(PhantomData<T>);
impl<T> LazyTrue<T> {
  pub const VALUE: bool = true;
}

/// 对应 tmp::equals<T, U>::value (TypeId 判等, 双方需 'static)
#[inline]
pub fn type_equals<T: 'static, U: 'static>() -> bool {
  TypeId::of::<T>() == TypeId::of::<U>()
}

/// 对应 tmp::list<Args...>: 值级类型列表。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeList {
  ids: Vec<TypeId>,
}

impl TypeList {
  /// 空列表
  #[inline]
  pub const fn new() -> Self {
    Self { ids: Vec::new() }
  }

  /// 由 TypeId 切片构造 (对应 map_ids 等结果落地)
  #[inline]
  pub fn from_ids(ids: &[TypeId]) -> Self {
    Self { ids: ids.to_vec() }
  }

  /// 对应 tmp::length_v<List>
  #[inline]
  pub fn length(&self) -> usize {
    self.ids.len()
  }

  /// 对应 tmp::element<List, I>::type 的判定形态: 第 I 个元素是否为 T
  #[inline]
  pub fn element_is<T: 'static>(&self, index: usize) -> bool {
    self.ids.get(index) == Some(&TypeId::of::<T>())
  }

  /// 第 I 个元素的 TypeId (越界 None; 原版 element 越界为编译错误,
  /// 调用方以 debug_assert 兜底)
  #[inline]
  pub fn element_id(&self, index: usize) -> Option<TypeId> {
    self.ids.get(index).copied()
  }

  /// 对应 tmp::index_of_v<List, T, Otherwise>: 首个匹配下标, 否则 otherwise
  #[inline]
  pub fn index_of<T: 'static>(&self, otherwise: usize) -> usize {
    let target = TypeId::of::<T>();
    for (idx, id) in self.ids.iter().enumerate() {
      if *id == target {
        return idx;
      }
    }
    otherwise
  }

  /// 缺省 Otherwise = size_t::max (原版 std::numeric_limits<size_t>::max())
  #[inline]
  pub fn index_of_default<T: 'static>(&self) -> usize {
    self.index_of::<T>(usize::MAX)
  }

  /// 对应 tmp::contains_v<List, T>
  #[inline]
  pub fn contains<T: 'static>(&self) -> bool {
    self.index_of::<T>(usize::MAX) != usize::MAX
  }

  /// 对应 tmp::all_equal_v<List, Cmp=equals>: 全部元素与首元素判等
  /// (原版以 Head 为基准逐个 Cmp<Head, Tail>; 空表与单元素表为 true)
  #[inline]
  pub fn all_equal(&self) -> bool {
    match self.ids.first() {
      Some(head) => self.ids.iter().all(|id| id == head),
      None => true,
    }
  }

  /// 对应 tmp::append_t<List, Ts...>: 追加类型 T, 返回新列表
  #[inline]
  pub fn append_type<T: 'static>(&self) -> Self {
    let mut out = self.clone();
    out.ids.push(TypeId::of::<T>());
    out
  }

  /// 追加另一列表 (list 拼接)
  #[inline]
  pub fn append(&self, other: &Self) -> Self {
    let mut out = self.clone();
    out.ids.extend_from_slice(&other.ids);
    out
  }

  /// 对应 tmp::insert_at_t<List, I, T>: 在 I 处插入 T;
  /// 原版 I 超过长度时直接追加 → 同语义
  #[inline]
  pub fn insert_at<T: 'static>(&self, index: usize) -> Self {
    let mut out = self.clone();
    let at = index.min(out.ids.len());
    out.ids.insert(at, TypeId::of::<T>());
    out
  }

  /// 底层 TypeId 切片视图 (map_ids/all_of_ids 等闭包操作复用)
  #[inline]
  pub fn ids(&self) -> &[TypeId] {
    &self.ids
  }

  /// 对应 tmp::map<F, List> 的值级等价: 对每个元素 TypeId 应用 f。
  /// (原版 F<T>::type 为类型→类型; 此处 TypeId→TypeId, 登记映射差异)
  #[inline]
  pub fn map_ids(&self, f: impl Fn(TypeId) -> TypeId) -> Self {
    Self {
      ids: self.ids.iter().copied().map(f).collect(),
    }
  }

  /// 对应 tmp::all_of_v<List, P> 的值级等价: 谓词按 TypeId 判定
  #[inline]
  pub fn all_of_ids(&self, pred: impl Fn(TypeId) -> bool) -> bool {
    self.ids.iter().all(|id| pred(*id))
  }

  /// 对应 tmp::fold_right 的值级等价: 从右折叠 (原版结果为类型,
  /// 此处结果类型由调用方决定; 空表返回 init 副本不可 Clone 泛型 →
  /// 以 &dyn 闭包+初值给出, 登记映射差异)
  #[inline]
  pub fn fold_right_ids<A, F>(&self, init: A, mut f: F) -> A
  where
    F: FnMut(TypeId, A) -> A,
  {
    let mut acc = init;
    for id in self.ids.iter().rev() {
      acc = f(*id, acc);
    }
    acc
  }
}

/// 对应 list<Args...> 构造: `tmp_list![A, B, C]`
#[macro_export]
macro_rules! tmp_list {
  ($($t:ty),* $(,)?) => {
    $crate::template_meta_programming::TypeList::from_ids(
      &[ $( ::core::any::TypeId::of::<$t>() ),* ])
  };
}

/// 对应 tmp::equals 的类型对判定宏形态 (签名对照): `tmp_equals!(A, B)`
#[macro_export]
macro_rules! tmp_equals {
  ($a:ty, $b:ty) => {
    $crate::template_meta_programming::type_equals::<$a, $b>()
  };
}

/// 对应 tmp::StringLiteral<N>: 编译期字符串。原版以 class-type NTTP 传递,
/// Rust stable 无同机制 → 以 &'static str 映射 (无 '\0' 结尾存储,
/// size()/as_bytes() 语义对应原版不含终止符的长度)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLiteral(pub &'static str);

impl StringLiteral {
  #[inline]
  pub const fn new(s: &'static str) -> Self {
    StringLiteral(s)
  }

  /// 原版 size(): 不含终止 '\0' 的长度 — str 本无终止符, 同值
  #[inline]
  pub fn size(&self) -> usize {
    self.0.len()
  }

  /// 原版 c_str(): 返回底层字节 (Rust 不保证 '\0' 结尾, 上层需要 C 字符串
  /// 时应使用 std::ffi::CString 另行物化, 登记)
  #[inline]
  pub const fn c_str(&self) -> &'static str {
    self.0
  }
}

// 推迟登记 (HKT/类型级函数依赖):
// - list1 / equals1 / index_of1 / contains1 / append1 / insert_at1 / fold_right1
// - instantiate<T, U> / is_instantiation_of<T<U>, T>
// - call_parameters<fn(A)->R> 的参数展开 (Rust 参数为单一元组类型)
// - call_foreach<Fn<T, I>>(需类型+下标双参数模板模板)
// - from_tuple<Tuple>(需类型级 tuple 展开)
