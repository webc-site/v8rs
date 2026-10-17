// 由 tool/src/interpreter/bytecode-flags-and-tokens.h.js 从 v8/src/interpreter/bytecode-flags-and-tokens.{h,cc} 生成; 修订就地改, 复跑覆盖

use v8_common::{LanguageMode, LookupHoistingMode};

use crate::Error;

/// typeof 字面量旗标 (原版 TestTypeOfFlags::LiteralFlag; TYPEOF_LITERAL_LIST 展开, 9 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LiteralFlag {
  /// 原版 kNumber ("number")
  Number = 0,
  /// 原版 kString ("string")
  String = 1,
  /// 原版 kSymbol ("symbol")
  Symbol = 2,
  /// 原版 kBoolean ("boolean")
  Boolean = 3,
  /// 原版 kBigInt ("bigint")
  BigInt = 4,
  /// 原版 kUndefined ("undefined")
  Undefined = 5,
  /// 原版 kFunction ("function")
  Function = 6,
  /// 原版 kObject ("object")
  Object = 7,
  /// 原版 kOther ("other")
  Other = 8,
}

pub type TestTypeOfLiteralFlag = LiteralFlag;

impl LiteralFlag {
  /// 原版名 (TestTypeOfFlags::ToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Number => "number",
      Self::String => "string",
      Self::Symbol => "symbol",
      Self::Boolean => "boolean",
      Self::BigInt => "bigint",
      Self::Undefined => "undefined",
      Self::Function => "function",
      Self::Object => "object",
      Self::Other => "other",
    }
  }

  #[inline]
  #[must_use]
  pub const fn encode(self) -> u8 {
    self as u8
  }

  #[inline]
  pub fn decode(raw: u8) -> crate::Result<Self> {
    TestTypeOfFlags::decode(raw)
  }
}

/// CreateArrayLiteralFlags (原版 class CreateArrayLiteralFlags)
pub struct CreateArrayLiteralFlags;

impl CreateArrayLiteralFlags {
  pub const FLAGS_MASK: u8 = 0b0001_1111;
  pub const FLAGS_SHIFT: u32 = 0;
  pub const FAST_CLONE_SUPPORTED_SHIFT: u32 = 5;
  pub const FAST_CLONE_SUPPORTED_BIT: u8 = 1 << 5;

  #[inline]
  #[must_use]
  pub const fn encode(use_fast_shallow_clone: bool, runtime_flags: u8) -> u8 {
    (runtime_flags & Self::FLAGS_MASK)
      | ((use_fast_shallow_clone as u8) << Self::FAST_CLONE_SUPPORTED_SHIFT)
  }
}

/// CreateObjectLiteralFlags (原版 class CreateObjectLiteralFlags)
pub struct CreateObjectLiteralFlags;

impl CreateObjectLiteralFlags {
  pub const FLAGS_MASK: u8 = 0b0001_1111;
  pub const FLAGS_SHIFT: u32 = 0;
  pub const FAST_CLONE_SUPPORTED_SHIFT: u32 = 5;
  pub const FAST_CLONE_SUPPORTED_BIT: u8 = 1 << 5;

  #[inline]
  #[must_use]
  pub const fn encode(runtime_flags: u8, fast_clone_supported: bool) -> u8 {
    (runtime_flags & Self::FLAGS_MASK)
      | ((fast_clone_supported as u8) << Self::FAST_CLONE_SUPPORTED_SHIFT)
  }
}

/// CreateClosureFlags (原版 class CreateClosureFlags)
pub struct CreateClosureFlags;

impl CreateClosureFlags {
  pub const PRETENURED_SHIFT: u32 = 0;
  pub const FAST_NEW_CLOSURE_SHIFT: u32 = 1;
  pub const PRETENURED_BIT: u8 = 1 << 0;
  pub const FAST_NEW_CLOSURE_BIT: u8 = 1 << 1;

  #[inline]
  #[must_use]
  pub const fn encode(pretenure: bool, is_function_scope: bool) -> u8 {
    (pretenure as u8) | (((!pretenure && is_function_scope) as u8) << 1)
  }
}

