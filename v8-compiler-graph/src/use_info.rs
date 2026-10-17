// 由 tool/src/compiler/use-info.js 从 v8/src/compiler/use-info.h + representation-change.cc (Truncation 格) + compiler/globals.h (CheckForMinusZeroMode) + feedback-source.h (占位) 生成; 修订就地改, 复跑覆盖

//! UseInfo / Truncation — 节点输入使用的表示需求 + 截断格
//!
//! Truncation 格上函数 (Generalize/LessGeneral 等) 原文在
//! representation-change.cc:24-145, 该文件其余依赖 simplified/machine operator
//! 层推迟; 格实现随本文件迁移。
//! FeedbackSource 为占位最小形 (原版依赖 feedback-vector 对象层):
//! vector 裸地址 + slot 下标, hash/相等语义保留。

use core::ptr::null;
use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

use crate::{
  Error,
  machine_type::{MachineRepresentation, MachineType},
};

/// 原版 IdentifyZeros — ±0 是否视同
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentifyZeros {
  /// kIdentifyZeros — 视同
  Identify,
  /// kDistinguishZeros — 区分
  Distinguish,
}

/// 原版 TruncationKind (私有枚举, 格点)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TruncationKind {
  // None, Bool, Word32, Word64, OddballAndBigIntToNumber, BooleanAndNullAndBigIntToNumber, Any
  None,
  Bool,
  Word32,
  Word64,
  OddballAndBigIntToNumber,
  BooleanAndNullAndBigIntToNumber,
  Any,
}

/// 原版 Truncation — 截断描述 (kind + ±0 识别 + 安全整数检查)
///
/// 格 (偏序示意, representation-change.cc:59-75):
/// Any ⊇ {OddballAndBigIntToNumber, BooleanAndNullAndBigIntToNumber} ⊇ Word64 ⊇ Word32, Bool ⊇ None
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Truncation {
  kind: TruncationKind,
  identify_zeros: IdentifyZeros,
  check_safe_integer: bool,
}

impl Truncation {
  /// 原 Truncation::None()
  pub const fn none() -> Self {
    Self::new(TruncationKind::None, IdentifyZeros::Identify)
  }
  /// 原 Truncation::Bool()
  pub const fn boolean() -> Self {
    Self::new(TruncationKind::Bool, IdentifyZeros::Identify)
  }
  /// 原 Truncation::Word32()
  pub const fn word32() -> Self {
    Self::new(TruncationKind::Word32, IdentifyZeros::Identify)
  }
  /// 原 Truncation::Word32WithSafeIntCheck()
  pub const fn word32_with_safe_int_check() -> Self {
    Self {
      kind: TruncationKind::Word32,
      identify_zeros: IdentifyZeros::Identify,
      check_safe_integer: true,
    }
  }
  /// 原 Truncation::Word64()
  pub const fn word64() -> Self {
    Self::new(TruncationKind::Word64, IdentifyZeros::Identify)
  }
  /// 原 Truncation::BooleanAndNullAndBigIntToNumber(identify_zeros 默认 Distinguish)
  pub const fn boolean_and_null_and_bigint_to_number(identify_zeros: IdentifyZeros) -> Self {
    Self::new(
      TruncationKind::BooleanAndNullAndBigIntToNumber,
      identify_zeros,
    )
  }
  /// 原 Truncation::OddballAndBigIntToNumber(identify_zeros 默认 Distinguish)
  pub const fn oddball_and_bigint_to_number(identify_zeros: IdentifyZeros) -> Self {
    Self::new(TruncationKind::OddballAndBigIntToNumber, identify_zeros)
  }
  /// 原 Truncation::Any(identify_zeros, check_safe_integer)
  pub const fn any(identify_zeros: IdentifyZeros, check_safe_integer: bool) -> Self {
    Self {
      kind: TruncationKind::Any,
      identify_zeros,
      check_safe_integer,
    }
  }

  /// 原 Generalize — 上确界; 不相容组合原版 FATAL → Err
  pub fn generalize(t1: Self, t2: Self) -> Result<Self, crate::Error> {
    Ok(Self {
      kind: generalize_kind(t1.kind, t2.kind)?,
      identify_zeros: generalize_identify_zeros(t1.identify_zeros, t2.identify_zeros),
      check_safe_integer: t1.check_safe_integer || t2.check_safe_integer,
    })
  }

