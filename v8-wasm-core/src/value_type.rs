//! v8/src/wasm/value-type.h 的 1:1 转写 (名字表来自 value-type.cc)
//! ValueType/HeapType/CanonicalValueType/IndependentValueType 为 uint32 位域编码:
//!   bits 0-1 TypeKind | bit 2 nullable | bit 3 exact | bit 4 shared
//!   bits 5-7 RefTypeKind | bits 8-28 payload(索引或 StandardType) | bits 29-31 保留
//! 禁改位布局 — wasm 二进制互操作前提。
//! 登记差异:
//! - C++ 继承体系 (ValueTypeBase + 3 子类) → 共享查询方法的 trait ValueTypeBits
//!   + 4 个 newtype; Module/Canonical 索引的类型区分保留 (防混用)。
//! - MachineType/MachineRepresentation 取 codegen/machine-type.h 的最小子集
//!   (仅 value-type 所需), 后续迁移至 codegen crate。
//! - FunctionSig/CanonicalSig/Signature 依赖 codegen/signature.h → 推迟。
//! - AsExactIfEnabled 依赖 WasmEnabledFeatures::has_custom_descriptors → 推迟
//!   (flags 梯队); WasmReturnTypeFromSignature/GetI32Sig 依赖 Signature → 推迟。
// 由 tool/src/wasm/value-type.h.js 从 v8/src/wasm/value-type.h 生成; 修订就地改, 复跑覆盖
// X-macro 展开计算判别值; 对数见脚本运行输出
use std::{
  hash::{Hash, Hasher},
  ops::Deref,
};

use v8_base::bit_field::{BitField32, BitFieldCodec};
use v8_common::{K_SYSTEM_POINTER_SIZE, K_TAGGED_SIZE, K_TAGGED_SIZE_LOG2};

use crate::wasm_constants::ValueTypeCode;

/// 值类型字节表示上限 (kMaxValueTypeSize)
pub const K_MAX_VALUE_TYPE_SIZE: usize = 16;

// ---------------------------------------------------------------------------
// 类型索引: Module 级与 Canonical 级禁止互比 (原版注释), 故各自独立类型
// ---------------------------------------------------------------------------

macro_rules! type_index {
  ($name:ident, $doc:expr) => {
    #[doc = $doc]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct $name {
      pub index: u32,
    }

    impl $name {
      /// kInvalid = ~0u
      pub const INVALID: Self = Self { index: u32::MAX };

      #[inline]
      pub const fn new(index: u32) -> Self {
        Self { index }
      }

      /// constexpr bool valid()
      #[inline]
      pub const fn is_valid(self) -> bool {
        self.index != u32::MAX
      }
    }
  };
}

type_index!(ModuleTypeIndex, "模块内类型索引 (module-specific)");
type_index!(CanonicalTypeIndex, "规范化类型索引 (canonical)");

/// kNoSuperType: 类型层级顶端的父类型
pub const K_NO_SUPER_TYPE: ModuleTypeIndex = ModuleTypeIndex::INVALID;
/// kNoType
pub const K_NO_TYPE: ModuleTypeIndex = ModuleTypeIndex::INVALID;

/// 可空性 (enum Nullability : bool)
pub type Nullability = bool;
pub const K_NULLABLE: Nullability = true;
pub const K_NON_NULLABLE: Nullability = false;

/// 精确性 (enum Exactness : bool)
pub type Exactness = bool;
pub const K_EXACT: Exactness = true;
pub const K_ANY_SUBTYPE: Exactness = false;

// ---------------------------------------------------------------------------
// ValueKind: FOREACH_VALUE_TYPE 枚举 (序号与原版一致)
// ---------------------------------------------------------------------------

/// 粗粒度 kind (FOREACH_VALUE_TYPE 展开序)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueKind {
  Void = 0,
  I32 = 1,
  I64 = 2,
  F32 = 3,
  F64 = 4,
  S128 = 5,
  I8 = 6,
  I16 = 7,
  F16 = 8,
  Ref = 9,
  RefNull = 10,
  Top = 11,
  Bottom = 12,
}

impl ValueKind {
  /// 枚举序 → ValueKind (越界 → None)
  pub const fn from_u8(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::Void),
      1 => Some(Self::I32),
      2 => Some(Self::I64),
      3 => Some(Self::F32),
      4 => Some(Self::F64),
      5 => Some(Self::S128),
      6 => Some(Self::I8),
      7 => Some(Self::I16),
      8 => Some(Self::F16),
      9 => Some(Self::Ref),
      10 => Some(Self::RefNull),
      11 => Some(Self::Top),
      12 => Some(Self::Bottom),
      _ => None,
    }
  }
}

// ---------------------------------------------------------------------------
// TypeKind / RefTypeKind: 位域低 8 位的两个子域
// ---------------------------------------------------------------------------

/// 2 位粗分类 (bits 0-1); bit0=是否引用, bit1=引用是否有索引/是否哨兵
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeKind {
  Numeric = 0b00,     // i32 等数值 (FOREACH_NUMERIC_VALUE_TYPE)
  AbstractRef = 0b01, // anyref/eqref/funcref 等抽象引用
  Sentinel = 0b10,    // top/bottom/void 哨兵
  IndexedRef = 0b11,  // 带索引的用户定义类型引用
}

/// 引用类别 (bits 5-7); 非引用类型存 Other
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RefTypeKind {
  Other = 0,
  Function = 1,
  Struct = 2,
  Array = 3,
  Cont = 4,
  /// 解码越界哨兵 (原版 static_cast UB; is_valid 以 > LastValue 判非法)
  Invalid = 5,
}

impl RefTypeKind {
  /// kLastValue = kCont
  pub const LAST_VALUE: Self = Self::Cont;
}

impl BitFieldCodec for TypeKind {
  #[inline]
  fn to_u64(self) -> u64 {
    self as u64
  }
  #[inline]
  fn from_u64(v: u64) -> Self {
    match v & 0b11 {
      0 => Self::Numeric,
      1 => Self::AbstractRef,
      2 => Self::Sentinel,
      _ => Self::IndexedRef,
    }
  }
}

impl BitFieldCodec for RefTypeKind {
  #[inline]
  fn to_u64(self) -> u64 {
    self as u64
  }
  #[inline]
  fn from_u64(v: u64) -> Self {
    match v & 0b111 {
      0 => Self::Other,
      1 => Self::Function,
      2 => Self::Struct,
      3 => Self::Array,
      4 => Self::Cont,
      _ => Self::Invalid,
    }
  }
}

// ---------------------------------------------------------------------------
// 位域布局 (value_type_impl namespace)
// ---------------------------------------------------------------------------

/// ValueType 位域字段族 (对应 value_type_impl 的 BitField 别名)
pub mod fields {
  use super::{BitField32, RefTypeKind, TypeKind};

  /// bit 0: 是否某种引用
  pub type IsRef = BitField32<bool, 0, 1>;
  /// bit 1: 引用是否带索引 / 非引用是否内部哨兵
  pub type HasIndexOrSentinel = BitField32<bool, 1, 1>;
  /// bits 0-1: TypeKind (IsRef | HasIndexOrSentinel 的联合视图)
  pub type TypeKindF = BitField32<TypeKind, 0, 2>;
  /// bit 2: 引用可空性
  pub type IsNullable = BitField32<bool, 2, 1>;
  /// bit 3: 引用精确性
  pub type IsExact = BitField32<bool, 3, 1>;
  /// bit 4: 共享标志 (数值类型视为共享)
  pub type IsShared = BitField32<bool, 4, 1>;
  /// bits 5-7: 引用类别
  pub type RefTypeKindF = BitField32<RefTypeKind, 5, 3>;
  /// bits 8-28: 索引 (has_index) 或 StandardType
  pub type Payload = BitField32<u32, 8, 21>;
  /// bits 29-31: 保留
  pub type Reserved = BitField32<u32, 29, 3>;

  /// kGenericKindMask: HeapType 相关位 (payload | refKind | typeKind)
  pub const GENERIC_KIND_MASK: u32 = 0x1fffffe3;
  /// kNumericKindMask: 数值类型恒共享 → 额外含 shared 位
  pub const NUMERIC_KIND_MASK: u32 = 0x1ffffff3;
  /// kNumericExtraBits: IsSharedField::encode(true)
  pub const NUMERIC_EXTRA_BITS: u32 = 0x10;

  /// 关联位断言 (原版 static_assert 组; k_mask() 非 const → 位运算常量复核)
  const _: () = {
    assert!(TypeKindF::K_SHIFT == 0 && TypeKindF::K_SIZE == 2);
    assert!(IsNullable::K_SHIFT == 2 && IsExact::K_SHIFT == 3 && IsShared::K_SHIFT == 4);
    assert!(RefTypeKindF::K_SHIFT == 5 && RefTypeKindF::K_SIZE == 3);
    assert!(Payload::K_SHIFT == 8 && Payload::K_SIZE == 21);
    assert!(Reserved::K_SHIFT == 29 && Reserved::K_SIZE == 3);
    assert!(Reserved::K_SHIFT + Reserved::K_SIZE == 32);
    // kGenericKindMask == Payload | RefTypeKindF | TypeKindF 位联合
    assert!(GENERIC_KIND_MASK == ((1 << 21) - 1) << 8 | 0b111 << 5 | 0b11);
    assert!(NUMERIC_KIND_MASK == GENERIC_KIND_MASK | 1 << 4);
    assert!(NUMERIC_EXTRA_BITS == 1 << 4);
  };
}