/// TestTypeOfFlags (原版 class TestTypeOfFlags)
pub struct TestTypeOfFlags;

impl TestTypeOfFlags {
  #[inline]
  #[must_use]
  pub const fn encode(literal_flag: LiteralFlag) -> u8 {
    literal_flag as u8
  }

  #[inline]
  pub fn decode(raw_flag: u8) -> crate::Result<LiteralFlag> {
    match raw_flag {
      0 => Ok(LiteralFlag::Number),
      1 => Ok(LiteralFlag::String),
      2 => Ok(LiteralFlag::Symbol),
      3 => Ok(LiteralFlag::Boolean),
      4 => Ok(LiteralFlag::BigInt),
      5 => Ok(LiteralFlag::Undefined),
      6 => Ok(LiteralFlag::Function),
      7 => Ok(LiteralFlag::Object),
      8 => Ok(LiteralFlag::Other),
      _ => Err(Error::InvalidFlag(raw_flag)),
    }
  }

  #[inline]
  #[must_use]
  pub const fn to_string(literal_flag: LiteralFlag) -> &'static str {
    literal_flag.as_str()
  }
}

/// StoreLookupSlotFlags (原版 class StoreLookupSlotFlags)
pub struct StoreLookupSlotFlags;

impl StoreLookupSlotFlags {
  pub const LANGUAGE_MODE_SHIFT: u32 = 0;
  pub const LOOKUP_HOISTING_SHIFT: u32 = 1;

  #[inline]
  #[must_use]
  pub fn encode(language_mode: LanguageMode, lookup_hoisting_mode: LookupHoistingMode) -> u8 {
    let lang_val = (language_mode == LanguageMode::Strict) as u8;
    let hoist_val = (lookup_hoisting_mode == LookupHoistingMode::LegacySloppy) as u8;
    (lang_val & 1) | (hoist_val << 1)
  }

  #[inline]
  #[must_use]
  pub fn get_language_mode(flags: u8) -> LanguageMode {
    if (flags & 1) != 0 {
      LanguageMode::Strict
    } else {
      LanguageMode::Sloppy
    }
  }

  #[inline]
  #[must_use]
  pub fn is_lookup_hoisting_mode(flags: u8) -> bool {
    (flags & 2) != 0
  }
}

/// CreateArrayLiteral / CreateObjectLiteral 共用位域模块 (兼容模块别名)
pub mod literal_flags {
  pub const FLAGS_MASK: u8 = super::CreateArrayLiteralFlags::FLAGS_MASK;
  pub const FLAGS_SHIFT: u32 = super::CreateArrayLiteralFlags::FLAGS_SHIFT;
  pub const FAST_CLONE_SUPPORTED_SHIFT: u32 =
    super::CreateArrayLiteralFlags::FAST_CLONE_SUPPORTED_SHIFT;
  pub const FAST_CLONE_SUPPORTED_MASK: u8 =
    super::CreateArrayLiteralFlags::FAST_CLONE_SUPPORTED_BIT;

  #[inline]
  #[must_use]
  pub const fn encode(runtime_flags: u8, fast_clone_supported: bool) -> u8 {
    super::CreateArrayLiteralFlags::encode(fast_clone_supported, runtime_flags)
  }
}

/// CreateClosure 位域模块 (兼容模块别名)
pub mod closure_flags {
  pub const PRETENURED_SHIFT: u32 = super::CreateClosureFlags::PRETENURED_SHIFT;
  pub const FAST_NEW_CLOSURE_SHIFT: u32 = super::CreateClosureFlags::FAST_NEW_CLOSURE_SHIFT;
  pub const PRETENURED_BIT: u8 = super::CreateClosureFlags::PRETENURED_BIT;
  pub const FAST_NEW_CLOSURE_BIT: u8 = super::CreateClosureFlags::FAST_NEW_CLOSURE_BIT;

  #[inline]
  #[must_use]
  pub const fn encode(pretenure: bool, is_function_scope: bool) -> u8 {
    super::CreateClosureFlags::encode(pretenure, is_function_scope)
  }
}

