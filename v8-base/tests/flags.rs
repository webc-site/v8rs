//! v8-base/src/flags.rs 集成测试 — 1:1 对应 flags-unittest.cc
//!
//! 原版用例: FlagsTest.BasicOperations / FlagsTest.NamespaceScope /
//! FlagsTest.ClassScope。
//! 原版 DEFINE_OPERATORS_FOR_FLAGS 的 enum|enum 语法 → Flags::flag(..)|..;
//! static_cast<int>(a) → a.mask()。死亡/退化路径无 (全值语义)。

use aok::{OK, Void};
use v8_base::flags::{FlagEnum, Flags, hash_value};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// enum Flag1 (原版匿名命名空间)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum Flag1 {
  None = 0,
  First = 1 << 1,
  Second = 1 << 2,
}
const FLAG1_ALL: i32 = Flag1::None as i32 | Flag1::First as i32 | Flag1::Second as i32;

impl FlagEnum<i32> for Flag1 {
  fn to_mask(self) -> i32 {
    self as i32
  }
}

type Flags1 = Flags<Flag1>;

// 原版 bar(Flags1) 直通函数: 验证按值传递语义
fn bar(flags1: Flags1) -> Flags1 {
  flags1
}

/// FlagsTest.BasicOperations
#[test]
fn flags_test_basic_operations() -> Void {
  let mut a = Flags1::new();
  assert_eq!(Flag1::None as i32, a.mask());
  a |= Flags1::flag(Flag1::First);
  assert_eq!(Flag1::First as i32, a.mask());
  a |= Flags1::flag(Flag1::Second);
  assert_eq!(FLAG1_ALL, a.mask());
  // 原版 a = a | kFlag1Second 的非赋值 | 路径单独锁定
  assert_eq!(FLAG1_ALL, (a | Flags1::flag(Flag1::Second)).mask());
  a &= Flags1::flag(Flag1::Second);
  assert_eq!(Flag1::Second as i32, a.mask());
  a = Flags1::flag(Flag1::None) & a;
  assert_eq!(Flag1::None as i32, a.mask());
  a ^= Flags1::from_mask(FLAG1_ALL);
  assert_eq!(FLAG1_ALL, a.mask());
  let b = !a;
  assert_eq!(FLAG1_ALL, a.mask());
  assert_eq!(!a.mask(), b.mask());
  let c = a;
  assert_eq!(a, c);
  assert_ne!(a, b);
  assert_eq!(a, bar(a));
  assert_eq!(a, bar(Flags1::from_mask(FLAG1_ALL)));
  OK
}

// enum Option (原版 foo 命名空间)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum Option {
  NoOptions = 0,
  Option1 = 1,
  Option2 = 2,
}
const ALL_OPTIONS: i32 = Option::NoOptions as i32 | Option::Option1 as i32 | Option::Option2 as i32;

impl FlagEnum<i32> for Option {
  fn to_mask(self) -> i32 {
    self as i32
  }
}

type Options = Flags<Option>;

/// FlagsTest.NamespaceScope
#[test]
fn flags_test_namespace_scope() -> Void {
  let mut options = Options::new();
  options ^= Options::flag(Option::NoOptions);
  options |= Options::flag(Option::Option1) | Options::flag(Option::Option2);
  assert_eq!(ALL_OPTIONS, options.mask());
  OK
}

// struct Foo::Enum (原版 class scope, BitfieldT = uint32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum Enum {
  Enum1 = 1,
  Enum2 = 2,
}

impl FlagEnum<u32> for Enum {
  fn to_mask(self) -> u32 {
    self as u32
  }
}

type Enums = Flags<Enum, u32>;

/// FlagsTest.ClassScope
#[test]
fn flags_test_class_scope() -> Void {
  let mut enums = Enums::new();
  enums.set_flag(Enum::Enum1, true);
  enums.set_flag(Enum::Enum2, true);
  assert!(enums.contains_flag(Enum::Enum1));
  assert!(enums.contains_flag(Enum::Enum2));
  // 补充: set(false) 清除 + with/without/hash_value (原版同级 API)
  enums.set_flag(Enum::Enum2, false);
  assert!(!enums.contains_flag(Enum::Enum2));
  assert!(
    enums
      .with(Enums::flag(Enum::Enum2))
      .contains_flag(Enum::Enum2)
  );
  assert!(
    !enums
      .without(Enums::flag(Enum::Enum1))
      .contains_flag(Enum::Enum1)
  );
  assert_eq!(hash_value(enums), enums.mask() as usize);
  OK
}