// ---------------------------------------------------------------------------
// StandardType / GenericKind / NumericKind
// ---------------------------------------------------------------------------

/// 非索引类型的紧凑枚举, 值 = PayloadField 内容 (FOREACH_GENERIC_TYPE + NUMERIC 序)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StandardType {
  Void = 0,
  Top = 1,
  Bottom = 2,
  ExternString = 3,
  NoCont = 4,
  NoExn = 5,
  NoExtern = 6,
  NoFunc = 7,
  NoWaitqueue = 8,
  None = 9,
  Func = 10,
  Any = 11,
  Extern = 12,
  Exn = 13,
  Cont = 14,
  Waitqueue = 15,
  Eq = 16,
  I31 = 17,
  Struct = 18,
  Array = 19,
  String = 20,
  StringViewWtf8 = 21,
  StringViewWtf16 = 22,
  StringViewIter = 23,
  I32 = 24,
  I64 = 25,
  F32 = 26,
  F64 = 27,
  S128 = 28,
  I8 = 29,
  I16 = 30,
  F16 = 31,
}

impl StandardType {
  /// 序号 → 枚举 (越界 → None)
  pub const fn from_u32(v: u32) -> Option<Self> {
    match v {
      0 => Some(Self::Void),
      1 => Some(Self::Top),
      2 => Some(Self::Bottom),
      3 => Some(Self::ExternString),
      4 => Some(Self::NoCont),
      5 => Some(Self::NoExn),
      6 => Some(Self::NoExtern),
      7 => Some(Self::NoFunc),
      8 => Some(Self::NoWaitqueue),
      9 => Some(Self::None),
      10 => Some(Self::Func),
      11 => Some(Self::Any),
      12 => Some(Self::Extern),
      13 => Some(Self::Exn),
      14 => Some(Self::Cont),
      15 => Some(Self::Waitqueue),
      16 => Some(Self::Eq),
      17 => Some(Self::I31),
      18 => Some(Self::Struct),
      19 => Some(Self::Array),
      20 => Some(Self::String),
      21 => Some(Self::StringViewWtf8),
      22 => Some(Self::StringViewWtf16),
      23 => Some(Self::StringViewIter),
      24 => Some(Self::I32),
      25 => Some(Self::I64),
      26 => Some(Self::F32),
      27 => Some(Self::F64),
      28 => Some(Self::S128),
      29 => Some(Self::I8),
      30 => Some(Self::I16),
      31 => Some(Self::F16),
      _ => None,
    }
  }
}

/// 抽象引用 + 内部哨兵 (FOREACH_GENERIC_TYPE); 判别值 = Payload | extra bits
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericKind {
  Void = 0x2,               // std=0 "<void>"
  Top = 0x102,              // std=1 "<top>"
  Bottom = 0x202,           // std=2 "<bot>"
  ExternString = 0x301,     // std=3 "<extern_string>"
  NoCont = 0x481,           // std=4 "nocont"
  NoExn = 0x501,            // std=5 "noexn"
  NoExtern = 0x601,         // std=6 "noextern"
  NoFunc = 0x721,           // std=7 "nofunc"
  NoWaitqueue = 0x801,      // std=8 "nowaitqueue"
  None = 0x901,             // std=9 "none"
  Func = 0xa21,             // std=10 "func"
  Any = 0xb01,              // std=11 "any"
  Extern = 0xc01,           // std=12 "extern"
  Exn = 0xd01,              // std=13 "exn"
  Cont = 0xe81,             // std=14 "cont"
  Waitqueue = 0xf01,        // std=15 "waitqueue"
  Eq = 0x1001,              // std=16 "eq"
  I31 = 0x1101,             // std=17 "i31"
  Struct = 0x1241,          // std=18 "struct"
  Array = 0x1361,           // std=19 "array"
  String = 0x1401,          // std=20 "string"
  StringViewWtf8 = 0x1501,  // std=21 "stringview_wtf8"
  StringViewWtf16 = 0x1601, // std=22 "stringview_wtf16"
  StringViewIter = 0x1701,  // std=23 "stringview_iter"
}

impl GenericKind {
  /// 位域值 → 枚举 (未定义值 → None)
  pub const fn from_bits(v: u32) -> Option<Self> {
    match v & 0x1fffffe3 {
      0x2 => Some(Self::Void),
      0x102 => Some(Self::Top),
      0x202 => Some(Self::Bottom),
      0x301 => Some(Self::ExternString),
      0x481 => Some(Self::NoCont),
      0x501 => Some(Self::NoExn),
      0x601 => Some(Self::NoExtern),
      0x721 => Some(Self::NoFunc),
      0x801 => Some(Self::NoWaitqueue),
      0x901 => Some(Self::None),
      0xa21 => Some(Self::Func),
      0xb01 => Some(Self::Any),
      0xc01 => Some(Self::Extern),
      0xd01 => Some(Self::Exn),
      0xe81 => Some(Self::Cont),
      0xf01 => Some(Self::Waitqueue),
      0x1001 => Some(Self::Eq),
      0x1101 => Some(Self::I31),
      0x1241 => Some(Self::Struct),
      0x1361 => Some(Self::Array),
      0x1401 => Some(Self::String),
      0x1501 => Some(Self::StringViewWtf8),
      0x1601 => Some(Self::StringViewWtf16),
      0x1701 => Some(Self::StringViewIter),
      _ => None,
    }
  }

  /// 底层位域值
  #[inline]
  pub const fn bits(self) -> u32 {
    self as u32
  }
}

/// 数值类型 (FOREACH_NUMERIC_VALUE_TYPE); 判别值 = Payload | kNumericExtraBits
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericKind {
  I32 = 0x1810,  // log2size=2 "i32"
  I64 = 0x1910,  // log2size=3 "i64"
  F32 = 0x1a10,  // log2size=2 "f32"
  F64 = 0x1b10,  // log2size=3 "f64"
  S128 = 0x1c10, // log2size=4 "v128"
  I8 = 0x1d10,   // log2size=0 "i8"
  I16 = 0x1e10,  // log2size=1 "i16"
  F16 = 0x1f10,  // log2size=1 "f16"
}

impl NumericKind {
  /// 位域值 → 枚举 (未定义值 → None)
  pub const fn from_bits(v: u32) -> Option<Self> {
    match v & 0x1ffffff3 {
      0x1810 => Some(Self::I32),
      0x1910 => Some(Self::I64),
      0x1a10 => Some(Self::F32),
      0x1b10 => Some(Self::F64),
      0x1c10 => Some(Self::S128),
      0x1d10 => Some(Self::I8),
      0x1e10 => Some(Self::I16),
      0x1f10 => Some(Self::F16),
      _ => None,
    }
  }

  /// 底层位域值
  #[inline]
  pub const fn bits(self) -> u32 {
    self as u32
  }

  /// ToZeroBasedIndex: 稠密表下标 (StandardType 中 generic 在前)
  #[inline]
  pub const fn zero_based_index(self) -> u32 {
    (self as u32 >> 8) - 24
  }
}

/// 数值 kind 的稠密查找表下标界
pub const NUMERIC_KINDS: usize = 8;
/// 泛型 kind (抽象引用 + 哨兵) 数
pub const NUMERIC_LOOKUP_OFFSET: u32 = 24;

/// IsNullKind: FOREACH_NONE_TYPE (null 堆类型的引用无居民)
pub const fn is_null_kind(kind: GenericKind) -> bool {
  matches!(
    kind,
    GenericKind::NoCont
      | GenericKind::NoExn
      | GenericKind::NoExtern
      | GenericKind::NoFunc
      | GenericKind::NoWaitqueue
      | GenericKind::None
  )
}

// ---------------------------------------------------------------------------
// ValueTypeBits: 共享查询方法 (原版 ValueTypeBase 公共接口)
// ---------------------------------------------------------------------------

/// 类型无关查询方法族 (原版 ValueTypeBase); 4 个 newtype 共用
pub trait ValueTypeBits: Copy {
  /// 原始位域 (bit_field_)
  fn bit_field(self) -> u32;

  // --- 分类 ---
  #[inline]
  fn type_kind(self) -> TypeKind {
    fields::TypeKindF::decode(self.bit_field())
  }
  #[inline]
  fn has_index(self) -> bool {
    self.type_kind() == TypeKind::IndexedRef
  }
  #[inline]
  fn is_sentinel(self) -> bool {
    self.type_kind() == TypeKind::Sentinel
  }
  #[inline]
  fn is_numeric(self) -> bool {
    self.type_kind() == TypeKind::Numeric
  }
  #[inline]
  fn is_abstract_ref(self) -> bool {
    self.type_kind() == TypeKind::AbstractRef
  }
  /// 任意引用: 抽象或索引
  #[inline]
  fn is_ref(self) -> bool {
    fields::IsRef::decode(self.bit_field())
  }
  /// 任意 GenericKind: 抽象引用或内部哨兵
  #[inline]
  fn is_generic(self) -> bool {
    matches!(self.type_kind(), TypeKind::AbstractRef | TypeKind::Sentinel)
  }