  /// 原 IsUnused
  pub const fn is_unused(&self) -> bool {
    matches!(self.kind, TruncationKind::None)
  }
  /// 原 IsUsedAsBool
  pub const fn is_used_as_bool(&self) -> bool {
    less_general(self.kind, TruncationKind::Bool)
  }
  /// 原 IsUsedAsWord32
  pub const fn is_used_as_word32(&self) -> bool {
    less_general(self.kind, TruncationKind::Word32)
  }
  /// 原 IsUsedAsWord64 (64 位构建)
  pub const fn is_used_as_word64(&self) -> bool {
    less_general(self.kind, TruncationKind::Word64)
  }
  /// 原 TruncatesBooleanAndNullAndBigIntToNumber
  pub const fn truncates_boolean_and_null_and_bigint_to_number(&self) -> bool {
    less_general(self.kind, TruncationKind::BooleanAndNullAndBigIntToNumber)
  }
  /// 原 TruncatesOddballAndBigIntToNumber
  pub const fn truncates_oddball_and_bigint_to_number(&self) -> bool {
    less_general(self.kind, TruncationKind::OddballAndBigIntToNumber)
  }
  /// 原 IdentifiesUndefinedAndZero
  pub const fn identifies_undefined_and_zero(&self) -> bool {
    less_general(self.kind, TruncationKind::Word32) || less_general(self.kind, TruncationKind::Bool)
  }
  /// 原 IdentifiesZeroAndMinusZero
  pub const fn identifies_zero_and_minus_zero(&self) -> bool {
    matches!(self.identify_zeros, IdentifyZeros::Identify)
  }
  /// 原 IsLessGeneralThan
  pub const fn is_less_general_than(&self, other: Self) -> bool {
    less_general(self.kind, other.kind)
      && less_general_identify_zeros(self.identify_zeros, other.identify_zeros)
      && (self.check_safe_integer as u8) <= (other.check_safe_integer as u8)
  }
  /// 原 identify_zeros()
  pub const fn identify_zeros(&self) -> IdentifyZeros {
    self.identify_zeros
  }
  /// 原 check_safe_integer()
  pub const fn check_safe_integer(&self) -> bool {
    self.check_safe_integer
  }
  /// 原 description() (representation-change.cc:24)
  pub fn description(&self) -> &'static str {
    match self.kind {
      TruncationKind::None => "no-value-use",
      TruncationKind::Bool => "truncate-to-bool",
      TruncationKind::Word32 => "truncate-to-word32",
      TruncationKind::Word64 => "truncate-to-word64",
      TruncationKind::OddballAndBigIntToNumber => match self.identify_zeros {
        IdentifyZeros::Identify => "truncate-oddball&bigint-to-number (identify zeros)",
        IdentifyZeros::Distinguish => "truncate-oddball&bigint-to-number (distinguish zeros)",
      },
      TruncationKind::BooleanAndNullAndBigIntToNumber => match self.identify_zeros {
        IdentifyZeros::Identify => "truncate-boolean&null&bigint-to-number (identify zeros)",
        IdentifyZeros::Distinguish => "truncate-boolean&null&bigint-to-number (distinguish zeros)",
      },
      TruncationKind::Any => match self.identify_zeros {
        IdentifyZeros::Identify => "no-truncation (but identify zeros)",
        IdentifyZeros::Distinguish => "no-truncation (but distinguish zeros)",
      },
    }
  }

  const fn new(kind: TruncationKind, identify_zeros: IdentifyZeros) -> Self {
    Self {
      kind,
      identify_zeros,
      check_safe_integer: false,
    }
  }
}

/// 原 hash_value(Truncation) — hash_combine(kind, identify_zeros)
impl Truncation {
  pub fn hash_value(&self) -> usize {
    let mut h = DefaultHasher::new();
    self.kind.hash(&mut h);
    self.identify_zeros.hash(&mut h);
    Hasher::finish(&h) as usize
  }
}

/// 原 LessGeneral — kind 偏序 (representation-change.cc:111-140)
const fn less_general(rep1: TruncationKind, rep2: TruncationKind) -> bool {
  use TruncationKind as K;
  matches!(
    (rep1, rep2),
    (K::None, _)
      | (K::Bool, K::Bool | K::Any)
      | (
        K::Word32,
        K::Word32
          | K::Word64
          | K::OddballAndBigIntToNumber
          | K::BooleanAndNullAndBigIntToNumber
          | K::Any
      )
      | (
        K::Word64,
        K::Word64 | K::OddballAndBigIntToNumber | K::BooleanAndNullAndBigIntToNumber | K::Any
      )
      | (
        K::OddballAndBigIntToNumber,
        K::OddballAndBigIntToNumber | K::BooleanAndNullAndBigIntToNumber | K::Any
      )
      | (
        K::BooleanAndNullAndBigIntToNumber,
        K::BooleanAndNullAndBigIntToNumber | K::Any
      )
      | (K::Any, K::Any)
  )
}

