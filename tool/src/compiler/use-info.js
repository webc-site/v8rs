#!/usr/bin/env bun
// v8/src/compiler/use-info.h + representation-change.cc (Truncation 格上实现,
// 仅 24-145 行) + compiler/globals.h (CheckForMinusZeroMode) +
// codegen/machine-type.h (MachineRepresentation/MachineSemantic/MachineType 最小子集)
// → v8-compiler-graph/src/use_info.rs + src/machine_type.rs 基线
//
// machine-type.h 完整迁移属 codegen 层, 此处内联最小子集 (MachineRepresentation
// 枚举 + MachineSemantic + MachineType 值类型 + 常用构造 + PointerRepresentation),
// README 登记迁移去向。FeedbackSource 为占位最小形 (vector 地址 + slot),
// feedback-vector 对象层落地后升级。
// 对数: Truncation 格偏序逐条 (LessGeneral 7 case / Generalize 4 规则 /
// GeneralizeIdentifyZeros / LessGeneralIdentifyZeros) 与原文逐字核对。
import { readV8, writeSrc, banner } from "../../lib.js";

const uh = readV8("src/compiler/use-info.h");
const rc = readV8("src/compiler/representation-change.cc");
const mt = readV8("src/codegen/machine-type.h");
const cg = readV8("src/compiler/globals.h");