  // --- 属性位 ---
  #[inline]
  fn nullability(self) -> Nullability {
    debug_assert!(self.is_ref());
    fields::IsNullable::decode(self.bit_field())
  }
  #[inline]
  fn is_nullable(self) -> bool {
    self.is_ref() && self.nullability()
  }
  #[inline]
  fn is_non_nullable(self) -> bool {
    self.is_ref() && !self.nullability()
  }
  #[inline]
  fn exactness(self) -> Exactness {
    fields::IsExact::decode(self.bit_field())
  }
  #[inline]
  fn is_exact(self) -> bool {
    self.exactness()
  }
  #[inline]
  fn is_shared(self) -> bool {
    fields::IsShared::decode(self.bit_field())
  }
  #[inline]
  fn ref_type_kind(self) -> RefTypeKind {
    fields::RefTypeKindF::decode(self.bit_field())
  }

  // --- 判别值 ---
  /// 未索引时 payload 即 StandardType (非法 payload → None)
  #[inline]
  fn standard_type(self) -> Option<StandardType> {
    debug_assert!(!self.has_index());
    match fields::Payload::decode(self.bit_field()) {
      v if v < 32 => StandardType::from_u32(v),
      _ => None,
    }
  }
  /// 数值判别值 (非数值 → None)
  #[inline]
  fn numeric_kind(self) -> Option<NumericKind> {
    debug_assert!(self.is_numeric());
    NumericKind::from_bits(self.bit_field())
  }
  /// 泛型判别值 (非泛型 → None)
  #[inline]
  fn generic_kind(self) -> Option<GenericKind> {
    debug_assert!(self.is_generic());
    GenericKind::from_bits(self.bit_field())
  }

  // --- 特定类型判断 (位比较, 与原版一致) ---
  #[inline]
  fn is_bottom(self) -> bool {
    self.bit_field() & fields::GENERIC_KIND_MASK == GenericKind::Bottom.bits()
  }
  #[inline]
  fn is_top(self) -> bool {
    self.bit_field() & fields::GENERIC_KIND_MASK == GenericKind::Top.bits()
  }
  #[inline]
  fn is_void(self) -> bool {
    self.bit_field() & fields::GENERIC_KIND_MASK == GenericKind::Void.bits()
  }
  /// none 或 bottom: (exact $t) 的子类型集
  #[inline]
  fn is_none_or_bottom(self) -> bool {
    let bits = self.bit_field() & fields::GENERIC_KIND_MASK;
    bits == GenericKind::None.bits() || bits == GenericKind::Bottom.bits()
  }
  #[inline]
  fn is_string_view(self) -> bool {
    let bits = self.bit_field() & fields::GENERIC_KIND_MASK;
    bits == GenericKind::StringViewWtf8.bits()
      || bits == GenericKind::StringViewWtf16.bits()
      || bits == GenericKind::StringViewIter.bits()
  }
  /// 打包类型 (i8/i16/f16)
  #[inline]
  fn is_packed(self) -> bool {
    matches!(
      self.numeric_kind(),
      Some(NumericKind::I8 | NumericKind::I16 | NumericKind::F16)
    )
  }
  #[inline]
  fn is_reference_to(self, kind: GenericKind) -> bool {
    self.is_abstract_ref() && self.generic_kind() == Some(kind)
  }
  /// 数值与可空引用可默认初始化
  #[inline]
  fn is_defaultable(self) -> bool {
    self.is_numeric() || self.is_nullable()
  }

  /// kind(): 位域 → 粗粒度 ValueKind (原版 ValueTypeBase::kind)
  #[inline]
  fn kind(self) -> ValueKind {
    if self.is_numeric() {
      return match self.numeric_kind() {
        Some(NumericKind::I32) => ValueKind::I32,
        Some(NumericKind::I64) => ValueKind::I64,
        Some(NumericKind::F32) => ValueKind::F32,
        Some(NumericKind::F64) => ValueKind::F64,
        Some(NumericKind::S128) => ValueKind::S128,
        Some(NumericKind::I8) => ValueKind::I8,
        Some(NumericKind::I16) => ValueKind::I16,
        Some(NumericKind::F16) => ValueKind::F16,
        // 原版 UNREACHABLE; 禁 panic → I32 兜底 (调用方错误输入)
        None => ValueKind::I32,
      };
    }
    if self.is_top() {
      return ValueKind::Top;
    }
    if self.is_bottom() {
      return ValueKind::Bottom;
    }
    if self.is_void() {
      return ValueKind::Void;
    }
    if self.is_nullable() {
      ValueKind::RefNull
    } else {
      ValueKind::Ref
    }
  }

  /// 无居民类型: bottom 或 非空 null 类型引用
  #[inline]
  fn is_uninhabited(self) -> bool {
    if self.is_bottom() {
      return true;
    }
    if self.is_defaultable() {
      return false; // 含全部可空引用
    }
    if self.has_index() {
      return false;
    }
    // 非空 null 类型引用无居民
    self.generic_kind().is_some_and(is_null_kind)
  }
  /// 是否使用 wasm null 表示 (extern 系例外)
  #[inline]
  fn use_wasm_null(self) -> bool {
    debug_assert!(self.is_ref());
    if self.has_index() {
      return true;
    }
    !matches!(
      self.generic_kind(),
      Some(GenericKind::Extern | GenericKind::ExternString | GenericKind::NoExtern)
    )
  }
  #[inline]
  fn is_none_type(self) -> bool {
    debug_assert!(!self.is_numeric());
    self.is_abstract_ref() && self.generic_kind().is_some_and(is_null_kind)
  }

  // --- 尺寸 ---
  /// log2(存储字节数); 引用 → kTaggedSizeLog2
  #[inline]
  fn value_kind_size_log2(self) -> u8 {
    debug_assert!(!self.is_sentinel());
    if self.is_ref() {
      return K_TAGGED_SIZE_LOG2 as u8;
    }
    K_VALUE_KIND_SIZE_LOG2[self
      .numeric_kind()
      .map(NumericKind::zero_based_index)
      .unwrap_or(0) as usize]
  }
  /// 存储字节数; 引用 → kTaggedSize
  #[inline]
  fn value_kind_size(self) -> u8 {
    debug_assert!(!self.is_sentinel());
    if self.is_ref() {
      return K_TAGGED_SIZE as u8;
    }
    K_VALUE_KIND_SIZE[self
      .numeric_kind()
      .map(NumericKind::zero_based_index)
      .unwrap_or(0) as usize]
  }
  /// 未压缩指针尺寸 (引用 → kSystemPointerSize)
  #[inline]
  fn value_kind_full_size(self) -> usize {
    if self.is_ref() {
      K_SYSTEM_POINTER_SIZE
    } else {
      self.value_kind_size() as usize
    }
  }

  // --- wire 编码 ---
  /// wasm 二进制首字节; 优先短编码 (如 kFuncRefCode)
  fn value_type_code(self) -> ValueTypeCode {
    if self.is_numeric() {
      return self.value_type_code_numeric();
    }
    if self.is_ref() && self.encoding_needs_heap_type() {
      return if self.is_nullable() {
        ValueTypeCode::RefNull
      } else {
        ValueTypeCode::Ref
      };
    }
    debug_assert!(!self.has_index()); // encoding_needs_heap_type 已处理
    self.value_type_code_generic()
  }
  /// 数值判别 → wire 码
  fn value_type_code_numeric(self) -> ValueTypeCode {
    match self.numeric_kind() {
      Some(NumericKind::I32) => ValueTypeCode::I32,
      Some(NumericKind::I64) => ValueTypeCode::I64,
      Some(NumericKind::F32) => ValueTypeCode::F32,
      Some(NumericKind::F64) => ValueTypeCode::F64,
      Some(NumericKind::S128) => ValueTypeCode::S128,
      Some(NumericKind::I8) => ValueTypeCode::I8,
      Some(NumericKind::I16) => ValueTypeCode::I16,
      Some(NumericKind::F16) => ValueTypeCode::F16,
      None => ValueTypeCode::Void, // 非法判别 (debug_assert 于 numeric_kind)
    }
  }
  /// 泛型判别 → wire 码 (哨兵中仅 void 有 wire 编码)
  fn value_type_code_generic(self) -> ValueTypeCode {
    match self.generic_kind() {
      Some(GenericKind::NoCont) => ValueTypeCode::NoCont,
      Some(GenericKind::NoExn) => ValueTypeCode::NoExn,
      Some(GenericKind::NoExtern) => ValueTypeCode::NoExtern,
      Some(GenericKind::NoFunc) => ValueTypeCode::NoFunc,
      Some(GenericKind::NoWaitqueue) => ValueTypeCode::NoWaitqueue,
      Some(GenericKind::None) => ValueTypeCode::None,
      Some(GenericKind::Func) => ValueTypeCode::FuncRef,
      Some(GenericKind::Any) => ValueTypeCode::AnyRef,
      Some(GenericKind::Extern) => ValueTypeCode::ExternRef,
      Some(GenericKind::Exn) => ValueTypeCode::ExnRef,
      Some(GenericKind::Cont) => ValueTypeCode::ContRef,
      Some(GenericKind::Waitqueue) => ValueTypeCode::WaitqueueRef,
      Some(GenericKind::Eq) => ValueTypeCode::EqRef,
      Some(GenericKind::I31) => ValueTypeCode::I31Ref,
      Some(GenericKind::Struct) => ValueTypeCode::StructRef,
      Some(GenericKind::Array) => ValueTypeCode::ArrayRef,
      Some(GenericKind::String) => ValueTypeCode::StringRef,
      Some(GenericKind::StringViewWtf8) => ValueTypeCode::StringViewWtf8,
      Some(GenericKind::StringViewWtf16) => ValueTypeCode::StringViewWtf16,
      Some(GenericKind::StringViewIter) => ValueTypeCode::StringViewIter,
      Some(GenericKind::Void) => ValueTypeCode::Void,
      _ => ValueTypeCode::Void, // Top/Bottom/ExternString 无 wire 编码 (原版 UNREACHABLE)
    }
  }
  /// 编码是否需要堆类型字节: 索引引用 / 非空或共享抽象引用
  #[inline]
  fn encoding_needs_heap_type(self) -> bool {
    if self.has_index() {
      return true;
    }
    if !self.is_abstract_ref() {
      return false;
    }
    !self.is_nullable() || self.is_shared()
  }
  #[inline]
  fn encoding_needs_shared(self) -> bool {
    self.is_abstract_ref() && self.is_shared()
  }
  #[inline]
  fn encoding_needs_exact(self) -> bool {
    self.is_exact()
  }

