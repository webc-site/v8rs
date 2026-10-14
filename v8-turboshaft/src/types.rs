// 由 tool/src/compiler/turboshaft/types.h.js 从 v8/src/compiler/turboshaft/types.{h,cc} 生成; 修订就地改, 复跑覆盖
// Turboshaft 类型体系: Type::Kind/SubKind/Special 位标志与常量

/// 原版 Type::Kind — 类型大类
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TypeKind {
  /// kInvalid
  Invalid = 0,
  /// kNone
  None = 1,
  /// kWord32
  Word32 = 2,
  /// kWord64
  Word64 = 3,
  /// kFloat32
  Float32 = 4,
  /// kFloat64
  Float64 = 5,
  /// kTuple
  Tuple = 6,
  /// kAny
  Any = 7,
}

/// 原版 Type::ResolutionMode — 类型运算不可精表达时的策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ResolutionMode {
  /// kPreciseOrInvalid
  PreciseOrInvalid = 0,
  /// kOverApproximate
  OverApproximate = 1,
  /// kGreatestLowerBound
  GreatestLowerBound = 2,
}

/// 原版 WordType::SubKind — 字类型形态 (区间/集合)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum WordSubKind {
  /// kRange
  Range = 0,
  /// kSet
  Set = 1,
}

/// 原版 FloatType::SubKind — 浮点类型形态
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FloatSubKind {
  /// kRange
  Range = 0,
  /// kSet
  Set = 1,
  /// kOnlySpecialValues
  OnlySpecialValues = 2,
}

/// 原版 FloatType::Special — 特殊浮点值位标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Special {
  /// kNoSpecialValues = 0x0
  NoSpecialValues = 0x0,
  /// kNaN = 0x1
  NaN = 0x1,
  /// kMinusZero = 0x2
  MinusZero = 0x2,
}

/// 原版 kMaxInlineSetSize (Word/Float 同值, 集合内联容量)
pub const MAX_INLINE_SET_SIZE: usize = 2;

/// 原版 kMaxSetSize (集合最大元素数; 超出走 Payload_OutlineSet 外联)
pub const MAX_SET_SIZE: usize = 8;

// 原版 Payload 结构面 (detail 命名空间):
// - Payload_Empty { u8 } — 无负载 (Invalid/None/Any/Tuple)
// - Payload_Range<T> { min, max }
// - Payload_InlineSet<T> { T elements[2] }
// - Payload_OutlineSet<T> { T* array }
// Rust 侧以枚举 Payload 表达:
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Payload<T: Copy> {
  Empty,
  /// 原版 Payload_Range
  Range {
    min: T,
    max: T,
  },
  /// 原版 Payload_InlineSet (≤2 元素内联)
  InlineSet([T; 2]),
  /// 原版 Payload_OutlineSet (外联数组; Rust 侧以引用承载, Developer 修订)
  OutlineSet,
}

/// 原版 WordType<Bits>::KIND 映射 (32 → Word32, 64 → Word64)
#[must_use]
pub const fn word_kind(bits: u8) -> TypeKind {
  if bits == 32 {
    TypeKind::Word32
  } else {
    TypeKind::Word64
  }
}

/// 原版 FloatType<Bits>::KIND 映射 (32 → Float32, 64 → Float64)
#[must_use]
pub const fn float_kind(bits: u8) -> TypeKind {
  if bits == 32 {
    TypeKind::Float32
  } else {
    TypeKind::Float64
  }
}

/// 原版 next_smaller<T> (浮点: nextafter 向 -inf; 整型: v-1)
pub trait NextValue: Copy {
  /// 原版 next_smaller
  fn next_smaller(self) -> Self;
  /// 原版 next_larger
  fn next_larger(self) -> Self;
}

// v8: src/compiler/turboshaft/types.cc Type::LeastUpperBound
//
// 推迟面 (依赖类型格 Float64Range/Word32Set/Tuple + Zone 堆分配, 见 readme/zh.md 台账):
// - kInvalid/kNone/kAny → UNREACHABLE
// - kInvalid → InvalidType::LeastUpperBound
// - kNone → NoneType::LeastUpperBound
// - kAny → AnyType::LeastUpperBound
// - kWord32 → Word32Type::LeastUpperBound
// - kWord64 → Word64Type::LeastUpperBound
// - kFloat32 → Float32Type::LeastUpperBound
// - kFloat64 → Float64Type::LeastUpperBound
// - kTuple → TupleType::LeastUpperBound
//
// 原版 Type::AllocateOnHeap 分派面 (types.cc):
// - kInvalid / kNone / kWord32 / kWord64 / kFloat32 / kFloat64 / kTuple / kAny
//
// 骨架签名 (待类型格就位后填体):
// pub fn least_upper_bound(a: Type, b: Type) -> Type { ... }

// —— 提取不变式断言 ——
const _: () = {
  assert!(TypeKind::Invalid as u8 == 0);
  assert!(TypeKind::Any as u8 == 7);
  assert!(WordSubKind::Range as u8 == 0);
  assert!(WordSubKind::Set as u8 == 1);
  assert!(FloatSubKind::OnlySpecialValues as u8 == 2);
  assert!(Special::NoSpecialValues as u32 == 0x0);
  assert!(Special::NaN as u32 == 0x1);
  assert!(Special::MinusZero as u32 == 0x2);
  assert!(MAX_INLINE_SET_SIZE == 2);
  assert!(MAX_SET_SIZE == 8);
  assert!(matches!(word_kind(32), TypeKind::Word32));
  assert!(matches!(word_kind(64), TypeKind::Word64));
  assert!(matches!(float_kind(32), TypeKind::Float32));
  assert!(matches!(float_kind(64), TypeKind::Float64));
};