/// 原 Generalize(TruncationKind) — 上确界 (representation-change.cc:82-99)
fn generalize_kind(
  rep1: TruncationKind,
  rep2: TruncationKind,
) -> Result<TruncationKind, crate::Error> {
  use TruncationKind as K;
  if less_general(rep1, rep2) {
    return Ok(rep2);
  }
  if less_general(rep2, rep1) {
    return Ok(rep1);
  }
  // 可 float64 表示值的泛化
  if less_general(rep1, K::OddballAndBigIntToNumber)
    && less_general(rep2, K::OddballAndBigIntToNumber)
  {
    return Ok(K::OddballAndBigIntToNumber);
  }
  // 任意可表示值的泛化
  if less_general(rep1, K::Any) && less_general(rep2, K::Any) {
    return Ok(K::Any);
  }
  // 其余组合非法 (原版 FATAL)
  Err(Error::IncompatibleTruncation)
}

/// 原 GeneralizeIdentifyZeros (representation-change.cc:101-108)
fn generalize_identify_zeros(i1: IdentifyZeros, i2: IdentifyZeros) -> IdentifyZeros {
  if i1 == i2 {
    i1
  } else {
    IdentifyZeros::Distinguish
  }
}

/// 原 LessGeneralIdentifyZeros (representation-change.cc:142-144)
const fn less_general_identify_zeros(u1: IdentifyZeros, u2: IdentifyZeros) -> bool {
  matches!(u1, IdentifyZeros::Identify)
    || matches!(
      (u1, u2),
      (IdentifyZeros::Distinguish, IdentifyZeros::Distinguish)
    )
}

/// 原版 TypeCheckKind — 去检查类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeCheckKind {
  // None, SignedSmall, Signed32, Signed64, Unsigned64, AdditiveSafeInteger, Number, NumberOrBoolean, NumberOrOddball, HeapObject, BigInt, BigInt64, ArrayIndex
  None,
  SignedSmall,
  Signed32,
  Signed64,
  Unsigned64,
  AdditiveSafeInteger,
  Number,
  NumberOrBoolean,
  NumberOrOddball,
  HeapObject,
  BigInt,
  BigInt64,
  ArrayIndex,
}

impl TypeCheckKind {
  /// 原版 operator<< 文本
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::None => "None",
      Self::SignedSmall => "SignedSmall",
      Self::Signed32 => "Signed32",
      Self::Signed64 => "Signed64",
      Self::Unsigned64 => "Unsigned64",
      Self::AdditiveSafeInteger => "AdditiveSafeInteger",
      Self::Number => "Number",
      Self::NumberOrBoolean => "NumberOrBoolean",
      Self::NumberOrOddball => "NumberOrOddball",
      Self::HeapObject => "HeapObject",
      Self::BigInt => "BigInt",
      Self::BigInt64 => "BigInt64",
      Self::ArrayIndex => "ArrayIndex",
    }
  }
}

/// 原版 CheckForMinusZeroMode (compiler/globals.h:54)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckForMinusZeroMode {
  /// kCheckForMinusZero
  Check,
  /// kDontCheckForMinusZero
  DontCheck,
}

/// FeedbackSource 占位最小形 — 反馈向量地址 + 槽位
///
/// 原版 = { IndirectHandle<FeedbackVector> vector; FeedbackSlot slot; },
/// 对象层 (feedback-vector) 落地后升级; hash/相等按 (地址, 槽位) 保留原语义。
#[derive(Debug, Clone, Copy)]
pub struct FeedbackSource {
  /// 反馈向量裸地址 (原版 handle.address())
  pub vector: *const (),
  /// 槽位下标 (原版 FeedbackSlot int; kInvalidSlot = -1)
  pub slot: i32,
}

impl FeedbackSource {
  /// 无效槽位哨兵 (feedback-vector.h kInvalidSlot)
  pub const INVALID_SLOT: i32 = -1;

  /// 原 Invalid() 形态
  pub const fn invalid() -> Self {
    Self {
      vector: null(),
      slot: Self::INVALID_SLOT,
    }
  }

