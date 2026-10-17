//! 原版 v8/test/unittests/base/unique-array-unittest.cc 1:1 迁移
//!
//! 3 个 TEST 全落地。C++ move 后读源对象是 UB; Rust 由所有权禁止,
//! "moved-from 置空" 可观察语义用 `std::mem::take` 承接 (src/unique_array.rs
//! 头注已登记该差异)。

use std::mem::take;

use aok::{OK, Void};
use v8_base::unique_array::{UniqueArray, unique_copy_of};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// UniqueArrayTest.Equals
#[test]
fn unique_array_test_equals() -> Void {
  let mut int_arr = UniqueArray::<i32>::new_with_default(4);
  assert_eq!(4, int_arr.size());
  // find_if(begin, end, i != 0) == end: 全 0
  assert_eq!(None, int_arr.as_slice().iter().position(|&i| i != 0));

  let k_init = [4, 11, 3];
  let init_arr1 = unique_copy_of(&k_init);
  let init_arr2 = unique_copy_of(&k_init);
  assert_eq!(init_arr1.as_slice(), k_init);
  assert!(init_arr1 == init_arr2);
  let _ = &mut int_arr;
  OK
}

/// UniqueArrayTest.FactoriesAndViews
#[test]
fn unique_array_test_factories_and_views() -> Void {
  let fill_arr = UniqueArray::new_filled(3, 42);
  assert_eq!(3, fill_arr.size());
  assert_eq!(&[42, 42, 42], fill_arr.as_slice());
  assert_eq!(&[42, 42, 42], fill_arr.as_vector().as_slice());

  // UniqueArray<const int>::New(2, 7): Rust 侧同类型 (只读性由 & 借用表达)
  let const_fill_arr = UniqueArray::new_filled(2, 7);
  assert_eq!(2, const_fill_arr.size());
  assert_eq!(&[7, 7], const_fill_arr.as_slice());

  let k_init = [4, 11, 3];
  let copied_ptr = UniqueArray::copied_from(&k_init);
  assert_eq!(&k_init, copied_ptr.as_slice());
  assert_eq!(copied_ptr.as_vector().as_slice(), &k_init);

  let copied_span = UniqueArray::copied_from(k_init.as_slice());
  assert_eq!(&k_init, copied_span.as_slice());
  assert_eq!(copied_span.as_vector().as_slice(), &k_init);

  // UniqueCopyOf(kInit, 2): 取前 2 个
  let copy_of_ptr = unique_copy_of(&k_init[..2]);
  assert_eq!(&[4, 11], copy_of_ptr.as_slice());
  assert_eq!(&[4, 11], copy_of_ptr.as_vector().as_slice());
  OK
}

/// UniqueArrayTest.MoveConstructionAndAssignment
#[test]
fn unique_array_test_move_construction_and_assignment() -> Void {
  let k_values = [4, 11, 3];
  let int_arr = unique_copy_of(&k_values);
  assert_eq!(3, int_arr.size());

  // move 构造
  let mut move_constructed_arr = int_arr;
  assert_eq!(move_constructed_arr.as_vector().as_slice(), &k_values);

  // move 赋值到空对象 (move 赋值即覆盖, 原值先行析构以表达覆盖语义)
  let mut move_assigned_to_empty = UniqueArray::<i32>::default();
  drop(move_assigned_to_empty);
  move_assigned_to_empty = take(&mut move_constructed_arr);
  assert_eq!(move_assigned_to_empty.as_vector().as_slice(), &k_values);

  // move 赋值到非空对象 (原 [0,0] 被覆盖析构)
  let mut move_assigned_to_non_empty = UniqueArray::<i32>::new_with_default(2);
  drop(move_assigned_to_non_empty);
  move_assigned_to_non_empty = take(&mut move_assigned_to_empty);
  assert_eq!(move_assigned_to_non_empty.as_vector().as_slice(), &k_values);

  // 除最后一个外全部为空 (length 0, data nullptr)
  assert!(move_constructed_arr.empty());
  assert!(move_constructed_arr.is_null());
  assert!(move_assigned_to_empty.empty());
  assert!(move_assigned_to_empty.is_null());
  OK
}