/// StoreLookupSlot 位域模块 (兼容模块别名)
pub mod store_lookup_slot_flags {
  pub const LANGUAGE_MODE_SHIFT: u32 = super::StoreLookupSlotFlags::LANGUAGE_MODE_SHIFT;
  pub const LOOKUP_HOISTING_SHIFT: u32 = super::StoreLookupSlotFlags::LOOKUP_HOISTING_SHIFT;

  #[inline]
  #[must_use]
  pub fn encode(
    language_mode: super::LanguageMode,
    lookup_hoisting_mode: super::LookupHoistingMode,
  ) -> u8 {
    super::StoreLookupSlotFlags::encode(language_mode, lookup_hoisting_mode)
  }

  #[inline]
  #[must_use]
  pub fn get_language_mode(flags: u8) -> super::LanguageMode {
    super::StoreLookupSlotFlags::get_language_mode(flags)
  }

  #[inline]
  #[must_use]
  pub fn is_lookup_hoisting_mode(flags: u8) -> bool {
    super::StoreLookupSlotFlags::is_lookup_hoisting_mode(flags)
  }
}

/// try-finally 续延 token (原版 enum class TryFinallyContinuationToken : int)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TryFinallyContinuationToken {
  /// 落穿 (跳转表 fallthrough 槽位)
  FallthroughToken = -1,
  /// 重抛
  RethrowToken = 0,
}
/// 原版 class/struct CreateArrayLiteralFlags 成员声明 (机械 1:1 提取, 1 条)
pub const CREATE_ARRAY_LITERAL_FLAGS_DECLARATIONS: &[&str] =
  &["[public/method] static uint8_t Encode(bool use_fast_shallow_clone, int runtime_flags)"];

/// 原版 class/struct CreateObjectLiteralFlags 成员声明 (机械 1:1 提取, 1 条)
pub const CREATE_OBJECT_LITERAL_FLAGS_DECLARATIONS: &[&str] =
  &["[public/method] static uint8_t Encode(int runtime_flags, bool fast_clone_supported)"];

/// 原版 class/struct CreateClosureFlags 成员声明 (机械 1:1 提取, 1 条)
pub const CREATE_CLOSURE_FLAGS_DECLARATIONS: &[&str] =
  &["[public/method] static uint8_t Encode(bool pretenure, bool is_function_scope)"];

/// 原版 class/struct TestTypeOfFlags 成员声明 (机械 1:1 提取, 4 条)
pub const TEST_TYPEOF_FLAGS_DECLARATIONS: &[&str] = &[
  "[public/method] static LiteralFlag GetFlagForLiteral(const AstStringConstants* ast_constants, Literal* literal)",
  "[public/method] static uint8_t Encode(LiteralFlag literal_flag)",
  "[public/method] static LiteralFlag Decode(uint8_t raw_flag)",
  "[public/const] static const char* ToString(LiteralFlag literal_flag)",
];

/// 原版 class/struct StoreLookupSlotFlags 成员声明 (机械 1:1 提取, 3 条)
pub const STORE_LOOKUP_SLOT_FLAGS_DECLARATIONS: &[&str] = &[
  "[public/method] static uint8_t Encode(LanguageMode language_mode, LookupHoistingMode lookup_hoisting_mode)",
  "[public/method] static LanguageMode GetLanguageMode(uint8_t flags)",
  "[public/method] static bool IsLookupHoistingMode(uint8_t flags)",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(CREATE_ARRAY_LITERAL_FLAGS_DECLARATIONS.len() == 1);
  assert!(TEST_TYPEOF_FLAGS_DECLARATIONS.len() == 4);
  assert!(STORE_LOOKUP_SLOT_FLAGS_DECLARATIONS.len() == 3);
  assert!(literal_flags::FAST_CLONE_SUPPORTED_SHIFT == 5);
  assert!(LiteralFlag::BigInt as u8 == 4);
  assert!(LiteralFlag::Other.as_str().len() == 5);
};
