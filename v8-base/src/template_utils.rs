//! v8/src/base/template-utils.h 的 1:1 转写
//! 映射说明 (7.1 签名优先):
//! - base::make_array<Size>(f) → make_array::<N>(f), 内部用 std array::from_fn。
//!   原版 constexpr 为编译期展开; Rust 闭包无法在泛型 const fn 中调用, 故为
//!   运行时一次性填充 (调用点若需 const 上下文手写数组字面量, 登记)。
//! - pass_value_or_ref<T> → C++ 传参策略类型函数 (标量/数组按值, 其余 const&);
//!   Rust 在签名处直接选 T / &T (Copy 语义等价), 不提供类型函数 (文档映射)。
//! - has_output_operator → Rust 约定即 Display; 上层以 T: Display 约束表达。
//! - append/prepend_tuple_type → 需要元组类型拼接, stable Rust 无 (登记推迟)。
//! - nth_type_t<N, Ts...> → 类型选择, Rust 侧直接书写具体类型 (文档映射)。
//! - tuple_head/tuple_drop/tuple_map2 → 宏实现 (字面量 N ≤ 8, 元组长度不限);
//!   越界索引展开为 const{panic} 编译错误, 对应原版 static_assert/requires。
//! - tuple_for_each 系列: 每元素独立 `let` 块重新单态化闭包, 支持异构元组
//!   (对应原版泛型 lambda); 用法为语句式。
//! - tuple_for_each_with_index: 原版 index 为 std::integral_constant<size_t>,
//!   Rust 传 usize 字面量 (单态化即等价于常量传递)。
//! - index_of_type/has_type → TypeId 探测 (类型需 'static); 原版 SearchT 至多
//!   出现一次的编译期约束 → debug_assert 兜底。
//!   原版 index_of_type_v 的编译期使用 (discriminated-union.h) 上层转写时
//!   可改用宏单态化的 const 上下文或运行期断言。

use std::{any::TypeId, array::from_fn};

/// 对应 base::make_array<Size>(f): 以 f(0..Size-1) 构造定长数组。
/// 例: `make_array::<3>(|i| (2 * i) as i32)` → [0, 2, 4]
#[inline]
pub fn make_array<const N: usize, T, F>(f: F) -> [T; N]
where
  F: FnMut(usize) -> T,
{
  from_fn(f)
}

/// 对应 index_of_type_v<SearchT, Ts...>: 返回首个匹配下标, 未找到返回 len。
/// 原版约束 SearchT 至多出现一次 (否则触发 duplicate_type_in_index_of_type_error
/// 编译错误) → 此处 debug_assert。
#[inline]
pub fn index_of_type(search: TypeId, candidates: &[TypeId]) -> usize {
  let mut found = candidates.len();
  let mut match_count = 0usize;
  for (idx, c) in candidates.iter().enumerate() {
    if *c == search {
      if match_count == 0 {
        found = idx;
      }
      match_count += 1;
    }
  }
  debug_assert!(match_count <= 1, "duplicate_type_in_index_of_type_error");
  found
}

/// 对应 has_type_v<SearchT, Ts...>
#[inline]
pub fn has_type(search: TypeId, candidates: &[TypeId]) -> bool {
  candidates.contains(&search)
}

/// 类型参数版: `index_of_type!(T => [A, B, C])` (要求 T/A/B/C: 'static)
#[macro_export]
macro_rules! index_of_type {
  ($t:ty => [ $($c:ty),* $(,)? ]) => {
    $crate::template_utils::index_of_type(
      ::core::any::TypeId::of::<$t>(),
      &[ $( ::core::any::TypeId::of::<$c>() ),* ],
    )
  };
}

/// 类型参数版: `has_type!(T => [A, B, C])`
#[macro_export]
macro_rules! has_type {
  ($t:ty => [ $($c:ty),* $(,)? ]) => {
    $crate::template_utils::has_type(
      ::core::any::TypeId::of::<$t>(),
      &[ $( ::core::any::TypeId::of::<$c>() ),* ],
    )
  };
}

/// 对应 tuple_for_each(tpl, f): 对每个元素调用 f(v)。语句式宏。
/// 每个元素在独立 `let` 块中重新推导闭包类型 → 支持异构元组
/// (对应原版泛型 lambda; Rust 闭包本身不可泛型)。
#[macro_export]
macro_rules! tuple_for_each {
  ($f:expr; $($e:expr),* $(,)?) => {
    $({ let mut __v8_f = $f; __v8_f($e); })*
  };
}