// —— 哨兵 ——
const sentinels = [
  [uh, /enum IdentifyZeros : uint8_t \{/, "IdentifyZeros 枚举"],
  [uh, /kBooleanAndNullAndBigIntToNumber,/, "TruncationKind 全序"],
  [uh, /LessGeneral\(kind_, TruncationKind::kBool\);/, "IsUsedAsBool 偏序"],
  [uh, /enum class TypeCheckKind : uint8_t \{/, "TypeCheckKind 枚举"],
  [uh, /minus_zero_check\(\) const \{/, "minus_zero_check"],
  [rc, /bool Truncation::LessGeneral\(TruncationKind rep1/, "LessGeneral 实现"],
  [rc, /Truncation::TruncationKind Truncation::Generalize\(/, "Generalize 实现"],
  [rc, /IdentifyZeros Truncation::GeneralizeIdentifyZeros\(/, "GeneralizeIdentifyZeros"],
  [rc, /bool Truncation::LessGeneralIdentifyZeros\(/, "LessGeneralIdentifyZeros"],
  [rc, /FATAL\("Tried to combine incompatible truncations"\);/, "Generalize FATAL"],
  [mt, /enum class MachineRepresentation : uint8_t \{/, "MachineRepresentation"],
  [mt, /enum class MachineSemantic : uint8_t \{/, "MachineSemantic"],
  [mt, /constexpr static MachineRepresentation PointerRepresentation\(\)/, "PointerRepresentation"],
  [cg, /enum class CheckForMinusZeroMode : uint8_t \{/, "CheckForMinusZeroMode"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// MachineRepresentation 变体序 (machine-type.h 19-51, 逐项提取)
const mrBlock = mt.match(/enum class MachineRepresentation : uint8_t \{([\s\S]*?)\n\};/)[1];
const mrVariants = [...mrBlock.matchAll(/^\s*k(\w+),/gm)].map((m) => m[1]);
if (mrVariants.length !== 21) throw new Error(`MachineRepresentation ${mrVariants.length} != 21`);
if (mrVariants[0] !== "None" || mrVariants[20] !== "Simd256") throw new Error("MachineRepresentation 首尾漂移");
// MachineSemantic 变体序
const msBlock = mt.match(/enum class MachineSemantic : uint8_t \{([\s\S]*?)\n\};/)[1];
const msVariants = [...msBlock.matchAll(/^\s*k(\w+),?\s*$/gm)].map((m) => m[1]);
if (msVariants.length !== 11) throw new Error(`MachineSemantic ${msVariants.length} != 11`);
// TruncationKind 变体序 (use-info.h 113-121)
const tkBlock = uh.match(/enum class TruncationKind : uint8_t \{([\s\S]*?)\};/)[1];
const tkVariants = [...tkBlock.matchAll(/^\s*k(\w+),?\s*$/gm)].map((m) => m[1]);
if (tkVariants.join() !== "None,Bool,Word32,Word64,OddballAndBigIntToNumber,BooleanAndNullAndBigIntToNumber,Any")
  throw new Error(`TruncationKind 序漂移: ${tkVariants}`);
// TypeCheckKind 变体序 (use-info.h 153-167)
const tcBlock = uh.match(/enum class TypeCheckKind : uint8_t \{([\s\S]*?)\};/)[1];
const tcVariants = [...tcBlock.matchAll(/^\s*k(\w+),?\s*$/gm)].map((m) => m[1]);
if (tcVariants.length !== 13) throw new Error(`TypeCheckKind ${tcVariants.length} != 13`);

const mr = mrVariants.map((v) => `${v[0].toUpperCase() + v.slice(1)},`).join(" ");
const ms = msVariants.map((v) => `${v[0].toUpperCase() + v.slice(1)},`).join(" ");
const tk = tkVariants.join(", ");
const tc = tcVariants.join(", ");

// ---------------------------------------------------------------------------
// machine_type.rs
// ---------------------------------------------------------------------------
const machineType = `${banner("src/compiler/use-info.js", "src/codegen/machine-type.h (最小子集)")}
//! MachineRepresentation / MachineSemantic / MachineType — codegen 层最小子集
//!
//! 完整 machine-type.h (ElementSize*/MachineReprToString 等) 属 codegen 迁移层;
//! 此处仅内联 use-info.h 所需面, 后续独立 crate 时整体迁出。
//! 64 位目标: PointerRepresentation = Word64 (kSystemPointerSize == 8)。

/// 原版 MachineRepresentation — 表示类别 (integral 连续升序, FP/SIMD 殿后)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MachineRepresentation {
${mrVariants.map((v, i) => `  /// k${v}\n  ${v[0].toUpperCase() + v.slice(1)} = ${i},`).join("\n")}
}

/// 原版 MachineSemantic — 语义类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MachineSemantic {
${msVariants.map((v, i) => `  /// k${v}\n  ${v[0].toUpperCase() + v.slice(1)} = ${i},`).join("\n")}
}

/// 原版 MachineType — (representation, semantic) 值对
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MachineType {
  rep: MachineRepresentation,
  sem: MachineSemantic,
}

#[allow(non_snake_case)]
impl MachineType {
  /// 原 representation()
  pub const fn representation(self) -> MachineRepresentation {
    self.rep
  }
  /// 原 semantic()
  pub const fn semantic(self) -> MachineSemantic {
    self.sem
  }
  /// 原 IsNone
  pub const fn IsNone(self) -> bool {
    matches!(self.rep, MachineRepresentation::None)
  }
  /// 原 IsMapWord
  pub const fn IsMapWord(self) -> bool {
    matches!(self.rep, MachineRepresentation::MapWord)
  }
  /// 原 IsSigned
  pub const fn IsSigned(self) -> bool {
    matches!(self.sem, MachineSemantic::Int32 | MachineSemantic::Int64)
  }
  /// 原 IsUnsigned
  pub const fn IsUnsigned(self) -> bool {
    matches!(self.sem, MachineSemantic::Uint32 | MachineSemantic::Uint64)
  }
  /// 原 IsTagged
  pub const fn IsTagged(self) -> bool {
    matches!(
      self.rep,
      MachineRepresentation::TaggedPointer
        | MachineRepresentation::TaggedSigned
        | MachineRepresentation::Tagged
    )
  }
  /// 原 IsTaggedSigned
  pub const fn IsTaggedSigned(self) -> bool {
    matches!(self.rep, MachineRepresentation::TaggedSigned)
  }
  /// 原 IsTaggedPointer
  pub const fn IsTaggedPointer(self) -> bool {
    matches!(self.rep, MachineRepresentation::TaggedPointer)
  }
  /// 原 IsCompressed
  pub const fn IsCompressed(self) -> bool {
    matches!(
      self.rep,
      MachineRepresentation::CompressedPointer | MachineRepresentation::Compressed
    )
  }
  /// 原 IsCompressedPointer
  pub const fn IsCompressedPointer(self) -> bool {
    matches!(self.rep, MachineRepresentation::CompressedPointer)
  }
  /// 原 IsIndirectPointer
  pub const fn IsIndirectPointer(self) -> bool {
    matches!(self.rep, MachineRepresentation::IndirectPointer)
  }

  /// 原 PointerRepresentation — 64 位目标恒 Word64
  pub const fn PointerRepresentation() -> MachineRepresentation {
    MachineRepresentation::Word64
  }
  /// 原 UintPtr / IntPtr (64 位)
  pub const fn UintPtr() -> Self {
    Self::Uint64()
  }
  pub const fn IntPtr() -> Self {
    Self::Int64()
  }
  pub const fn Int8() -> Self {
    Self::new(MachineRepresentation::Word8, MachineSemantic::Int32)
  }
  pub const fn Uint8() -> Self {
    Self::new(MachineRepresentation::Word8, MachineSemantic::Uint32)
  }
  pub const fn Int16() -> Self {
    Self::new(MachineRepresentation::Word16, MachineSemantic::Int32)
  }
  pub const fn Uint16() -> Self {
    Self::new(MachineRepresentation::Word16, MachineSemantic::Uint32)
  }
  pub const fn Int32() -> Self {
    Self::new(MachineRepresentation::Word32, MachineSemantic::Int32)
  }
  pub const fn Uint32() -> Self {
    Self::new(MachineRepresentation::Word32, MachineSemantic::Uint32)
  }
  pub const fn Int64() -> Self {
    Self::new(MachineRepresentation::Word64, MachineSemantic::Int64)
  }
  pub const fn Uint64() -> Self {
    Self::new(MachineRepresentation::Word64, MachineSemantic::Uint64)
  }
  pub const fn Float32() -> Self {
    Self::new(MachineRepresentation::Float32, MachineSemantic::Number)
  }
  pub const fn Float64() -> Self {
    Self::new(MachineRepresentation::Float64, MachineSemantic::Number)
  }
  pub const fn new(representation: MachineRepresentation, semantic: MachineSemantic) -> Self {
    Self { rep: representation, sem: semantic }
  }
  /// 原 TaggedSigned()
  pub const fn TaggedSigned() -> Self {
    Self::new(MachineRepresentation::TaggedSigned, MachineSemantic::Any)
  }
  /// 原 TaggedPointer()
  pub const fn TaggedPointer() -> Self {
    Self::new(MachineRepresentation::TaggedPointer, MachineSemantic::Any)
  }
  /// 原 AnyTagged()
  pub const fn AnyTagged() -> Self {
    Self::new(MachineRepresentation::Tagged, MachineSemantic::Any)
  }
  /// 原 Pointer() (64 位)
  pub const fn Pointer() -> Self {
    Self::new(Self::PointerRepresentation(), MachineSemantic::None)
  }
}
`;

// ---------------------------------------------------------------------------
// use_info.rs
// ---------------------------------------------------------------------------
const useInfo = `${banner("src/compiler/use-info.js", "src/compiler/use-info.h + representation-change.cc (Truncation 格) + compiler/globals.h (CheckForMinusZeroMode) + feedback-source.h (占位)")}
//! UseInfo / Truncation — 节点输入使用的表示需求 + 截断格
//!
//! Truncation 格上函数 (Generalize/LessGeneral 等) 原文在
//! representation-change.cc:24-145, 该文件其余依赖 simplified/machine operator
//! 层推迟; 格实现随本文件迁移。
//! FeedbackSource 为占位最小形 (原版依赖 feedback-vector 对象层):
//! vector 裸地址 + slot 下标, hash/相等语义保留。

use std::hash::{Hash, Hasher};

use crate::machine_type::{MachineRepresentation, MachineType};

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
  // ${tk}
  ${tkVariants.map((v) => v[0].toUpperCase() + v.slice(1)).join(",\n  ")},
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
    Self { kind: TruncationKind::Word32, identify_zeros: IdentifyZeros::Identify, check_safe_integer: true }
  }
  /// 原 Truncation::Word64()
  pub const fn word64() -> Self {
    Self::new(TruncationKind::Word64, IdentifyZeros::Identify)
  }
  /// 原 Truncation::BooleanAndNullAndBigIntToNumber(identify_zeros 默认 Distinguish)
  pub const fn boolean_and_null_and_bigint_to_number(identify_zeros: IdentifyZeros) -> Self {
    Self::new(TruncationKind::BooleanAndNullAndBigIntToNumber, identify_zeros)
  }
  /// 原 Truncation::OddballAndBigIntToNumber(identify_zeros 默认 Distinguish)
  pub const fn oddball_and_bigint_to_number(identify_zeros: IdentifyZeros) -> Self {
    Self::new(TruncationKind::OddballAndBigIntToNumber, identify_zeros)
  }
  /// 原 Truncation::Any(identify_zeros, check_safe_integer)
  pub const fn any(identify_zeros: IdentifyZeros, check_safe_integer: bool) -> Self {
    Self { kind: TruncationKind::Any, identify_zeros, check_safe_integer }
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
    Self { kind, identify_zeros, check_safe_integer: false }
  }
}

/// 原 hash_value(Truncation) — hash_combine(kind, identify_zeros)
impl Truncation {
  pub fn hash_value(&self) -> usize {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    self.kind.hash(&mut h);
    self.identify_zeros.hash(&mut h);
    std::hash::Hasher::finish(&h) as usize
  }
}

/// 原 LessGeneral — kind 偏序 (representation-change.cc:111-140)
const fn less_general(rep1: TruncationKind, rep2: TruncationKind) -> bool {
  use TruncationKind as K;
  matches!(
    (rep1, rep2),
    (K::None, _)
      | (K::Bool, K::Bool | K::Any)
      | (K::Word32, K::Word32 | K::Word64 | K::OddballAndBigIntToNumber | K::BooleanAndNullAndBigIntToNumber | K::Any)
      | (K::Word64, K::Word64 | K::OddballAndBigIntToNumber | K::BooleanAndNullAndBigIntToNumber | K::Any)
      | (K::OddballAndBigIntToNumber, K::OddballAndBigIntToNumber | K::BooleanAndNullAndBigIntToNumber | K::Any)
      | (K::BooleanAndNullAndBigIntToNumber, K::BooleanAndNullAndBigIntToNumber | K::Any)
      | (K::Any, K::Any)
  )
}

/// 原 Generalize(TruncationKind) — 上确界 (representation-change.cc:82-99)
fn generalize_kind(rep1: TruncationKind, rep2: TruncationKind) -> Result<TruncationKind, crate::Error> {
  use TruncationKind as K;
  if less_general(rep1, rep2) {
    return Ok(rep2);
  }
  if less_general(rep2, rep1) {
    return Ok(rep1);
  }
  // 可 float64 表示值的泛化
  if less_general(rep1, K::OddballAndBigIntToNumber) && less_general(rep2, K::OddballAndBigIntToNumber) {
    return Ok(K::OddballAndBigIntToNumber);
  }
  // 任意可表示值的泛化
  if less_general(rep1, K::Any) && less_general(rep2, K::Any) {
    return Ok(K::Any);
  }
  // 其余组合非法 (原版 FATAL)
  Err(crate::Error::IncompatibleTruncation)
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
    || matches!((u1, u2), (IdentifyZeros::Distinguish, IdentifyZeros::Distinguish))
}

/// 原版 TypeCheckKind — 去检查类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeCheckKind {
  // ${tc}
  ${tcVariants.map((v) => v[0].toUpperCase() + v.slice(1)).join(",\n  ")},
}

impl TypeCheckKind {
  /// 原版 operator<< 文本
  pub const fn as_str(&self) -> &'static str {
    match self {
${tcVariants.map((v) => `      Self::${v[0].toUpperCase() + v.slice(1)} => "${v}",`).join("\n")}
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
    Self { vector: core::ptr::null(), slot: Self::INVALID_SLOT }
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

#[allow(non_snake_case)]
impl UseInfo {
  /// 原通用构造 (representation, truncation, type_check, feedback)
  pub const fn new(
    representation: MachineRepresentation,
    truncation: Truncation,
    type_check: TypeCheckKind,
    feedback: FeedbackSource,
  ) -> Self {
    Self { representation, truncation, type_check, feedback }
  }

  const fn with_check(
    representation: MachineRepresentation,
    truncation: Truncation,
    type_check: TypeCheckKind,
  ) -> Self {
    Self::new(representation, truncation, type_check, FeedbackSource::invalid())
  }

  use_info_ctor!(
    /// 原 TruncatingWord32()
    TruncatingWord32(MachineRepresentation::Word32, Truncation::word32())
  );
  use_info_ctor!(
    /// 原 TruncatingWord64()
    TruncatingWord64(MachineRepresentation::Word64, Truncation::word64())
  );

  /// 原 CheckedBigIntTruncatingWord64(feedback)
  pub const fn CheckedBigIntTruncatingWord64(feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Word64, Truncation::word64(), TypeCheckKind::BigInt).with_feedback(feedback)
  }
  /// 原 CheckedBigInt64AsWord64(feedback)
  pub const fn CheckedBigInt64AsWord64(feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Word64, Truncation::any(IdentifyZeros::Distinguish, false), TypeCheckKind::BigInt64).with_feedback(feedback)
  }
  /// 原 Word64(identify_zeros 默认 Distinguish)
  pub const fn Word64(identify_zeros: IdentifyZeros) -> Self {
    Self::with_check(MachineRepresentation::Word64, Truncation::any(identify_zeros, false), TypeCheckKind::None)
  }
  /// 原 Word() — 指针表示
  pub const fn Word() -> Self {
    Self::with_check(MachineType::PointerRepresentation(), Truncation::any(IdentifyZeros::Distinguish, false), TypeCheckKind::None)
  }
  use_info_ctor!(
    /// 原 Bool()
    Bool(MachineRepresentation::Bit, Truncation::boolean())
  );
  use_info_ctor!(
    /// 原 Float32()
    Float32(MachineRepresentation::Float32, Truncation::any(IdentifyZeros::Distinguish, false))
  );
  use_info_ctor!(
    /// 原 Float16RawBits()
    Float16RawBits(MachineRepresentation::Float16RawBits, Truncation::any(IdentifyZeros::Distinguish, false))
  );
  use_info_ctor!(
    /// 原 Float64()
    Float64(MachineRepresentation::Float64, Truncation::any(IdentifyZeros::Distinguish, false))
  );
  /// 原 TruncatingFloat64(identify_zeros 默认 Distinguish)
  pub const fn TruncatingFloat64(identify_zeros: IdentifyZeros) -> Self {
    Self::with_check(MachineRepresentation::Float64, Truncation::oddball_and_bigint_to_number(identify_zeros), TypeCheckKind::None)
  }
  /// 原 TruncatingFloat16RawBits(identify_zeros 默认 Distinguish)
  pub const fn TruncatingFloat16RawBits(identify_zeros: IdentifyZeros) -> Self {
    Self::with_check(MachineRepresentation::Float16, Truncation::oddball_and_bigint_to_number(identify_zeros), TypeCheckKind::None)
  }
  /// 原 CheckedSafeIntTruncatingWord32(feedback)
  pub const fn CheckedSafeIntTruncatingWord32(feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Word32, Truncation::word32_with_safe_int_check(), TypeCheckKind::AdditiveSafeInteger).with_feedback(feedback)
  }
  /// 原 CheckedSafeIntAsWord64(feedback)
  pub const fn CheckedSafeIntAsWord64(feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Word64, Truncation::any(IdentifyZeros::Distinguish, true), TypeCheckKind::AdditiveSafeInteger).with_feedback(feedback)
  }
  /// 原 TruncatingFloat64OrUndefined(identify_zeros 默认 Distinguish)
  pub const fn TruncatingFloat64OrUndefined(identify_zeros: IdentifyZeros) -> Self {
    Self::with_check(MachineRepresentation::Float64, Truncation::boolean_and_null_and_bigint_to_number(identify_zeros), TypeCheckKind::None)
  }
  use_info_ctor!(
    /// 原 AnyTagged()
    AnyTagged(MachineRepresentation::Tagged, Truncation::any(IdentifyZeros::Distinguish, false))
  );
  use_info_ctor!(
    /// 原 TaggedSigned()
    TaggedSigned(MachineRepresentation::TaggedSigned, Truncation::any(IdentifyZeros::Distinguish, false))
  );
  use_info_ctor!(
    /// 原 TaggedPointer()
    TaggedPointer(MachineRepresentation::TaggedPointer, Truncation::any(IdentifyZeros::Distinguish, false))
  );

  /// 原 CheckedTaggedAsArrayIndex(feedback)
  pub const fn CheckedTaggedAsArrayIndex(feedback: FeedbackSource) -> Self {
    Self::with_check(MachineType::PointerRepresentation(), Truncation::any(IdentifyZeros::Identify, false), TypeCheckKind::ArrayIndex).with_feedback(feedback)
  }
  /// 原 CheckedHeapObjectAsTaggedPointer(feedback)
  pub const fn CheckedHeapObjectAsTaggedPointer(feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::TaggedPointer, Truncation::any(IdentifyZeros::Distinguish, false), TypeCheckKind::HeapObject).with_feedback(feedback)
  }
  /// 原 CheckedBigIntAsTaggedPointer(feedback)
  pub const fn CheckedBigIntAsTaggedPointer(feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::TaggedPointer, Truncation::any(IdentifyZeros::Distinguish, false), TypeCheckKind::BigInt).with_feedback(feedback)
  }
  /// 原 CheckedSignedSmallAsTaggedSigned(feedback, identify_zeros 默认 Distinguish)
  pub const fn CheckedSignedSmallAsTaggedSigned(feedback: FeedbackSource, identify_zeros: IdentifyZeros) -> Self {
    Self::with_check(MachineRepresentation::TaggedSigned, Truncation::any(identify_zeros, false), TypeCheckKind::SignedSmall).with_feedback(feedback)
  }
  /// 原 CheckedSignedSmallAsWord32(identify_zeros, feedback)
  pub const fn CheckedSignedSmallAsWord32(identify_zeros: IdentifyZeros, feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Word32, Truncation::any(identify_zeros, false), TypeCheckKind::SignedSmall).with_feedback(feedback)
  }
  /// 原 CheckedSigned32AsWord32(identify_zeros, feedback)
  pub const fn CheckedSigned32AsWord32(identify_zeros: IdentifyZeros, feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Word32, Truncation::any(identify_zeros, false), TypeCheckKind::Signed32).with_feedback(feedback)
  }
  /// 原 CheckedSigned64AsWord64(identify_zeros, feedback)
  pub const fn CheckedSigned64AsWord64(identify_zeros: IdentifyZeros, feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Word64, Truncation::any(identify_zeros, false), TypeCheckKind::Signed64).with_feedback(feedback)
  }
  /// 原 CheckedUnsigned64AsWord64(identify_zeros, feedback)
  pub const fn CheckedUnsigned64AsWord64(identify_zeros: IdentifyZeros, feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Word64, Truncation::any(identify_zeros, false), TypeCheckKind::Unsigned64).with_feedback(feedback)
  }
  /// 原 CheckedNumberAsFloat64(identify_zeros, feedback)
  pub const fn CheckedNumberAsFloat64(identify_zeros: IdentifyZeros, feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Float64, Truncation::any(identify_zeros, false), TypeCheckKind::Number).with_feedback(feedback)
  }
  /// 原 CheckedNumberAsWord32(feedback)
  pub const fn CheckedNumberAsWord32(feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Word32, Truncation::word32(), TypeCheckKind::Number).with_feedback(feedback)
  }
  /// 原 CheckedNumberOrBooleanAsFloat64(identify_zeros, feedback)
  pub const fn CheckedNumberOrBooleanAsFloat64(identify_zeros: IdentifyZeros, feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Float64, Truncation::any(identify_zeros, false), TypeCheckKind::NumberOrBoolean).with_feedback(feedback)
  }
  /// 原 CheckedNumberOrOddballAsFloat64(identify_zeros, feedback)
  pub const fn CheckedNumberOrOddballAsFloat64(identify_zeros: IdentifyZeros, feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Float64, Truncation::any(identify_zeros, false), TypeCheckKind::NumberOrOddball).with_feedback(feedback)
  }
  /// 原 CheckedNumberOrOddballAsWord32(feedback)
  pub const fn CheckedNumberOrOddballAsWord32(feedback: FeedbackSource) -> Self {
    Self::with_check(MachineRepresentation::Word32, Truncation::word32(), TypeCheckKind::NumberOrOddball).with_feedback(feedback)
  }
  use_info_ctor!(
    /// 原 Any() — 表示未定
    Any(MachineRepresentation::None, Truncation::any(IdentifyZeros::Distinguish, false))
  );
  use_info_ctor!(
    /// 原 AnyTruncatingToBool()
    AnyTruncatingToBool(MachineRepresentation::None, Truncation::boolean())
  );
  use_info_ctor!(
    /// 原 None() — 值未用
    None(MachineRepresentation::None, Truncation::none())
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
`;

const p1 = writeSrc("v8-compiler-graph", "use_info.rs", useInfo);
const p2 = writeSrc("v8-compiler-graph", "machine_type.rs", machineType);
console.log(
  [
    `written ${p1}`,
    `written ${p2}`,
    `MachineRepresentation=${mrVariants.length} MachineSemantic=${msVariants.length} TruncationKind=${tkVariants.length} TypeCheckKind=${tcVariants.length}`,
  ].join("\n"),
);
