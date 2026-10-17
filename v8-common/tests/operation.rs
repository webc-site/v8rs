//! 原版 v8/src/common/operation.h 模块级校验 (上游无专属 unittest)
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - operation_variants_and_discriminants: OPERATION_LIST 展开 22 变体,
//!   判别值 0..=21, 顺序 ARITHMETIC → UNARY → COMPARISON (operation.h:12-47)
//! - operation_as_str_matches_names: as_str / Display 输出与 #name 一致
//!   (operator<<, operation.h:94-103)
//! - operation_predicates_match_lists: is_bitwise_binary 6 元集 (h:12-18),
//!   is_unary 4 元集 (h:28-32), is_comparison 6 元集 (h:34-40),
//!   is_binary = !is_unary (h:78-80)

use aok::{OK, Void};
use v8_common::Operation;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const ALL: [Operation; 22] = [
  Operation::Add,
  Operation::Subtract,
  Operation::Multiply,
  Operation::Divide,
  Operation::Modulus,
  Operation::Exponentiate,
  Operation::BitwiseAnd,
  Operation::BitwiseOr,
  Operation::BitwiseXor,
  Operation::ShiftLeft,
  Operation::ShiftRight,
  Operation::ShiftRightLogical,
  Operation::BitwiseNot,
  Operation::Negate,
  Operation::Increment,
  Operation::Decrement,
  Operation::Equal,
  Operation::StrictEqual,
  Operation::LessThan,
  Operation::LessThanOrEqual,
  Operation::GreaterThan,
  Operation::GreaterThanOrEqual,
];

/// OPERATION_LIST 22 变体判别值与顺序 (uint8_t 底层枚举)
#[test]
fn operation_variants_and_discriminants() -> Void {
  for (i, &op) in ALL.iter().enumerate() {
    assert_eq!(op as u8, i as u8, "variant order at index {i}");
  }
  // 首尾锚点 (operation.h:48-52)
  assert_eq!(Operation::Add as u8, 0);
  assert_eq!(Operation::GreaterThanOrEqual as u8, 21);
  // Ord 与判别值同序
  assert!(Operation::Add < Operation::GreaterThanOrEqual);
  assert!(ALL.windows(2).all(|w| w[0] < w[1]));
  OK
}

/// operator<< 输出名 = #name (operation.h:94-103)
#[test]
fn operation_as_str_matches_names() -> Void {
  let names = [
    "Add",
    "Subtract",
    "Multiply",
    "Divide",
    "Modulus",
    "Exponentiate",
    "BitwiseAnd",
    "BitwiseOr",
    "BitwiseXor",
    "ShiftLeft",
    "ShiftRight",
    "ShiftRightLogical",
    "BitwiseNot",
    "Negate",
    "Increment",
    "Decrement",
    "Equal",
    "StrictEqual",
    "LessThan",
    "LessThanOrEqual",
    "GreaterThan",
    "GreaterThanOrEqual",
  ];
  for (op, name) in ALL.iter().zip(names) {
    assert_eq!(op.as_str(), name);
    assert_eq!(op.to_string(), name);
  }
  OK
}

/// 谓词集与 X-macro 子列表 1:1 (operation.h:54-92)
#[test]
fn operation_predicates_match_lists() -> Void {
  let bitwise: Vec<_> = ALL
    .iter()
    .copied()
    .filter(|o| o.is_bitwise_binary())
    .collect();
  assert_eq!(
    bitwise,
    vec![
      Operation::BitwiseAnd,
      Operation::BitwiseOr,
      Operation::BitwiseXor,
      Operation::ShiftLeft,
      Operation::ShiftRight,
      Operation::ShiftRightLogical,
    ]
  );
  let unary: Vec<_> = ALL.iter().copied().filter(|o| o.is_unary()).collect();
  assert_eq!(
    unary,
    vec![
      Operation::BitwiseNot,
      Operation::Negate,
      Operation::Increment,
      Operation::Decrement,
    ]
  );
  let cmp: Vec<_> = ALL.iter().copied().filter(|o| o.is_comparison()).collect();
  assert_eq!(
    cmp,
    vec![
      Operation::Equal,
      Operation::StrictEqual,
      Operation::LessThan,
      Operation::LessThanOrEqual,
      Operation::GreaterThan,
      Operation::GreaterThanOrEqual,
    ]
  );
  // IsBinaryOperation = !IsUnaryOperation
  for &op in &ALL {
    assert_eq!(op.is_binary(), !op.is_unary());
  }
  // 集合互斥核对: 位运算均为二元、一元均非位运算
  for &op in &bitwise {
    assert!(op.is_binary());
  }
  for &op in &unary {
    assert!(!op.is_binary());
    assert!(!op.is_bitwise_binary());
  }
  // 比较运算不属于位运算/一元
  for &op in &cmp {
    assert!(!op.is_bitwise_binary());
    assert!(op.is_binary());
  }
  OK
}