  // --- 短名 ---
  /// 单字符短名 (调试用)
  fn short_name(self) -> char {
    if self.is_ref() {
      return if self.is_nullable() { 'n' } else { 'r' };
    }
    if self.is_sentinel() {
      if self.is_void() {
        return 'v';
      }
      if self.is_top() {
        return '\\';
      }
      if self.is_bottom() {
        return '*';
      }
    }
    K_NUMERIC_SHORT_NAME[self
      .numeric_kind()
      .map(NumericKind::zero_based_index)
      .unwrap_or(0) as usize]
  }

  // --- 校验 ---
  /// 位域是否为合法编码 (原版 constexpr is_valid)
  fn is_valid(self) -> bool {
    let payload = fields::Payload::decode(self.bit_field());
    if self.is_numeric() {
      // 数值类型 payload 之外固定位必须恰好是 kNumericExtraBits
      if self.bit_field() & !fields::Payload::k_mask() != fields::NUMERIC_EXTRA_BITS {
        return false;
      }
      return (24..32).contains(&payload);
    }
    if self.ref_type_kind() > RefTypeKind::LAST_VALUE {
      return false;
    }
    if !self.has_index() {
      // 泛型类型必须在预定义集合内; 仅索引类型可 exact
      if payload >= 24 {
        return false;
      }
      if self.is_exact() {
        return false;
      }
    }
    // 泛型与索引引用都必须保留位为零
    fields::Reserved::decode(self.bit_field()) == 0
  }

  /// Populate: 解码器先按索引建对象, 后补 shared/refKind 位
  #[inline]
  fn populate(&mut self, shared: bool, kind: RefTypeKind) {
    let bits = fields::IsShared::update(self.bit_field(), shared);
    let bits = fields::RefTypeKindF::update(bits, kind);
    self.set_bit_field(bits);
  }

  /// 内部位域写入 (populate 用)
  fn set_bit_field(&mut self, bits: u32);

  /// raw_bit_field()
  #[inline]
  fn raw_bit_field(self) -> u32 {
    self.bit_field()
  }
}

/// 数值 kind 稠密表: log2 尺寸 (FOREACH_NUMERIC_VALUE_TYPE log2Size 列)
static K_VALUE_KIND_SIZE_LOG2: [u8; 8] = [2, 3, 2, 3, 4, 0, 1, 1];
/// 数值 kind 稠密表: 字节数 (1 << log2Size)
static K_VALUE_KIND_SIZE: [u8; 8] = [4, 8, 4, 8, 16, 1, 2, 2];
/// 数值 kind 稠密表: 单字符短名
static K_NUMERIC_SHORT_NAME: [char; 8] = ['i', 'l', 'f', 'd', 's', 'b', 'h', 'p'];

// ---------------------------------------------------------------------------
// newtype: ValueType / CanonicalValueType (构造宏共享)
// ---------------------------------------------------------------------------

macro_rules! wasm_type_newtype {
  ($name:ident, $doc:expr) => {
    #[doc = $doc]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct $name(u32);

    impl ValueTypeBits for $name {
      #[inline]
      fn bit_field(self) -> u32 {
        self.0
      }
      #[inline]
      fn set_bit_field(&mut self, bits: u32) {
        self.0 = bits;
      }
    }

    impl $name {
      /// 位域值 (const fn; ValueTypeBits::raw_bit_field 同语义)
      #[inline]
      pub const fn bits(self) -> u32 {
        self.0
      }

      /// FromRawBitField (不校验; 调用方保证或先行 is_valid)
      #[inline]
      pub const fn from_raw_bit_field(bits: u32) -> Self {
        Self(bits)
      }

      /// Primitive(NumericKind)
      #[inline]
      pub const fn primitive(kind: NumericKind) -> Self {
        Self(kind.bits())
      }

      /// Generic(kind, nullable, shared)
      /// (const fn 不能调 BitField::encode → 手写位装配; 位布局由 fields 断言保证)
      #[inline]
      pub const fn generic(kind: GenericKind, nullable: Nullability, shared: bool) -> Self {
        Self(kind.bits() | ((nullable as u32) << 2) | ((shared as u32) << 4))
      }

      /// 索引引用底层装配 (原版 ValueTypeBase(TypeIndex, nullable, exact, shared, refKind))
      #[inline]
      pub const fn indexed(index: u32, nullable: Nullability, exact: Exactness, shared: bool, ref_kind: RefTypeKind) -> Self {
        debug_assert!(index < (1u32 << 21));
        Self(
          TypeKind::IndexedRef as u32 // bits 0-1
            | ((nullable as u32) << 2)
            | ((exact as u32) << 3)
            | ((shared as u32) << 4)
            | ((ref_kind as u32) << 5)
            | (index << 8),
        )
      }

      /// AsNullable
      #[inline]
      pub const fn as_nullable(self, nullable: Nullability) -> Self {
        Self((self.0 & !(1 << 2)) | ((nullable as u32) << 2))
      }
      /// AsExact
      #[inline]
      pub const fn as_exact(self, exact: Exactness) -> Self {
        Self((self.0 & !(1 << 3)) | ((exact as u32) << 3))
      }
      /// shared 位更新 (AsNonShared 的补)
      #[inline]
      pub const fn as_shared(self, shared: bool) -> Self {
        Self((self.0 & !(1 << 4)) | ((shared as u32) << 4))
      }
    }
  };
}

wasm_type_newtype!(ValueType, "模块索引值类型 (module-specific type indices)");
wasm_type_newtype!(
  CanonicalValueType,
  "规范化索引值类型 (canonicalized type indices)"
);

// ---------------------------------------------------------------------------
// HeapType: is_numeric 恒 false; nullable 位忽略 (等值比较掩掉)
// ---------------------------------------------------------------------------

/// 堆类型 (ValueType 的子集视图: 非数值; nullable 位无意义)
#[derive(Debug, Clone, Copy, Eq)]
pub struct HeapType(u32);

impl ValueTypeBits for HeapType {
  #[inline]
  fn bit_field(self) -> u32 {
    self.0
  }
  #[inline]
  fn set_bit_field(&mut self, bits: u32) {
    self.0 = bits;
  }
}

/// 等值: 掩掉 is_ref 与 nullable 位 (原版 operator== 掩码);
/// Hash 同掩码 — 原版 hash_value 用 raw 与 == 不一致, 登记差异修正
impl PartialEq for HeapType {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    let mask = !(fields::IsRef::k_mask() | fields::IsNullable::k_mask());
    (self.0 & mask) == (other.0 & mask)
  }
}

impl Hash for HeapType {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    let mask = !(fields::IsRef::k_mask() | fields::IsNullable::k_mask());
    (self.0 & mask).hash(state);
  }
}

impl HeapType {
  /// 位域值 (const fn; ValueTypeBits::raw_bit_field 同语义)
  #[inline]
  pub const fn bits(self) -> u32 {
    self.0
  }

  /// Generic(kind, shared) (const fn 手写位装配)
  #[inline]
  pub const fn generic(kind: GenericKind, shared: bool) -> Self {
    Self(kind.bits() | (1 << 2) | ((shared as u32) << 4))
  }

  /// nullable 位重置 (HeapType 内部位操作; 语义上 nullable 位无效)
  #[inline]
  pub const fn as_nullable(self, nullable: Nullability) -> Self {
    Self((self.0 & !(1 << 2)) | ((nullable as u32) << 2))
  }

  /// Index(index, shared, kind, exact)
  #[inline]
  pub const fn index(
    index: ModuleTypeIndex,
    shared: bool,
    kind: RefTypeKind,
    exact: Exactness,
  ) -> Self {
    Self(ValueType::indexed(index.index, K_NULLABLE, exact, shared, kind).bits())
  }

  /// FromBits (不校验; 须非数值)
  #[inline]
  pub const fn from_bits(bits: u32) -> Self {
    Self(bits)
  }