  /// 原 IsValid() — 向量与槽位均有效
  pub const fn is_valid(&self) -> bool {
    !self.vector.is_null() && self.slot >= 0
  }
}

impl PartialEq for FeedbackSource {
  /// 原 operator== — (地址, 槽位) 相等
  fn eq(&self, other: &Self) -> bool {
    self.vector == other.vector && self.slot == other.slot
  }
}
impl Eq for FeedbackSource {}

impl Hash for FeedbackSource {
  /// 原 FeedbackSource::Hash — hash_combine(address, slot)
  fn hash<H: Hasher>(&self, state: &mut H) {
    (self.vector as usize).hash(state);
    self.slot.hash(state);
  }
}

/// UseInfo — 节点输入使用描述 (表示偏好 + 截断 + 检查类别 + 反馈源)
#[derive(Debug, Clone, Copy)]
pub struct UseInfo {
  representation: MachineRepresentation,
  truncation: Truncation,
  type_check: TypeCheckKind,
  feedback: FeedbackSource,
}

macro_rules! use_info_ctor {
  ($(#[$doc:meta])* $name:ident ($repr:expr, $trunc:expr)) => {
    $(#[$doc])*
    pub const fn $name() -> Self {
      Self::with_check($repr, $trunc, TypeCheckKind::None)
    }
  };
  ($(#[$doc:meta])* $name:ident($repr:expr, $trunc:expr, $check:expr)) => {
    $(#[$doc])*
    pub const fn $name() -> Self {
      Self::with_check($repr, $trunc, $check)
    }
  };
}

impl UseInfo {
  /// 原通用构造 (representation, truncation, type_check, feedback)
  pub const fn new(
    representation: MachineRepresentation,
    truncation: Truncation,
    type_check: TypeCheckKind,
    feedback: FeedbackSource,
  ) -> Self {
    Self {
      representation,
      truncation,
      type_check,
      feedback,
    }
  }

  const fn with_check(
    representation: MachineRepresentation,
    truncation: Truncation,
    type_check: TypeCheckKind,
  ) -> Self {
    Self::new(
      representation,
      truncation,
      type_check,
      FeedbackSource::invalid(),
    )
  }

  use_info_ctor!(
    /// 原 TruncatingWord32()
    truncating_word32(MachineRepresentation::Word32, Truncation::word32())
  );
  use_info_ctor!(
    /// 原 TruncatingWord64()
    truncating_word64(MachineRepresentation::Word64, Truncation::word64())
  );

  /// 原 CheckedBigIntTruncatingWord64(feedback)
  pub const fn checked_bigint_truncating_word64(feedback: FeedbackSource) -> Self {
    Self::with_check(
      MachineRepresentation::Word64,
      Truncation::word64(),
      TypeCheckKind::BigInt,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedBigInt64AsWord64(feedback)
  pub const fn checked_bigint64_as_word64(feedback: FeedbackSource) -> Self {
    Self::with_check(
      MachineRepresentation::Word64,
      Truncation::any(IdentifyZeros::Distinguish, false),
      TypeCheckKind::BigInt64,
    )
    .with_feedback(feedback)
  }
  /// 原 Word64(identify_zeros 默认 Distinguish)
  pub const fn word64(identify_zeros: IdentifyZeros) -> Self {
    Self::with_check(
      MachineRepresentation::Word64,
      Truncation::any(identify_zeros, false),
      TypeCheckKind::None,
    )
  }
  /// 原 Word() — 指针表示
  pub const fn word() -> Self {
    Self::with_check(
      MachineType::pointer_representation(),
      Truncation::any(IdentifyZeros::Distinguish, false),
      TypeCheckKind::None,
    )
  }
  use_info_ctor!(
    /// 原 Bool()
    boolean(MachineRepresentation::Bit, Truncation::boolean())
  );
  use_info_ctor!(
    /// 原 Float32()
    float32(
      MachineRepresentation::Float32,
      Truncation::any(IdentifyZeros::Distinguish, false)
    )
  );
  use_info_ctor!(
    /// 原 Float16RawBits()
    float16_raw_bits(
      MachineRepresentation::Float16RawBits,
      Truncation::any(IdentifyZeros::Distinguish, false)
    )
  );
  use_info_ctor!(
    /// 原 Float64()
    float64(
      MachineRepresentation::Float64,
      Truncation::any(IdentifyZeros::Distinguish, false)
    )
  );
  /// 原 TruncatingFloat64(identify_zeros 默认 Distinguish)
  pub const fn truncating_float64(identify_zeros: IdentifyZeros) -> Self {
    Self::with_check(
      MachineRepresentation::Float64,
      Truncation::oddball_and_bigint_to_number(identify_zeros),
      TypeCheckKind::None,
    )
  }
  /// 原 TruncatingFloat16RawBits(identify_zeros 默认 Distinguish)
  pub const fn truncating_float16_raw_bits(identify_zeros: IdentifyZeros) -> Self {
    Self::with_check(
      MachineRepresentation::Float16,
      Truncation::oddball_and_bigint_to_number(identify_zeros),
      TypeCheckKind::None,
    )
  }
  /// 原 CheckedSafeIntTruncatingWord32(feedback)
  pub const fn checked_safe_int_truncating_word32(feedback: FeedbackSource) -> Self {
    Self::with_check(
      MachineRepresentation::Word32,
      Truncation::word32_with_safe_int_check(),
      TypeCheckKind::AdditiveSafeInteger,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedSafeIntAsWord64(feedback)
  pub const fn checked_safe_int_as_word64(feedback: FeedbackSource) -> Self {
    Self::with_check(
      MachineRepresentation::Word64,
      Truncation::any(IdentifyZeros::Distinguish, true),
      TypeCheckKind::AdditiveSafeInteger,
    )
    .with_feedback(feedback)
  }
  /// 原 TruncatingFloat64OrUndefined(identify_zeros 默认 Distinguish)
  pub const fn truncating_float64_or_undefined(identify_zeros: IdentifyZeros) -> Self {
    Self::with_check(
      MachineRepresentation::Float64,
      Truncation::boolean_and_null_and_bigint_to_number(identify_zeros),
      TypeCheckKind::None,
    )
  }
  use_info_ctor!(
    /// 原 AnyTagged()
    any_tagged(
      MachineRepresentation::Tagged,
      Truncation::any(IdentifyZeros::Distinguish, false)
    )
  );
  use_info_ctor!(
    /// 原 TaggedSigned()
    tagged_signed(
      MachineRepresentation::TaggedSigned,
      Truncation::any(IdentifyZeros::Distinguish, false)
    )
  );
  use_info_ctor!(
    /// 原 TaggedPointer()
    tagged_pointer(
      MachineRepresentation::TaggedPointer,
      Truncation::any(IdentifyZeros::Distinguish, false)
    )
  );

  /// 原 CheckedTaggedAsArrayIndex(feedback)
  pub const fn checked_tagged_as_array_index(feedback: FeedbackSource) -> Self {
    Self::with_check(
      MachineType::pointer_representation(),
      Truncation::any(IdentifyZeros::Identify, false),
      TypeCheckKind::ArrayIndex,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedHeapObjectAsTaggedPointer(feedback)
  pub const fn checked_heap_object_as_tagged_pointer(feedback: FeedbackSource) -> Self {
    Self::with_check(
      MachineRepresentation::TaggedPointer,
      Truncation::any(IdentifyZeros::Distinguish, false),
      TypeCheckKind::HeapObject,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedBigIntAsTaggedPointer(feedback)
  pub const fn checked_bigint_as_tagged_pointer(feedback: FeedbackSource) -> Self {
    Self::with_check(
      MachineRepresentation::TaggedPointer,
      Truncation::any(IdentifyZeros::Distinguish, false),
      TypeCheckKind::BigInt,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedSignedSmallAsTaggedSigned(feedback, identify_zeros 默认 Distinguish)
  pub const fn checked_signed_small_as_tagged_signed(
    feedback: FeedbackSource,
    identify_zeros: IdentifyZeros,
  ) -> Self {
    Self::with_check(
      MachineRepresentation::TaggedSigned,
      Truncation::any(identify_zeros, false),
      TypeCheckKind::SignedSmall,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedSignedSmallAsWord32(identify_zeros, feedback)
  pub const fn checked_signed_small_as_word32(
    identify_zeros: IdentifyZeros,
    feedback: FeedbackSource,
  ) -> Self {
    Self::with_check(
      MachineRepresentation::Word32,
      Truncation::any(identify_zeros, false),
      TypeCheckKind::SignedSmall,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedSigned32AsWord32(identify_zeros, feedback)
  pub const fn checked_signed32_as_word32(
    identify_zeros: IdentifyZeros,
    feedback: FeedbackSource,
  ) -> Self {
    Self::with_check(
      MachineRepresentation::Word32,
      Truncation::any(identify_zeros, false),
      TypeCheckKind::Signed32,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedSigned64AsWord64(identify_zeros, feedback)
  pub const fn checked_signed64_as_word64(
    identify_zeros: IdentifyZeros,
    feedback: FeedbackSource,
  ) -> Self {
    Self::with_check(
      MachineRepresentation::Word64,
      Truncation::any(identify_zeros, false),
      TypeCheckKind::Signed64,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedUnsigned64AsWord64(identify_zeros, feedback)
  pub const fn checked_unsigned64_as_word64(
    identify_zeros: IdentifyZeros,
    feedback: FeedbackSource,
  ) -> Self {
    Self::with_check(
      MachineRepresentation::Word64,
      Truncation::any(identify_zeros, false),
      TypeCheckKind::Unsigned64,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedNumberAsFloat64(identify_zeros, feedback)
  pub const fn checked_number_as_float64(
    identify_zeros: IdentifyZeros,
    feedback: FeedbackSource,
  ) -> Self {
    Self::with_check(
      MachineRepresentation::Float64,
      Truncation::any(identify_zeros, false),
      TypeCheckKind::Number,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedNumberAsWord32(feedback)
  pub const fn checked_number_as_word32(feedback: FeedbackSource) -> Self {
    Self::with_check(
      MachineRepresentation::Word32,
      Truncation::word32(),
      TypeCheckKind::Number,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedNumberOrBooleanAsFloat64(identify_zeros, feedback)
  pub const fn checked_number_or_boolean_as_float64(
    identify_zeros: IdentifyZeros,
    feedback: FeedbackSource,
  ) -> Self {
    Self::with_check(
      MachineRepresentation::Float64,
      Truncation::any(identify_zeros, false),
      TypeCheckKind::NumberOrBoolean,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedNumberOrOddballAsFloat64(identify_zeros, feedback)
  pub const fn checked_number_or_oddball_as_float64(
    identify_zeros: IdentifyZeros,
    feedback: FeedbackSource,
  ) -> Self {
    Self::with_check(
      MachineRepresentation::Float64,
      Truncation::any(identify_zeros, false),
      TypeCheckKind::NumberOrOddball,
    )
    .with_feedback(feedback)
  }
  /// 原 CheckedNumberOrOddballAsWord32(feedback)
  pub const fn checked_number_or_oddball_as_word32(feedback: FeedbackSource) -> Self {
    Self::with_check(
      MachineRepresentation::Word32,
      Truncation::word32(),
      TypeCheckKind::NumberOrOddball,
    )
    .with_feedback(feedback)
  }
  use_info_ctor!(
    /// 原 Any() — 表示未定
    any(
      MachineRepresentation::None,
      Truncation::any(IdentifyZeros::Distinguish, false)
    )
  );
  use_info_ctor!(
    /// 原 AnyTruncatingToBool()
    any_truncating_to_bool(MachineRepresentation::None, Truncation::boolean())
  );
  use_info_ctor!(
    /// 原 None() — 值未用
    none(MachineRepresentation::None, Truncation::none())
  );

  const fn with_feedback(self, feedback: FeedbackSource) -> Self {
    Self { feedback, ..self }
  }

  /// 原 representation()
  pub const fn representation(&self) -> MachineRepresentation {
    self.representation
  }
  /// 原 truncation()
  pub const fn truncation(&self) -> Truncation {
    self.truncation
  }
  /// 原 type_check()
  pub const fn type_check(&self) -> TypeCheckKind {
    self.type_check
  }
  /// 原 minus_zero_check()
  pub const fn minus_zero_check(&self) -> CheckForMinusZeroMode {
    if self.truncation.identifies_zero_and_minus_zero() {
      CheckForMinusZeroMode::DontCheck
    } else {
      CheckForMinusZeroMode::Check
    }
  }
  /// 原 feedback()
  pub const fn feedback(&self) -> FeedbackSource {
    self.feedback
  }
}

impl PartialEq for UseInfo {
  /// 原 operator== — 四元组相等
  fn eq(&self, other: &Self) -> bool {
    self.representation == other.representation
      && self.truncation == other.truncation
      && self.type_check == other.type_check
      && self.feedback == other.feedback
  }
}
impl Eq for UseInfo {}

impl Hash for UseInfo {
  /// 原 hash_value — 四元组组合
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.representation.hash(state);
    self.truncation.hash(state);
    self.type_check.hash(state);
    self.feedback.hash(state);
  }
}
