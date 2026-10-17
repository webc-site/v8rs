//! use-info.h + representation-change.cc (Truncation 格) 单测:
//! 格偏序 / Generalize 规则 / ±0 识别 / UseInfo 构造面 / minus_zero_check /
//! FeedbackSource 占位语义

use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

use aok::{OK, Void};
use log::info;
use v8_compiler_graph::{
  CheckForMinusZeroMode, FeedbackSource, IdentifyZeros, MachineRepresentation, MachineSemantic,
  MachineType, Truncation, TypeCheckKind, UseInfo,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// Truncation 格偏序: LessGeneral 抽样 (representation-change.cc:111-140)
#[test]
fn truncation_test_less_general() -> Void {
  use Truncation as T;
  // None ≤ 一切
  assert!(T::none().is_less_general_than(T::word32()));
  assert!(T::none().is_less_general_than(T::any(IdentifyZeros::Distinguish, false)));
  // Bool: 仅 Bool/Any 之上
  assert!(T::boolean().is_less_general_than(T::any(IdentifyZeros::Distinguish, false)));
  assert!(!T::boolean().is_less_general_than(T::word32()));
  // Word32 ≤ Word64 / Oddball / BooleanNull / Any
  assert!(T::word32().is_less_general_than(T::word64()));
  assert!(
    T::word32().is_less_general_than(T::oddball_and_bigint_to_number(IdentifyZeros::Distinguish))
  );
  assert!(
    T::word32().is_less_general_than(T::boolean_and_null_and_bigint_to_number(
      IdentifyZeros::Distinguish
    ))
  );
  assert!(!T::word32().is_less_general_than(T::boolean()));
  // Word64 ≤ Oddball / BooleanNull / Any
  assert!(T::word64().is_less_general_than(T::any(IdentifyZeros::Distinguish, false)));
  assert!(!T::word64().is_less_general_than(T::word32()));
  // Oddball ≤ BooleanNull / Any
  assert!(
    T::oddball_and_bigint_to_number(IdentifyZeros::Distinguish).is_less_general_than(
      T::boolean_and_null_and_bigint_to_number(IdentifyZeros::Distinguish)
    )
  );
  assert!(
    !T::boolean_and_null_and_bigint_to_number(IdentifyZeros::Distinguish)
      .is_less_general_than(T::oddball_and_bigint_to_number(IdentifyZeros::Distinguish))
  );
  // Any 仅 ≤ Any
  assert!(
    T::any(IdentifyZeros::Distinguish, false)
      .is_less_general_than(T::any(IdentifyZeros::Distinguish, false))
  );
  assert!(!T::any(IdentifyZeros::Distinguish, false).is_less_general_than(T::word64()));
  info!("lattice ok");
  OK
}

/// Generalize 规则 + 不相容组合 → Err (representation-change.cc:82-99)
#[test]
fn truncation_test_generalize() -> Void {
  use Truncation as T;
  let iz = IdentifyZeros::Distinguish;
  // 可比 → 上者
  assert_eq!(T::word32(), T::generalize(T::none(), T::word32())?);
  assert_eq!(T::word64(), T::generalize(T::word32(), T::word64())?);
  // Bool × Word32: 不可比且 Bool ∉ Oddball 下链 → Any (Any 合并规则; Oddball 分支为原版保留路径)
  assert_eq!(
    T::any(IdentifyZeros::Identify, false),
    T::generalize(T::boolean(), T::word32())?
  );
  // 可比对: Word32 × Oddball → Oddball
  assert_eq!(
    T::oddball_and_bigint_to_number(iz),
    T::generalize(T::word32(), T::oddball_and_bigint_to_number(iz))?
  );
  // check_safe_integer 合并取或
  let a = T::any(iz, true);
  let b = T::any(iz, false);
  assert!(T::generalize(a, b)?.check_safe_integer());
  // 不相容: BooleanNull × Oddball? 两者可比 (Oddball ≤ BooleanNull) → OK
  assert!(
    T::generalize(
      T::oddball_and_bigint_to_number(iz),
      T::boolean_and_null_and_bigint_to_number(iz)
    )
    .is_ok()
  );
  OK
}

/// ±0 识别 (IdentifyZeros 族)
#[test]
fn truncation_test_identify_zeros() -> Void {
  use Truncation as T;
  assert!(T::none().identifies_zero_and_minus_zero());
  assert!(T::word32().identifies_zero_and_minus_zero());
  assert!(!T::any(IdentifyZeros::Distinguish, false).identifies_zero_and_minus_zero());
  assert!(T::any(IdentifyZeros::Identify, false).identifies_zero_and_minus_zero());
  // GeneralizeIdentifyZeros: 不同 → Distinguish
  let mixed = T::generalize(
    T::any(IdentifyZeros::Identify, false),
    T::any(IdentifyZeros::Distinguish, false),
  )?;
  assert!(!mixed.identifies_zero_and_minus_zero());
  // LessGeneralIdentifyZeros: Identify ≤ Distinguish
  assert!(
    T::any(IdentifyZeros::Identify, false)
      .is_less_general_than(T::any(IdentifyZeros::Distinguish, false))
  );
  OK
}

/// 查询面 + description (use-info.h:67-110)
#[test]
fn truncation_test_queries() -> Void {
  use Truncation as T;
  assert!(T::none().is_unused());
  assert!(!T::word32().is_unused());
  assert!(T::boolean().is_used_as_bool());
  assert!(!T::word64().is_used_as_bool());
  assert!(T::word32().is_used_as_word32());
  assert!(T::word64().is_used_as_word64());
  assert!(T::word32().identifies_undefined_and_zero());
  assert!(T::boolean().identifies_undefined_and_zero());
  assert_eq!("no-value-use", T::none().description());
  assert_eq!("truncate-to-word32", T::word32().description());
  assert_eq!(
    "truncate-oddball&bigint-to-number (distinguish zeros)",
    T::oddball_and_bigint_to_number(IdentifyZeros::Distinguish).description()
  );
  assert_eq!(
    "no-truncation (but identify zeros)",
    T::any(IdentifyZeros::Identify, false).description()
  );
  // word32_with_safe_int_check
  let w = T::word32_with_safe_int_check();
  assert!(w.check_safe_integer());
  assert!(w.is_used_as_word32());
  OK
}

/// UseInfo 构造面 (use-info.h:226-390 抽样)
#[test]
fn use_info_test_constructors() -> Void {
  let u = UseInfo::truncating_word32();
  assert_eq!(MachineRepresentation::Word32, u.representation());
  assert!(u.truncation().is_used_as_word32());
  assert_eq!(TypeCheckKind::None, u.type_check());
  assert!(!u.feedback().is_valid());
  let u64 = UseInfo::truncating_word64();
  assert_eq!(MachineRepresentation::Word64, u64.representation());
  let tagged = UseInfo::any_tagged();
  assert_eq!(MachineRepresentation::Tagged, tagged.representation());
  let none = UseInfo::none();
  assert_eq!(MachineRepresentation::None, none.representation());
  assert!(none.truncation().is_unused());
  let any = UseInfo::any();
  assert_eq!(MachineRepresentation::None, any.representation());
  // 带反馈源构造
  let fb = FeedbackSource {
    vector: 0x1000 as *const (),
    slot: 3,
  };
  let checked = UseInfo::checked_signed_small_as_tagged_signed(fb, IdentifyZeros::Distinguish);
  assert!(checked.feedback().is_valid());
  assert_eq!(TypeCheckKind::SignedSmall, checked.type_check());
  assert_eq!(
    MachineRepresentation::TaggedSigned,
    checked.representation()
  );
  // 指针表示 (64 位 Word64)
  assert_eq!(
    MachineRepresentation::Word64,
    MachineType::pointer_representation()
  );
  assert_eq!(
    MachineRepresentation::Word64,
    UseInfo::word().representation()
  );
  info!("ctors ok");
  OK
}

/// minus_zero_check (use-info.h:395-399)
#[test]
fn use_info_test_minus_zero_check() -> Void {
  let identify = UseInfo::checked_tagged_as_array_index(FeedbackSource::invalid());
  assert_eq!(
    CheckForMinusZeroMode::DontCheck,
    identify.minus_zero_check()
  ); // Truncation::Any(kIdentifyZeros)
  let distinguish = UseInfo::any_tagged();
  assert_eq!(CheckForMinusZeroMode::Check, distinguish.minus_zero_check());
  assert_eq!(
    CheckForMinusZeroMode::DontCheck,
    UseInfo::truncating_word32().minus_zero_check()
  );
  OK
}

/// 相等 + hash 语义 (use-info.h:409-419)
#[test]
fn use_info_test_eq_hash() -> Void {
  let a = UseInfo::truncating_word64();
  let b = UseInfo::truncating_word64();
  assert_eq!(a, b);
  let c = UseInfo::word64(IdentifyZeros::Identify);
  assert_ne!(a, c); // truncation 不同 (Word64() 用 Any)
  let mut ha = DefaultHasher::new();
  a.hash(&mut ha);
  let mut hb = DefaultHasher::new();
  b.hash(&mut hb);
  assert_eq!(ha.finish(), hb.finish());
  // TypeCheckKind 文本 (原版 operator<<)
  assert_eq!("SignedSmall", TypeCheckKind::SignedSmall.as_str());
  assert_eq!("NumberOrOddball", TypeCheckKind::NumberOrOddball.as_str());
  OK
}

/// FeedbackSource 占位语义 (原版 feedback-source.h 的 hash/eq/valid 面)
#[test]
fn feedback_source_test_placeholder() -> Void {
  let inv = FeedbackSource::invalid();
  assert!(!inv.is_valid());
  assert_eq!(FeedbackSource::INVALID_SLOT, -1);
  let a = FeedbackSource {
    vector: 0x2000 as *const (),
    slot: 7,
  };
  let b = FeedbackSource {
    vector: 0x2000 as *const (),
    slot: 7,
  };
  let c = FeedbackSource {
    vector: 0x2000 as *const (),
    slot: 8,
  };
  assert_eq!(a, b);
  assert_ne!(a, c);
  assert!(a.is_valid());
  let mut ha = DefaultHasher::new();
  a.hash(&mut ha);
  let mut hb = DefaultHasher::new();
  b.hash(&mut hb);
  assert_eq!(ha.finish(), hb.finish());
  OK
}

/// MachineType 最小子集 (machine-type.h 谓词面)
#[test]
fn machine_type_test_subset() -> Void {
  let f64t = MachineType::float64();
  assert_eq!(MachineRepresentation::Float64, f64t.representation());
  assert!(!f64t.is_tagged());
  assert!(MachineType::any_tagged().is_tagged());
  assert!(MachineType::tagged_pointer().is_tagged_pointer());
  assert!(MachineType::tagged_signed().is_tagged_signed());
  assert!(MachineType::int8().is_signed());
  assert!(MachineType::uint8().is_unsigned());
  assert!(MachineType::uint_ptr().is_unsigned());
  assert_eq!(MachineType::uint_ptr(), MachineType::uint64());
  assert_eq!(MachineType::int_ptr(), MachineType::int64());
  assert!(MachineType::new(MachineRepresentation::None, MachineSemantic::None).is_none());
  OK
}