  /// from_code: wire 堆类型码 → 堆类型。
  /// 界外码 → Bottom (丢弃 shared; 原版 kFirst/kLast 前置检查);
  /// 界内未分配码 → Bottom (保留 shared; 原版 lookup 表 default 项)
  pub const fn from_code(code: u8, shared: bool) -> Self {
    use crate::wasm_constants::{FIRST_HEAP_TYPE_CODE, LAST_HEAP_TYPE_CODE};
    if code < FIRST_HEAP_TYPE_CODE || code > LAST_HEAP_TYPE_CODE {
      return Self::generic(GenericKind::Bottom, false);
    }
    let kind = match code {
      0x5b => GenericKind::NoWaitqueue,
      0x5c => GenericKind::Waitqueue,
      0x60 => GenericKind::StringViewWtf16,
      0x61 => GenericKind::StringViewIter,
      0x66 => GenericKind::StringViewWtf8,
      0x67 => GenericKind::String,
      0x68 => GenericKind::Cont,
      0x69 => GenericKind::Exn,
      0x6a => GenericKind::Array,
      0x6b => GenericKind::Struct,
      0x6c => GenericKind::I31,
      0x6d => GenericKind::Eq,
      0x6e => GenericKind::Any,
      0x6f => GenericKind::Extern,
      0x70 => GenericKind::Func,
      0x71 => GenericKind::None,
      0x72 => GenericKind::NoExtern,
      0x73 => GenericKind::NoFunc,
      0x74 => GenericKind::NoExn,
      0x75 => GenericKind::NoCont,
      _ => GenericKind::Bottom,
    };
    Self::generic(kind, shared)
  }

  /// AsExact (kAnySubtype 仅索引类型允许)
  #[inline]
  pub const fn as_exact(self, exact: Exactness) -> Self {
    Self((self.0 & !(1 << 3)) | ((exact as u32) << 3))
  }

  /// wire 码或索引; 抽象码符号扩展 (LEB128 首字节 7→32 位)
  #[inline]
  pub fn code(self) -> i32 {
    if self.has_index() {
      return ((self.0 >> 8) & 0x1F_FFFF) as i32;
    }
    (0xFFFF_FF80u32 | self.value_type_code_generic() as u32) as i32
  }

  /// 模块类型索引
  #[inline]
  pub const fn ref_index(self) -> ModuleTypeIndex {
    debug_assert!((self.0 & 0b11) == 0b11);
    ModuleTypeIndex::new((self.0 >> 8) & 0x1F_FFFF)
  }

  /// 堆类型名 (数字或 exact 前缀)
  pub fn name(self) -> String {
    if self.has_index() {
      if self.is_exact() {
        return format!("exact {}", fields::Payload::decode(self.0));
      }
      return fields::Payload::decode(self.0).to_string();
    }
    generic_heaptype_name_prefixed(self.0)
  }

  /// generic_heaptype_name()
  pub fn generic_heaptype_name(self) -> String {
    generic_heaptype_name_prefixed(self.0)
  }
}

// ---------------------------------------------------------------------------
// IndependentValueType / IndependentHeapType: 全局常量载体 (可转三者)
// ---------------------------------------------------------------------------

/// 非索引全局常量类型 (与 ValueType/CanonicalValueType/HeapType 互转)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndependentValueType(u32);

impl ValueTypeBits for IndependentValueType {
  #[inline]
  fn bit_field(self) -> u32 {
    self.0
  }
  #[inline]
  fn set_bit_field(&mut self, bits: u32) {
    self.0 = bits;
  }
}

impl IndependentValueType {
  /// 位域值 (const fn)
  #[inline]
  pub const fn bits(self) -> u32 {
    self.0
  }

  /// FromRawBitField
  #[inline]
  pub const fn from_raw_bit_field(bits: u32) -> Self {
    Self(bits)
  }

  #[inline]
  pub const fn primitive(kind: NumericKind) -> Self {
    Self(kind.bits())
  }

  #[inline]
  pub const fn generic(kind: GenericKind, nullable: Nullability, shared: bool) -> Self {
    Self(kind.bits() | ((nullable as u32) << 2) | ((shared as u32) << 4))
  }

  /// name()
  pub fn name(self) -> String {
    name_common(self.0)
  }
}

impl From<IndependentValueType> for ValueType {
  #[inline]
  fn from(v: IndependentValueType) -> Self {
    Self(v.bits())
  }
}
impl From<IndependentValueType> for CanonicalValueType {
  #[inline]
  fn from(v: IndependentValueType) -> Self {
    Self(v.bits())
  }
}

/// 独立堆类型常量 (默认可空/非共享)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndependentHeapType(IndependentValueType);

impl Deref for IndependentHeapType {
  type Target = IndependentValueType;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl ValueTypeBits for IndependentHeapType {
  #[inline]
  fn bit_field(self) -> u32 {
    self.0.bits()
  }
  #[inline]
  fn set_bit_field(&mut self, bits: u32) {
    self.0 = IndependentValueType::from_raw_bit_field(bits);
  }
}

impl IndependentHeapType {
  /// 位域值 (const fn)
  #[inline]
  pub const fn bits(self) -> u32 {
    self.0.bits()
  }

  #[inline]
  pub const fn new(kind: GenericKind, nullable: Nullability, shared: bool) -> Self {
    Self(IndependentValueType::generic(kind, nullable, shared))
  }

  /// AsNonNull
  #[inline]
  pub const fn as_non_null(self) -> Self {
    let shared = (self.bits() & (1 << 4)) != 0;
    match GenericKind::from_bits(self.bits() & fields::GENERIC_KIND_MASK) {
      Some(k) => Self::new(k, K_NON_NULLABLE, shared),
      None => Self::new(GenericKind::Bottom, K_NON_NULLABLE, shared),
    }
  }

  /// heap_type()
  #[inline]
  pub const fn heap_type(self) -> HeapType {
    HeapType::from_bits(self.bits())
  }

  /// name()
  pub fn name(self) -> String {
    name_common(self.bits())
  }
}

impl From<IndependentHeapType> for HeapType {
  #[inline]
  fn from(v: IndependentHeapType) -> HeapType {
    v.heap_type()
  }
}
impl From<IndependentHeapType> for ValueType {
  #[inline]
  fn from(v: IndependentHeapType) -> Self {
    Self(v.bits())
  }
}
impl From<IndependentHeapType> for CanonicalValueType {
  #[inline]
  fn from(v: IndependentHeapType) -> Self {
    Self(v.bits())
  }
}

// ---------------------------------------------------------------------------
// ValueType / CanonicalValueType 特有构造 (索引引用族)
// ---------------------------------------------------------------------------

impl ValueType {
  /// Ref(index, shared, kind): 非空索引引用
  #[inline]
  pub const fn r#ref(index: ModuleTypeIndex, shared: bool, kind: RefTypeKind) -> Self {
    Self::indexed(index.index, K_NON_NULLABLE, K_ANY_SUBTYPE, shared, kind)
  }

  /// RefNull(index, shared, kind)
  #[inline]
  pub const fn ref_null(index: ModuleTypeIndex, shared: bool, kind: RefTypeKind) -> Self {
    Self::indexed(index.index, K_NULLABLE, K_ANY_SUBTYPE, shared, kind)
  }

  /// RefMaybeNull(index, nullable, shared, kind)
  #[inline]
  pub const fn ref_maybe_null(
    index: ModuleTypeIndex,
    nullable: Nullability,
    shared: bool,
    kind: RefTypeKind,
  ) -> Self {
    Self::indexed(index.index, nullable, K_ANY_SUBTYPE, shared, kind)
  }

  /// Ref(HeapType): 堆类型 → 非空引用
  #[inline]
  pub const fn from_heap_type(t: HeapType) -> Self {
    Self(t.as_nullable(K_NON_NULLABLE).bits())
  }

  /// RefNull(HeapType)
  #[inline]
  pub const fn from_heap_type_nullable(t: HeapType) -> Self {
    Self(t.as_nullable(K_NULLABLE).bits())
  }

  /// RefMaybeNull(HeapType, nullable)
  #[inline]
  pub const fn from_heap_type_maybe_null(t: HeapType, nullable: Nullability) -> Self {
    Self(t.as_nullable(nullable).bits())
  }

  /// AsNonNull
  #[inline]
  pub const fn as_non_null(self) -> Self {
    self.as_nullable(K_NON_NULLABLE)
  }

  /// AsExactIfIndexed
  #[inline]
  pub const fn as_exact_if_indexed(self, exact: Exactness) -> Self {
    if (self.bits() & 0b11) == 0b11 {
      self.as_exact(exact)
    } else {
      self
    }
  }

  /// AsNonShared
  #[inline]
  pub const fn as_non_shared(self) -> Self {
    if self.bits() & 1 != 0 {
      self.as_shared(false)
    } else {
      self
    }
  }

  /// Unpacked: i8/i16 → i32, f16 → f32
  #[inline]
  pub const fn unpacked(self) -> Self {
    match self.bits() {
      0x1d10 => Self::primitive(NumericKind::I32),
      0x1e10 => Self::primitive(NumericKind::I32),
      0x1f10 => Self::primitive(NumericKind::F32),
      _ => self,
    }
  }

  /// Canonicalize(index): 模块索引 → 规范索引 (const fn 手写 payload 位替换)
  #[inline]
  pub const fn canonicalize(self, index: CanonicalTypeIndex) -> CanonicalValueType {
    debug_assert!((self.bits() & 0b11) == 0b11);
    CanonicalValueType::from_raw_bit_field(
      (self.bits() & !(((1u32 << 21) - 1) << 8)) | (index.index << 8),
    )
  }

  /// heap_type()
  #[inline]
  pub const fn heap_type(self) -> HeapType {
    debug_assert!(self.bits() & 0b11 != 0b00);
    HeapType::from_bits(self.bits())
  }

