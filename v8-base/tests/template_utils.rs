//! v8/test/unittests/base/template-utils-unittest.cc 的规约迁移
//!
//! 规约清单:
//! - MakeArraySimple: make_array<3>(i => 1 + i*3) == [1, 4, 7]
//! - MakeArrayConstexpr: make_array<3>(i => i*2) == [0, 2, 4]
//!   (Rust 侧 make_array 为运行时填充, 值规约等价; 常量上下文由数组字面量承载)
//! - pass_value_or_ref / has_output_operator: 纯类型函数, Rust 以签名处
//!   T/&T 与 Display 约束直接表达 (登记跳过理由), 以下改为对 Rust 侧承接
//!   API (index_of_type / has_type / tuple 宏族) 的行为断言 (加强)

use std::any::TypeId;

use aok::{OK, Void};
use v8_base::template_utils::{has_type, index_of_type, make_array};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// TEST(TemplateUtilsTest, MakeArraySimple)
#[test]
fn template_utils_make_array_simple() -> Void {
  let computed = make_array::<3, i32, _>(|i| 1 + (i as i32 * 3));
  let expected = [1, 4, 7];
  assert_eq!(computed, expected);
  OK
}

/// TEST(TemplateUtilsTest, MakeArrayConstexpr)
/// 原版 constexpr doubleIntValue → Rust 闭包, 值规约一致
#[test]
fn template_utils_make_array_constexpr() -> Void {
  let computed = make_array::<3, i32, _>(|i| i as i32 * 2);
  let expected = [0, 2, 4];
  assert_eq!(computed, expected);
  OK
}

/// index_of_type_v: 找到 → 首匹配下标; 未找到 → len;
/// 重复出现 → debug 断言 duplicate (原版编译错误)
#[test]
fn template_utils_index_of_type() -> Void {
  #[derive(Clone, Copy)]
  struct A;
  #[derive(Clone, Copy)]
  struct B;
  let candidates = [TypeId::of::<A>(), TypeId::of::<B>()];
  assert_eq!(1, index_of_type(TypeId::of::<B>(), &candidates));
  assert_eq!(
    candidates.len(),
    index_of_type(TypeId::of::<u8>(), &candidates)
  );
  assert!(has_type(TypeId::of::<A>(), &candidates));
  assert!(!has_type(TypeId::of::<i8>(), &candidates));
  OK
}

/// tuple_for_each / tuple_for_each_with_index: 逐元素访问与下标
#[test]
fn template_utils_tuple_for_each() -> Void {
  let mut sum = 0;
  v8_base::tuple_for_each!(|v: i32| { sum += v; }; 1, 2, 3);
  assert_eq!(6, sum);

  let mut indexed = String::new();
  v8_base::tuple_for_each_with_index!(|v: &str, i: usize| { indexed.push_str(&format!("{v}{i}")); }; "a", "b");
  assert_eq!("a0b1", indexed);
  OK
}

/// tuple_map / tuple_map2: 结果元组 (map2 等长约束)
#[test]
fn template_utils_tuple_map() -> Void {
  let mapped = v8_base::tuple_map!(|v: i32| v * 2; 1, 2, 3);
  assert_eq!((2, 4, 6), mapped);

  let zipped = v8_base::tuple_map2!(|a: i32, b: i32| a + b, (1, 2), (10, 20));
  assert_eq!((11, 22), zipped);
  OK
}

/// tuple_fold: 左折叠
#[test]
fn template_utils_tuple_fold() -> Void {
  // 先绑定再折叠: 避免 clippy::redundant_closure_call (宏展开为立即调用)
  let add = |acc: i32, v: i32| acc + v;
  let folded = v8_base::tuple_fold!(0, add; 1, 2, 3, 4);
  assert_eq!(10, folded);
  OK
}

/// tuple_head / tuple_drop: 取前缀 / 去前缀 (原版 static_assert 越界防护)
#[test]
fn template_utils_tuple_head_drop() -> Void {
  let head = v8_base::tuple_head!(2, (1, "a", 2.0));
  assert_eq!((1, "a"), head);

  let dropped = v8_base::tuple_drop!(1, (1, "a", 2.0));
  assert_eq!(("a", 2.0), dropped);
  OK
}