/// 对应 tuple_for_each_with_index(tpl, f): 调用 f(v, index), index: usize。
/// 展开上限 8 元素 (原版无上限, 为编译期折叠; 超限触发宏匹配错误=编译错误)。
#[macro_export]
macro_rules! tuple_for_each_with_index {
  ($f:expr;) => {};
  ($f:expr; $e:expr) => {
    { let mut __v8_f = $f; __v8_f($e, 0usize); }
  };
  ($f:expr; $e:expr, $($rest:tt)*) => {
    { let mut __v8_f = $f; __v8_f($e, 0usize); $crate::__tfwi_m!(@c1 $f, $($rest)*); }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tfwi_m {
  (@c0 $f:expr) => {};
  (@c0 $f:expr, $e:expr) => {
    { let mut __v8_f = $f; __v8_f($e, 0usize); }
  };
  (@c0 $f:expr, $e:expr, $($rest:tt)*) => {
    { let mut __v8_f = $f; __v8_f($e, 0usize); $crate::__tfwi_m!(@c1 $f, $($rest)*); }
  };
  (@c1 $f:expr) => {};
  (@c1 $f:expr, $e:expr) => {
    { let mut __v8_f = $f; __v8_f($e, 1usize); }
  };
  (@c1 $f:expr, $e:expr, $($rest:tt)*) => {
    { let mut __v8_f = $f; __v8_f($e, 1usize); $crate::__tfwi_m!(@c2 $f, $($rest)*); }
  };
  (@c2 $f:expr) => {};
  (@c2 $f:expr, $e:expr) => {
    { let mut __v8_f = $f; __v8_f($e, 2usize); }
  };
  (@c2 $f:expr, $e:expr, $($rest:tt)*) => {
    { let mut __v8_f = $f; __v8_f($e, 2usize); $crate::__tfwi_m!(@c3 $f, $($rest)*); }
  };
  (@c3 $f:expr) => {};
  (@c3 $f:expr, $e:expr) => {
    { let mut __v8_f = $f; __v8_f($e, 3usize); }
  };
  (@c3 $f:expr, $e:expr, $($rest:tt)*) => {
    { let mut __v8_f = $f; __v8_f($e, 3usize); $crate::__tfwi_m!(@c4 $f, $($rest)*); }
  };
  (@c4 $f:expr) => {};
  (@c4 $f:expr, $e:expr) => {
    { let mut __v8_f = $f; __v8_f($e, 4usize); }
  };
  (@c4 $f:expr, $e:expr, $($rest:tt)*) => {
    { let mut __v8_f = $f; __v8_f($e, 4usize); $crate::__tfwi_m!(@c5 $f, $($rest)*); }
  };
  (@c5 $f:expr) => {};
  (@c5 $f:expr, $e:expr) => {
    { let mut __v8_f = $f; __v8_f($e, 5usize); }
  };
  (@c5 $f:expr, $e:expr, $($rest:tt)*) => {
    { let mut __v8_f = $f; __v8_f($e, 5usize); $crate::__tfwi_m!(@c6 $f, $($rest)*); }
  };
  (@c6 $f:expr) => {};
  (@c6 $f:expr, $e:expr) => {
    { let mut __v8_f = $f; __v8_f($e, 6usize); }
  };
  (@c6 $f:expr, $e:expr, $($rest:tt)*) => {
    { let mut __v8_f = $f; __v8_f($e, 6usize); $crate::__tfwi_m!(@c7 $f, $($rest)*); }
  };
  (@c7 $f:expr) => {};
  (@c7 $f:expr, $e:expr) => {
    { let mut __v8_f = $f; __v8_f($e, 7usize); }
  };
  (@c7 $f:expr, $e:expr, $($rest:tt)*) => {
    { let mut __v8_f = $f; __v8_f($e, 7usize); $crate::__tfwi_m!(@c8 $f, $($rest)*); }
  };
  (@c8 $f:expr) => {};
}

/// 对应 tuple_map(tpl, f): 返回结果元组 (每元素独立单态化)。
#[macro_export]
macro_rules! tuple_map {
  ($f:expr; $($e:expr),* $(,)?) => {
    ($({ let mut __v8_f = $f; __v8_f($e) },)*)
  };
}

/// 对应 tuple_map2(tplv, tplu, f): 逐对调用 f(v, u) 生成结果元组。
/// 原版 static_assert 等长约束 → 不等长时展开为编译错误。
#[macro_export]
macro_rules! tuple_map2 {
  ($f:expr, ($($v:tt)*), ($($u:tt)*)) => {
    $crate::__tm2_m!(@acc [], $f, ($($v)*), ($($u)*))
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tm2_m {
  // 双双到尾: 输出
  (@acc [$($acc:expr,)*], $f:expr, (), ()) => { ($($acc,)*) };
  // 各自剩 1 个 (含尾逗号形态)
  (@acc [$($acc:expr,)*], $f:expr, ($ve:expr), ($ue:expr)) => {
    ($($acc,)* { let mut __v8_f = $f; __v8_f($ve, $ue) },)
  };
  (@acc [$($acc:expr,)*], $f:expr, ($ve:expr,), ($ue:expr)) => {
    ($($acc,)* { let mut __v8_f = $f; __v8_f($ve, $ue) },)
  };
  (@acc [$($acc:expr,)*], $f:expr, ($ve:expr), ($ue:expr,)) => {
    ($($acc,)* { let mut __v8_f = $f; __v8_f($ve, $ue) },)
  };
  (@acc [$($acc:expr,)*], $f:expr, ($ve:expr,), ($ue:expr,)) => {
    ($($acc,)* { let mut __v8_f = $f; __v8_f($ve, $ue) },)
  };
  // 双剩 ≥2: 取头配对
  (@acc [$($acc:expr,)*], $f:expr, ($ve:expr, $($vrest:tt)*), ($ue:expr, $($urest:tt)*)) => {
    $crate::__tm2_m!(
      @acc [$($acc,)* { let mut __v8_f = $f; __v8_f($ve, $ue) },],
      $f, ($($vrest)*), ($($urest)*))
  };
  // 长度不等 → 编译错误 (对应原版 static_assert)
  (@acc $acc:tt, $f:expr, (), ($($u:tt)+)) => {
    const { panic!("tuple_map2: tuple sizes differ") }
  };
  (@acc $acc:tt, $f:expr, ($($v:tt)+), ()) => {
    const { panic!("tuple_map2: tuple sizes differ") }
  };
}

/// 对应 tuple_fold(initial, tpl, f): 左折叠 f(...f(f(initial, t0), t1)..., tn)。
#[macro_export]
macro_rules! tuple_fold {
  ($init:expr, $f:expr; $($e:expr),* $(,)?) => {{
    let mut __v8_acc = $init;
    $({ let mut __v8_f = $f; __v8_acc = __v8_f(__v8_acc, $e); })*
    __v8_acc
  }};
}

/// 对应 tuple_head<N>(tpl): 取前 N 个元素 (N 为字面量 ≤ 8)。
/// 例: `tuple_head!(2, (a, b, c))` → `(a, b,)`
/// 注意: 元组参数必须以原始括号组传入 (不可是不可拆分的 expr 片段)。
#[macro_export]
macro_rules! tuple_head {
  ($n:tt, ( $($t:tt)* )) => {
    $crate::__tuple_counted!(head, $n, ($($t)*))
  };
}

/// 对应 tuple_drop<N>(tpl): 去掉前 N 个元素 (N 为字面量 ≤ 8)。
/// 原版 requires N<=size 的编译期防护 → 越界展开为 const{panic} 编译错误。
#[macro_export]
macro_rules! tuple_drop {
  ($n:tt, ( $($t:tt)* )) => {
    $crate::__tuple_counted!(drop, $n, ($($t)*))
  };
}

/// N→预算标记分派 (预算为空格分隔的 # 序列)
#[doc(hidden)]
#[macro_export]
macro_rules! __tuple_counted {
  (head, 0, ($($t:tt)*)) => { $crate::__tuple_head_m!(@t [], [], ($($t)*)) };
  (head, 1, ($($t:tt)*)) => { $crate::__tuple_head_m!(@t [#], [], ($($t)*)) };
  (head, 2, ($($t:tt)*)) => { $crate::__tuple_head_m!(@t [# #], [], ($($t)*)) };
  (head, 3, ($($t:tt)*)) => { $crate::__tuple_head_m!(@t [# # #], [], ($($t)*)) };
  (head, 4, ($($t:tt)*)) => { $crate::__tuple_head_m!(@t [# # # #], [], ($($t)*)) };
  (head, 5, ($($t:tt)*)) => { $crate::__tuple_head_m!(@t [# # # # #], [], ($($t)*)) };
  (head, 6, ($($t:tt)*)) => { $crate::__tuple_head_m!(@t [# # # # # #], [], ($($t)*)) };
  (head, 7, ($($t:tt)*)) => { $crate::__tuple_head_m!(@t [# # # # # # #], [], ($($t)*)) };
  (head, 8, ($($t:tt)*)) => { $crate::__tuple_head_m!(@t [# # # # # # # #], [], ($($t)*)) };
  (drop, 0, ($($t:tt)*)) => { $crate::__tuple_drop_m!(@s [], ($($t)*)) };
  (drop, 1, ($($t:tt)*)) => { $crate::__tuple_drop_m!(@s [#], ($($t)*)) };
  (drop, 2, ($($t:tt)*)) => { $crate::__tuple_drop_m!(@s [# #], ($($t)*)) };
  (drop, 3, ($($t:tt)*)) => { $crate::__tuple_drop_m!(@s [# # #], ($($t)*)) };
  (drop, 4, ($($t:tt)*)) => { $crate::__tuple_drop_m!(@s [# # # #], ($($t)*)) };
  (drop, 5, ($($t:tt)*)) => { $crate::__tuple_drop_m!(@s [# # # # #], ($($t)*)) };
  (drop, 6, ($($t:tt)*)) => { $crate::__tuple_drop_m!(@s [# # # # # #], ($($t)*)) };
  (drop, 7, ($($t:tt)*)) => { $crate::__tuple_drop_m!(@s [# # # # # # #], ($($t)*)) };
  (drop, 8, ($($t:tt)*)) => { $crate::__tuple_drop_m!(@s [# # # # # # # #], ($($t)*)) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tuple_head_m {
  // 预算用尽: 输出已取前缀 (原版 N<=total 由调用方保证, 剩余元素忽略)
  (@t [], [$($acc:expr,)*], $rest:tt) => { ($($acc,)*) };
  // 剩 1 预算 + 尾元素 → 恰取满
  (@t [#], [$($acc:expr,)*], ($e:expr)) => { ($($acc,)* $e,) };
  (@t [#], [$($acc:expr,)*], ($e:expr,)) => { ($($acc,)* $e,) };
  // 取一个元素继续
  (@t [# $($b:tt)*], [$($acc:expr,)*], ($e:expr, $($rest:tt)*)) => {
    $crate::__tuple_head_m!(@t [$($b)*], [$($acc,)* $e,], ($($rest)*))
  };
  // 预算未用尽但元素耗尽 → N>size, 编译错误 (对应原版 static_assert)
  (@t [$head:tt $($b:tt)*], [$($acc:expr,)*], ()) => {
    const { panic!("tuple_head: N greater than tuple size") }
  };
  (@t [$head:tt $($b:tt)*], [$($acc:expr,)*], ($e:expr)) => {
    const { panic!("tuple_head: N greater than tuple size") }
  };
  (@t [$head:tt $($b:tt)*], [$($acc:expr,)*], ($e:expr,)) => {
    const { panic!("tuple_head: N greater than tuple size") }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tuple_drop_m {
  // 恰剩 1 预算 + 尾元素 → 丢弃完毕
  (@s [#], ($e:expr)) => { () };
  (@s [#], ($e:expr,)) => { () };
  // 恰剩 1 预算但无元素 → N>size (原版 requires 防护)
  (@s [#], ()) => { const { panic!("tuple_drop: N greater than tuple size") } };
  // 跳过一个元素
  (@s [# $($b:tt)*], ($e:expr, $($rest:tt)*)) => {
    $crate::__tuple_drop_m!(@s [$($b)*], ($($rest)*))
  };
  // 预算更长但元素耗尽 → N>size
  (@s [$head:tt $($b:tt)+], ()) => { const { panic!("tuple_drop: N greater than tuple size") } };
  // 预算用尽 → 收集剩余
  (@s [], $cur:tt) => { $crate::__tuple_drop_m!(@g [], $cur) };
  (@g [$($acc:expr,)*], ($e:expr, $($rest:tt)*)) => {
    $crate::__tuple_drop_m!(@g [$($acc,)* $e,], ($($rest)*))
  };
  (@g [$($acc:expr,)*], ($e:expr)) => { ($($acc,)* $e,) };
  (@g [$($acc:expr,)*], ($e:expr,)) => { ($($acc,)* $e,) };
  (@g [$($acc:expr,)*], ()) => { ($($acc,)*) };
}