  /// ref_index()
  #[inline]
  pub const fn ref_index(self) -> ModuleTypeIndex {
    debug_assert!((self.bits() & 0b11) == 0b11);
    ModuleTypeIndex::new((self.bits() >> 8) & 0x1F_FFFF)
  }

  /// Primitive(ValueKind) (增量过渡)
  pub const fn primitive_kind(kind: ValueKind) -> Self {
    match kind {
      ValueKind::I32 => Self::primitive(NumericKind::I32),
      ValueKind::I64 => Self::primitive(NumericKind::I64),
      ValueKind::F32 => Self::primitive(NumericKind::F32),
      ValueKind::F64 => Self::primitive(NumericKind::F64),
      ValueKind::S128 => Self::primitive(NumericKind::S128),
      ValueKind::I8 => Self::primitive(NumericKind::I8),
      ValueKind::I16 => Self::primitive(NumericKind::I16),
      ValueKind::F16 => Self::primitive(NumericKind::F16),
      ValueKind::Void => Self::generic(GenericKind::Void, K_NON_NULLABLE, false),
      // 原版 UNREACHABLE; 禁 panic → Bottom 兜底 (调用方错误输入)
      _ => Self::generic(GenericKind::Bottom, K_NON_NULLABLE, false),
    }
  }

  /// For(MachineType)
  pub const fn for_machine_type(t: MachineType) -> Self {
    match t.rep {
      MachineRepresentation::Word8
      | MachineRepresentation::Word16
      | MachineRepresentation::Word32 => Self::primitive(NumericKind::I32),
      MachineRepresentation::Word64 => Self::primitive(NumericKind::I64),
      MachineRepresentation::Float32 => Self::primitive(NumericKind::F32),
      MachineRepresentation::Float64 => Self::primitive(NumericKind::F64),
      MachineRepresentation::AnyTagged => Self::generic(GenericKind::Any, K_NULLABLE, false),
      MachineRepresentation::Simd128 => Self::primitive(NumericKind::S128),
      MachineRepresentation::Float16 => Self::primitive(NumericKind::F32),
      MachineRepresentation::None => Self::generic(GenericKind::Bottom, K_NON_NULLABLE, false),
    }
  }

  /// name()
  pub fn name(self) -> String {
    name_common(self.bits())
  }
}

impl CanonicalValueType {
  /// Ref(index, shared, kind)
  #[inline]
  pub const fn r#ref(index: CanonicalTypeIndex, shared: bool, kind: RefTypeKind) -> Self {
    Self::indexed(index.index, K_NON_NULLABLE, K_ANY_SUBTYPE, shared, kind)
  }

  /// RefNull(index, shared, kind)
  #[inline]
  pub const fn ref_null(index: CanonicalTypeIndex, shared: bool, kind: RefTypeKind) -> Self {
    Self::indexed(index.index, K_NULLABLE, K_ANY_SUBTYPE, shared, kind)
  }

  /// RefMaybeNull(type, nullable)
  #[inline]
  pub const fn ref_maybe_null(t: Self, nullable: Nullability) -> Self {
    t.as_nullable(nullable)
  }

  /// ref_index()
  #[inline]
  pub const fn ref_index(self) -> CanonicalTypeIndex {
    debug_assert!((self.bits() & 0b11) == 0b11);
    CanonicalTypeIndex::new((self.bits() >> 8) & 0x1F_FFFF)
  }

  /// 除索引外全部位 (哈希用)
  #[inline]
  pub const fn all_bits_without_index(self) -> u32 {
    self.bits() & !(((1u32 << 21) - 1) << 8)
  }

  /// 除索引外等值
  #[inline]
  pub const fn is_equal_except_index(self, other: Self) -> bool {
    self.all_bits_without_index() == other.all_bits_without_index()
  }

  /// IsFunctionType()
  #[inline]
  pub const fn is_function_type(self) -> bool {
    (self.bits() >> 5) & 0b111 == RefTypeKind::Function as u32
  }

  /// name()
  pub fn name(self) -> String {
    name_common(self.bits())
  }
}

// ---------------------------------------------------------------------------
// 名字表公共实现 (value-type.cc name / generic_heaptype_name)
// ---------------------------------------------------------------------------

/// generic_heaptype_name 带 shared 前缀
pub(crate) fn generic_heaptype_name_prefixed(bits: u32) -> String {
  let t = ValueType::from_raw_bit_field(bits);
  let base = match GenericKind::from_bits(bits) {
    Some(GenericKind::Void) => "<void>",
    Some(GenericKind::Top) => "<top>",
    Some(GenericKind::Bottom) => "<bot>",
    Some(GenericKind::ExternString) => "<extern_string>",
    Some(GenericKind::NoCont) => "nocont",
    Some(GenericKind::NoExn) => "noexn",
    Some(GenericKind::NoExtern) => "noextern",
    Some(GenericKind::NoFunc) => "nofunc",
    Some(GenericKind::NoWaitqueue) => "nowaitqueue",
    Some(GenericKind::None) => "none",
    Some(GenericKind::Func) => "func",
    Some(GenericKind::Any) => "any",
    Some(GenericKind::Extern) => "extern",
    Some(GenericKind::Exn) => "exn",
    Some(GenericKind::Cont) => "cont",
    Some(GenericKind::Waitqueue) => "waitqueue",
    Some(GenericKind::Eq) => "eq",
    Some(GenericKind::I31) => "i31",
    Some(GenericKind::Struct) => "struct",
    Some(GenericKind::Array) => "array",
    Some(GenericKind::String) => "string",
    Some(GenericKind::StringViewWtf8) => "stringview_wtf8",
    Some(GenericKind::StringViewWtf16) => "stringview_wtf16",
    Some(GenericKind::StringViewIter) => "stringview_iter",
    None => "<invalid>",
  };
  if t.is_shared() {
    format!("shared {base}")
  } else {
    base.into()
  }
}

/// name() 公共实现
pub(crate) fn name_common(bits: u32) -> String {
  let t = ValueType::from_raw_bit_field(bits);
  if t.is_numeric() {
    return match t.numeric_kind() {
      Some(NumericKind::I32) => "i32".into(),
      Some(NumericKind::I64) => "i64".into(),
      Some(NumericKind::F32) => "f32".into(),
      Some(NumericKind::F64) => "f64".into(),
      Some(NumericKind::S128) => "v128".into(),
      Some(NumericKind::I8) => "i8".into(),
      Some(NumericKind::I16) => "i16".into(),
      Some(NumericKind::F16) => "f16".into(),
      None => "<invalid>".into(),
    };
  }
  if t.has_index() {
    let mut s = String::from("(ref ");
    if t.is_nullable() {
      s.push_str("null ");
    }
    if t.is_exact() {
      s.push_str("exact ");
    }
    s.push_str(&fields::Payload::decode(bits).to_string());
    s.push(')');
    return s;
  }
  // 泛型; 可空 null 堆类型走简写特例 (nullref 族; 原版特判五种, NoWaitqueue 走通用名)
  if t.is_nullable()
    && let Some(k) = t.generic_kind()
    && is_null_kind(k)
  {
    let n = match k {
      GenericKind::NoCont => Some("nullcontref"),
      GenericKind::NoExn => Some("nullexnref"),
      GenericKind::NoExtern => Some("nullexternref"),
      GenericKind::NoFunc => Some("nullfuncref"),
      GenericKind::None => Some("nullref"),
      _ => None,
    };
    if let Some(n) = n {
      return if t.is_shared() {
        format!("(shared {n})")
      } else {
        n.into()
      };
    }
  }
  let mut buf = String::new();
  let shorthand = (t.is_nullable() || t.is_sentinel() || t.is_string_view()) && !t.is_exact();
  let append_ref = t.is_nullable() && !t.is_sentinel() && !t.is_exact();
  if !shorthand {
    buf.push_str("(ref ");
  }
  if !shorthand && t.is_nullable() {
    buf.push_str("null ");
  }
  buf.push_str(&generic_heaptype_name_prefixed(bits));
  if append_ref {
    buf.push_str("ref");
  }
  if !shorthand {
    buf.push(')');
  }
  buf
}

// ---------------------------------------------------------------------------
// ValueKind 辅助 (增量过渡; 原 free functions)
// ---------------------------------------------------------------------------

/// is_reference(kind)
#[inline]
pub const fn is_reference(kind: ValueKind) -> bool {
  matches!(kind, ValueKind::Ref | ValueKind::RefNull)
}

/// machine_type(kind) (FOREACH_VALUE_TYPE machineType 列; 原版 DCHECK_NE(kBottom, kind))
pub const fn machine_type(kind: ValueKind) -> MachineType {
  match kind {
    // Void/Top/Bottom → None (原版表列 None)
    ValueKind::Void | ValueKind::Top | ValueKind::Bottom => MachineType::none(),
    ValueKind::I32 => MachineType::int32(),
    ValueKind::I64 => MachineType::int64(),
    ValueKind::F32 => MachineType::float32(),
    ValueKind::F64 => MachineType::float64(),
    ValueKind::S128 => MachineType::simd128(),
    ValueKind::I8 => MachineType::int8(),
    ValueKind::I16 => MachineType::int16(),
    ValueKind::F16 => MachineType::float16(),
    ValueKind::Ref | ValueKind::RefNull => MachineType::any_tagged(),
  }
}

