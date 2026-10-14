//! integer-literal 与 hash-seed 模块级测试 (原版无独立 unittest, 按语义规约构造)

use std::mem::size_of;

use aok::{OK, Void};
use v8_numbers::{
  hash_seed::{HashSeed, HashSeedData, RAPIDHASH_DEFAULT_SECRET, SECRETS_COUNT},
  integer_literal::{IntegerLiteral, Ordering},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// IntegerLiteral 构造与比较 (integer-literal.h 语义)
#[test]
fn integer_literal_basics() -> Void {
  // 零归一: (true, 0) → 非负
  let zero = IntegerLiteral::from_sign_abs(true, 0);
  assert!(!zero.is_negative());
  assert_eq!(0, zero.absolute_value());

  // compare 三态
  let one = IntegerLiteral::from_sign_abs(false, 1);
  let neg_one = IntegerLiteral::from_sign_abs(true, 1);
  assert_eq!(Ordering::Greater, one.compare(&neg_one));
  assert_eq!(Ordering::Less, neg_one.compare(&one));
  assert_eq!(
    Ordering::Equal,
    one.compare(&IntegerLiteral::from_sign_abs(false, 1))
  );
  // 等绝对值不同号
  assert_eq!(Ordering::Less, neg_one.compare(&one));
  // 大负数 < 小正数; 大绝对值负数更小
  let big_neg = IntegerLiteral::from_sign_abs(true, 100);
  assert_eq!(Ordering::Less, big_neg.compare(&one));
  assert_eq!(
    Ordering::Greater,
    big_neg.compare(&IntegerLiteral::from_sign_abs(true, 200))
  );

  // PartialEq 与 Ord (走 compare)
  assert!(one == IntegerLiteral::from(1i32));
  assert!(one != neg_one);
  assert!(neg_one < one);
  assert!(one > neg_one);
  assert!(one >= one);
  assert!(neg_one <= one);
  OK
}

// IsRepresentableAs / To / TryTo 值域检查
#[test]
fn integer_literal_representable() -> Void {
  let v = IntegerLiteral::from(i32::MAX);
  assert!(v.is_representable_as::<i32>());
  assert_eq!(i32::MAX, v.to::<i32>());
  let over = IntegerLiteral::from(2_147_483_648i64);
  assert!(!over.is_representable_as::<i32>());
  assert!(over.try_to::<i32>().is_none());
  assert!(over.try_to::<i64>().is_some());

  // i64::MIN 可表示 (位型直转)
  let min64 = IntegerLiteral::from(i64::MIN);
  assert!(min64.is_representable_as::<i64>());
  assert_eq!(i64::MIN, min64.to::<i64>());
  assert!(!min64.is_representable_as::<u64>());

  // 负 i32 直转
  let neg = IntegerLiteral::from(-5i32);
  assert_eq!(-5i64, neg.to::<i64>());
  OK
}

// 位运算与加法 (integer-literal-inl.h)
#[test]
fn integer_literal_ops() -> Void {
  // 按位或 (非负域)
  let a = IntegerLiteral::from(0b1100u64);
  let b = IntegerLiteral::from(0b0011u64);
  assert_eq!(IntegerLiteral::from(0b1111u64), a.bit_or(&b));

  // 左移 (绝对值移位, 符号保留)
  let n = IntegerLiteral::from_sign_abs(true, 3);
  let shifted = n.shift_left(&IntegerLiteral::from(4i32));
  assert_eq!(48, shifted.absolute_value());
  assert!(shifted.is_negative());

  // 加法: 同号 / 异号 / 零
  let x = IntegerLiteral::from(100i64);
  let y = IntegerLiteral::from(200i64);
  assert_eq!(IntegerLiteral::from(300i64), x.add(&y));
  let neg_y = IntegerLiteral::from(-200i64);
  assert_eq!(IntegerLiteral::from(-100i64), x.add(&neg_y));
  // 100 + (-100) = 0 (归一非负)
  let sum = x.add(&IntegerLiteral::from(-100i64));
  assert!(!sum.is_negative() && sum.absolute_value() == 0);
  // (-5) + 3 = -2
  assert_eq!(
    IntegerLiteral::from(-2i64),
    IntegerLiteral::from(-5i64).add(&IntegerLiteral::from(3i64))
  );

  // Display / ToString
  assert_eq!("-12345", IntegerLiteral::from(-12345i64).to_string_value());
  assert_eq!("12345", IntegerLiteral::from(12345i64).to_string_value());
  assert_eq!("-12345", format!("{}", IntegerLiteral::from(-12345i64)));
  assert_eq!("12345", format!("{}", IntegerLiteral::from(12345i64)));
  assert_eq!("0", format!("{}", IntegerLiteral::from(0i32)));
  OK
}

// HashSeed 数据面 (脚本提取常量)
#[test]
fn hash_seed_data() -> Void {
  assert_eq!(3, SECRETS_COUNT);
  assert_eq!(
    [
      0x2d35_8dcc_aa6c_78a5,
      0x8bb8_4b93_962e_acc9,
      0x4b33_a62e_d433_d4a3
    ],
    RAPIDHASH_DEFAULT_SECRET
  );

  // 默认种子视图
  let seed = HashSeed::default_seed();
  assert_eq!(0, seed.seed());
  assert_eq!(&RAPIDHASH_DEFAULT_SECRET, seed.secret());

  // Data 布局字节数: seed + 3 secrets
  assert_eq!(32, size_of::<HashSeedData>());
  OK
}