/// value_kind_size_log2(kind) (哨兵 -1 → None)
#[inline]
pub const fn value_kind_size_log2(kind: ValueKind) -> Option<u8> {
  match kind {
    ValueKind::Void => None,
    ValueKind::I32 => Some(2),
    ValueKind::I64 => Some(3),
    ValueKind::F32 => Some(2),
    ValueKind::F64 => Some(3),
    ValueKind::S128 => Some(4),
    ValueKind::I8 => Some(0),
    ValueKind::I16 => Some(1),
    ValueKind::F16 => Some(1),
    ValueKind::Ref => Some(2),
    ValueKind::RefNull => Some(2),
    ValueKind::Top => None,
    ValueKind::Bottom => None,
  }
}

/// value_kind_size(kind)
#[inline]
pub const fn value_kind_size(kind: ValueKind) -> Option<u8> {
  match value_kind_size_log2(kind) {
    Some(l) => Some(1 << l),
    None => None,
  }
}

/// value_kind_full_size(kind) (引用 → kSystemPointerSize)
#[inline]
pub const fn value_kind_full_size(kind: ValueKind) -> Option<usize> {
  if is_reference(kind) {
    Some(K_SYSTEM_POINTER_SIZE)
  } else {
    match value_kind_size(kind) {
      Some(s) => Some(s as usize),
      None => None,
    }
  }
}

/// name(kind) (FOREACH_VALUE_TYPE typeName 列)
pub const fn kind_name(kind: ValueKind) -> &'static str {
  match kind {
    ValueKind::Void => "<void>",
    ValueKind::I32 => "i32",
    ValueKind::I64 => "i64",
    ValueKind::F32 => "f32",
    ValueKind::F64 => "f64",
    ValueKind::S128 => "v128",
    ValueKind::I8 => "i8",
    ValueKind::I16 => "i16",
    ValueKind::F16 => "f16",
    ValueKind::Ref => "ref",
    ValueKind::RefNull => "ref null",
    ValueKind::Top => "<top>",
    ValueKind::Bottom => "<bot>",
  }
}

/// is_packed(kind)
#[inline]
pub const fn is_packed(kind: ValueKind) -> bool {
  matches!(kind, ValueKind::I8 | ValueKind::I16 | ValueKind::F16)
}

/// unpacked(kind): i8/i16 → i32, f16 → f32
#[inline]
pub const fn unpacked(kind: ValueKind) -> ValueKind {
  match kind {
    ValueKind::I8 | ValueKind::I16 => ValueKind::I32,
    ValueKind::F16 => ValueKind::F32,
    _ => kind,
  }
}

// ---------------------------------------------------------------------------
// 预定义全局类型常量 (kWasm*; value-type.h:1144-1208)
// ---------------------------------------------------------------------------
pub const K_WASM_I32: IndependentValueType = IndependentValueType::primitive(NumericKind::I32);
pub const K_WASM_I64: IndependentValueType = IndependentValueType::primitive(NumericKind::I64);
pub const K_WASM_F32: IndependentValueType = IndependentValueType::primitive(NumericKind::F32);
pub const K_WASM_F64: IndependentValueType = IndependentValueType::primitive(NumericKind::F64);
pub const K_WASM_S128: IndependentValueType = IndependentValueType::primitive(NumericKind::S128);
pub const K_WASM_I8: IndependentValueType = IndependentValueType::primitive(NumericKind::I8);
pub const K_WASM_I16: IndependentValueType = IndependentValueType::primitive(NumericKind::I16);
pub const K_WASM_F16: IndependentValueType = IndependentValueType::primitive(NumericKind::F16);

pub const K_WASM_VOID: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Void, K_NON_NULLABLE, false);
pub const K_WASM_TOP: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Top, K_NULLABLE, false);
pub const K_WASM_BOTTOM: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Bottom, K_NON_NULLABLE, false);
pub const K_WASM_FUNC_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Func, K_NULLABLE, false);
pub const K_WASM_ANY_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Any, K_NULLABLE, false);
pub const K_WASM_SHARED_ANY_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Any, K_NULLABLE, true);
pub const K_WASM_EXTERN_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Extern, K_NULLABLE, false);
pub const K_WASM_REF_EXTERN: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Extern, K_NON_NULLABLE, false);
pub const K_WASM_SHARED_EXTERN_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Extern, K_NULLABLE, true);
pub const K_WASM_EXN_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Exn, K_NULLABLE, false);
pub const K_WASM_SHARED_EXN_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Exn, K_NULLABLE, true);
pub const K_WASM_EQ_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Eq, K_NULLABLE, false);
pub const K_WASM_SHARED_EQ_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Eq, K_NULLABLE, true);
pub const K_WASM_I31_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::I31, K_NULLABLE, false);
pub const K_WASM_REF_I31: IndependentHeapType =
  IndependentHeapType::new(GenericKind::I31, K_NON_NULLABLE, false);
pub const K_WASM_STRUCT_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Struct, K_NULLABLE, false);
pub const K_WASM_ARRAY_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Array, K_NULLABLE, false);
pub const K_WASM_WAITQUEUE_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Waitqueue, K_NULLABLE, true);
pub const K_WASM_NULL_WAITQUEUE_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::NoWaitqueue, K_NULLABLE, true);
pub const K_WASM_STRING_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::String, K_NULLABLE, false);
pub const K_WASM_REF_STRING: IndependentHeapType =
  IndependentHeapType::new(GenericKind::String, K_NON_NULLABLE, false);
pub const K_WASM_REF_NULL_EXTERN_STRING: IndependentHeapType =
  IndependentHeapType::new(GenericKind::ExternString, K_NULLABLE, false);
pub const K_WASM_REF_EXTERN_STRING: IndependentHeapType =
  IndependentHeapType::new(GenericKind::ExternString, K_NON_NULLABLE, false);
pub const K_WASM_REF_NULL_SHARED_EXTERN_STRING: IndependentHeapType =
  IndependentHeapType::new(GenericKind::ExternString, K_NULLABLE, true);
pub const K_WASM_REF_SHARED_EXTERN_STRING: IndependentHeapType =
  IndependentHeapType::new(GenericKind::ExternString, K_NON_NULLABLE, true);
pub const K_WASM_STRING_VIEW_WTF8: IndependentHeapType =
  IndependentHeapType::new(GenericKind::StringViewWtf8, K_NON_NULLABLE, false);
pub const K_WASM_STRING_VIEW_WTF16: IndependentHeapType =
  IndependentHeapType::new(GenericKind::StringViewWtf16, K_NON_NULLABLE, false);
pub const K_WASM_STRING_VIEW_ITER: IndependentHeapType =
  IndependentHeapType::new(GenericKind::StringViewIter, K_NON_NULLABLE, false);
pub const K_WASM_NULL_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::None, K_NULLABLE, false);
pub const K_WASM_SHARED_NULL_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::None, K_NULLABLE, true);
pub const K_WASM_REF_NONE: IndependentHeapType =
  IndependentHeapType::new(GenericKind::None, K_NON_NULLABLE, false);
pub const K_WASM_NULL_EXTERN_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::NoExtern, K_NULLABLE, false);
pub const K_WASM_NULL_EXN_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::NoExn, K_NULLABLE, false);
pub const K_WASM_NULL_FUNC_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::NoFunc, K_NULLABLE, false);
pub const K_WASM_CONT_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::Cont, K_NULLABLE, false);
pub const K_WASM_NULL_CONT_REF: IndependentHeapType =
  IndependentHeapType::new(GenericKind::NoCont, K_NULLABLE, false);

// ---------------------------------------------------------------------------
// MachineType 最小子集 (源: codegen/machine-type.h; 后续迁 codegen crate)
// ---------------------------------------------------------------------------

/// 机器表示 (value-type 所需子集)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineRepresentation {
  None,
  Word8,
  Word16,
  Word32,
  Word64,
  Float16,
  Float32,
  Float64,
  Simd128,
  AnyTagged,
}

/// ElementSizeLog2Of
#[inline]
pub const fn element_size_log2(rep: MachineRepresentation) -> u8 {
  match rep {
    MachineRepresentation::None => 0,
    MachineRepresentation::Word8 => 0,
    MachineRepresentation::Word16 | MachineRepresentation::Float16 => 1,
    MachineRepresentation::Word32 | MachineRepresentation::Float32 => 2,
    MachineRepresentation::Word64 | MachineRepresentation::Float64 => 3,
    MachineRepresentation::Simd128 => 4,
    MachineRepresentation::AnyTagged => K_TAGGED_SIZE_LOG2 as u8,
  }
}

/// ElementSizeInBytes
#[inline]
pub const fn element_size(rep: MachineRepresentation) -> u32 {
  1u32 << element_size_log2(rep)
}

/// MachineType {representation, signed} (最小子集)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MachineType {
  pub rep: MachineRepresentation,
  pub signed: bool,
}

impl MachineType {
  #[inline]
  pub const fn none() -> Self {
    Self {
      rep: MachineRepresentation::None,
      signed: false,
    }
  }
  #[inline]
  pub const fn any_tagged() -> Self {
    Self {
      rep: MachineRepresentation::AnyTagged,
      signed: false,
    }
  }
  #[inline]
  pub const fn int8() -> Self {
    Self {
      rep: MachineRepresentation::Word8,
      signed: true,
    }
  }
  #[inline]
  pub const fn uint8() -> Self {
    Self {
      rep: MachineRepresentation::Word8,
      signed: false,
    }
  }
  #[inline]
  pub const fn int16() -> Self {
    Self {
      rep: MachineRepresentation::Word16,
      signed: true,
    }
  }
  #[inline]
  pub const fn uint16() -> Self {
    Self {
      rep: MachineRepresentation::Word16,
      signed: false,
    }
  }
  #[inline]
  pub const fn int32() -> Self {
    Self {
      rep: MachineRepresentation::Word32,
      signed: true,
    }
  }
  #[inline]
  pub const fn uint32() -> Self {
    Self {
      rep: MachineRepresentation::Word32,
      signed: false,
    }
  }
  #[inline]
  pub const fn int64() -> Self {
    Self {
      rep: MachineRepresentation::Word64,
      signed: true,
    }
  }
  #[inline]
  pub const fn uint64() -> Self {
    Self {
      rep: MachineRepresentation::Word64,
      signed: false,
    }
  }
  #[inline]
  pub const fn float16() -> Self {
    Self {
      rep: MachineRepresentation::Float16,
      signed: false,
    }
  }
  #[inline]
  pub const fn float32() -> Self {
    Self {
      rep: MachineRepresentation::Float32,
      signed: false,
    }
  }
  #[inline]
  pub const fn float64() -> Self {
    Self {
      rep: MachineRepresentation::Float64,
      signed: false,
    }
  }
  #[inline]
  pub const fn simd128() -> Self {
    Self {
      rep: MachineRepresentation::Simd128,
      signed: false,
    }
  }
}

impl ValueType {
  /// machine_type(): 哨兵 → None, 引用 → AnyTagged, 数值查表
  pub fn machine_type(self) -> MachineType {
    if self.is_sentinel() {
      return MachineType::none();
    }
    if self.is_ref() {
      return MachineType::any_tagged();
    }
    K_MACHINE_TYPE[self
      .numeric_kind()
      .map(NumericKind::zero_based_index)
      .unwrap_or(0) as usize]
  }

  /// machine_representation()
  pub fn machine_representation(self) -> MachineRepresentation {
    self.machine_type().rep
  }
}

/// 数值 kind 机器类型表 (FOREACH_NUMERIC_VALUE_TYPE machineType 列)
const K_MACHINE_TYPE: [MachineType; 8] = [
  MachineType::int32(),   // I32
  MachineType::int64(),   // I64
  MachineType::float32(), // F32
  MachineType::float64(), // F64
  MachineType::simd128(), // S128
  MachineType::int8(),    // I8
  MachineType::int16(),   // I16
  MachineType::float16(), // F16
];

// ---------------------------------------------------------------------------
// LoadType (FOREACH_LOAD_TYPE)
// ---------------------------------------------------------------------------

/// 内存读取类型枚举
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoadTypeValue {
  I32Load,
  I32Load8S,
  I32Load8U,
  I32Load16S,
  I32Load16U,
  I64Load,
  I64Load8S,
  I64Load8U,
  I64Load16S,
  I64Load16U,
  I64Load32S,
  I64Load32U,
  F32LoadF16,
  F32Load,
  F64Load,
  S128Load,
}

/// LoadType 包装 (value/size/memtype 查表)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadType(pub LoadTypeValue);

impl LoadType {
  /// size_log_2()
  #[inline]
  pub const fn size_log_2(self) -> u8 {
    K_LOAD_SIZE_LOG2[self.0 as usize]
  }
  /// size()
  #[inline]
  pub const fn size(self) -> u8 {
    K_LOAD_SIZE[self.0 as usize]
  }
  /// value_type()
  #[inline]
  pub const fn value_type(self) -> ValueType {
    K_LOAD_VALUE_TYPE[self.0 as usize]
  }
  /// mem_type()
  #[inline]
  pub const fn mem_type(self) -> MachineType {
    K_LOAD_MEM_TYPE[self.0 as usize]
  }

  /// ForValueKind(kind, is_signed)
  pub const fn for_value_kind(kind: ValueKind, is_signed: bool) -> Self {
    match kind {
      ValueKind::I32 => Self(LoadTypeValue::I32Load),
      ValueKind::I64 => Self(LoadTypeValue::I64Load),
      ValueKind::F32 => Self(LoadTypeValue::F32Load),
      ValueKind::F64 => Self(LoadTypeValue::F64Load),
      ValueKind::S128 => Self(LoadTypeValue::S128Load),
      ValueKind::I8 => Self(if is_signed {
        LoadTypeValue::I32Load8S
      } else {
        LoadTypeValue::I32Load8U
      }),
      ValueKind::I16 => Self(if is_signed {
        LoadTypeValue::I32Load16S
      } else {
        LoadTypeValue::I32Load16U
      }),
      ValueKind::F16 => Self(LoadTypeValue::F32LoadF16),
      // 原版 UNREACHABLE; 禁 panic → I32Load 兜底
      _ => Self(LoadTypeValue::I32Load),
    }
  }
}

const K_LOAD_SIZE: [u8; 16] = [4, 1, 1, 2, 2, 8, 1, 1, 2, 2, 4, 4, 2, 4, 8, 16];
const K_LOAD_SIZE_LOG2: [u8; 16] = [2, 0, 0, 1, 1, 3, 0, 0, 1, 1, 2, 2, 1, 2, 3, 4];
const K_LOAD_VALUE_TYPE: [ValueType; 16] = [
  ValueType::primitive(NumericKind::I32),
  ValueType::primitive(NumericKind::I32),
  ValueType::primitive(NumericKind::I32),
  ValueType::primitive(NumericKind::I32),
  ValueType::primitive(NumericKind::I32),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::F32),
  ValueType::primitive(NumericKind::F32),
  ValueType::primitive(NumericKind::F64),
  ValueType::primitive(NumericKind::S128),
];
const K_LOAD_MEM_TYPE: [MachineType; 16] = [
  MachineType::int32(),
  MachineType::int8(),
  MachineType::uint8(),
  MachineType::int16(),
  MachineType::uint16(),
  MachineType::int64(),
  MachineType::int8(),
  MachineType::uint8(),
  MachineType::int16(),
  MachineType::uint16(),
  MachineType::int32(),
  MachineType::uint32(),
  MachineType::float16(),
  MachineType::float32(),
  MachineType::float64(),
  MachineType::simd128(),
];

// ---------------------------------------------------------------------------
// StoreType (FOREACH_STORE_TYPE)
// ---------------------------------------------------------------------------

/// 内存存储类型枚举
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreTypeValue {
  I32Store,
  I32Store8,
  I32Store16,
  I64Store,
  I64Store8,
  I64Store16,
  I64Store32,
  F32StoreF16,
  F32Store,
  F64Store,
  S128Store,
}

/// StoreType 包装
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreType(pub StoreTypeValue);

impl StoreType {
  /// size_log_2()
  #[inline]
  pub const fn size_log_2(self) -> u8 {
    K_STORE_SIZE_LOG2[self.0 as usize]
  }
  /// size()
  #[inline]
  pub const fn size(self) -> u8 {
    1 << self.size_log_2()
  }
  /// value_type()
  #[inline]
  pub const fn value_type(self) -> ValueType {
    K_STORE_VALUE_TYPE[self.0 as usize]
  }
  /// mem_rep()
  #[inline]
  pub const fn mem_rep(self) -> MachineRepresentation {
    K_STORE_MEM_REP[self.0 as usize]
  }

  /// ForValueKind(kind)
  pub const fn for_value_kind(kind: ValueKind) -> Self {
    match kind {
      ValueKind::I32 => Self(StoreTypeValue::I32Store),
      ValueKind::I64 => Self(StoreTypeValue::I64Store),
      ValueKind::F32 => Self(StoreTypeValue::F32Store),
      ValueKind::F64 => Self(StoreTypeValue::F64Store),
      ValueKind::S128 => Self(StoreTypeValue::S128Store),
      ValueKind::I8 => Self(StoreTypeValue::I32Store8),
      ValueKind::I16 => Self(StoreTypeValue::I32Store16),
      ValueKind::F16 => Self(StoreTypeValue::F32StoreF16),
      // 原版 UNREACHABLE; 禁 panic → I32Store 兜底
      _ => Self(StoreTypeValue::I32Store),
    }
  }
}

const K_STORE_SIZE_LOG2: [u8; 11] = [2, 0, 1, 3, 0, 1, 2, 1, 2, 3, 4];
const K_STORE_VALUE_TYPE: [ValueType; 11] = [
  ValueType::primitive(NumericKind::I32),
  ValueType::primitive(NumericKind::I32),
  ValueType::primitive(NumericKind::I32),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::I64),
  ValueType::primitive(NumericKind::F32),
  ValueType::primitive(NumericKind::F32),
  ValueType::primitive(NumericKind::F64),
  ValueType::primitive(NumericKind::S128),
];
const K_STORE_MEM_REP: [MachineRepresentation; 11] = [
  MachineRepresentation::Word32,
  MachineRepresentation::Word8,
  MachineRepresentation::Word16,
  MachineRepresentation::Word64,
  MachineRepresentation::Word8,
  MachineRepresentation::Word16,
  MachineRepresentation::Word32,
  MachineRepresentation::Float16,
  MachineRepresentation::Float32,
  MachineRepresentation::Float64,
  MachineRepresentation::Simd128,
];
