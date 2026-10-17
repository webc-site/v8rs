// 由 tool/src/interpreter/bytecode-operands.h.js 从 v8/src/interpreter/bytecode-operands.{h,cc} 生成; 修订就地改, 复跑覆盖
// Ignition 操作数类型/刻度/累加器位标志体系 (clang -E 对数: OPERAND_TYPE_LIST=23, SCALE=3, INFO=5)

use std::ops::BitOr;

use crate::bytecode_traits::OPERAND_KIND_SIZES;

/// receiver 转换模式 (原版 src/common/globals.h ConvertReceiverMode; 供字节码 receiver 模式查询就近落位)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ReceiverMode {
  /// kNotNullOrUndefined: 调用方保证 receiver 非空非 undefined (CallProperty 族)
  NotNullOrUndefined = 0,
  /// kNullOrUndefined: receiver 可为 null/undefined (CallUndefinedReceiver 族)
  NullOrUndefined = 1,
  /// kAny: 任意 (CallAnyReceiver / Construct 族)
  Any = 2,
}

/// 操作数刻度 (原版 enum class OperandScale : uint8_t; 可缩放操作数的缩放系数)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OperandScale {
  /// 原版 kSingle
  Single = 1,
  /// 原版 kDouble
  Double = 2,
  /// 原版 kQuadruple
  Quadruple = 4,
}

impl OperandScale {
  /// 原版 kLast = kQuadruple
  pub const LAST: OperandScale = Self::Quadruple;
  /// 表索引 (原版 OperandScaleAsIndex: scale >> 1, Single->0 Double->1 Quadruple->2)
  #[inline]
  #[must_use]
  pub const fn as_index(self) -> usize {
    ((self as u8) >> 1) as usize
  }
  /// 原版名 (bytecode-operands.cc OperandScaleToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Single => "Single",
      Self::Double => "Double",
      Self::Quadruple => "Quadruple",
    }
  }
}

/// 操作数尺寸类 (原版 enum class OperandSize : uint8_t; 数值即字节数)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OperandSize {
  None = 0,
  Byte = 1,
  Short = 2,
  Quad = 4,
}

impl OperandSize {
  /// 原版 kLast = kQuad
  pub const LAST: OperandSize = Self::Quad;
  /// 原版名 (bytecode-operands.cc OperandSizeToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::None => "None",
      Self::Byte => "Byte",
      Self::Short => "Short",
      Self::Quad => "Quad",
    }
  }
}

/// 操作数元信息 (原版 enum class OperandTypeInfo : uint8_t; 列 = 可缩放/无符号/基准尺寸)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OperandTypeInfo {
  /// 可缩放=false 无符号=false 基准=None
  None,
  /// 可缩放=true 无符号=false 基准=Byte
  ScalableSignedByte,
  /// 可缩放=true 无符号=true 基准=Byte
  ScalableUnsignedByte,
  /// 可缩放=false 无符号=true 基准=Byte
  FixedUnsignedByte,
  /// 可缩放=false 无符号=true 基准=Short
  FixedUnsignedShort,
}

impl OperandTypeInfo {
  /// 是否可缩放 (bytecode-traits.h OperandTypeInfoTraits::kIsScalable)
  #[inline]
  #[must_use]
  pub const fn is_scalable(self) -> bool {
    matches!(self, Self::ScalableSignedByte | Self::ScalableUnsignedByte)
  }
  /// 是否无符号
  #[inline]
  #[must_use]
  pub const fn is_unsigned(self) -> bool {
    matches!(
      self,
      Self::ScalableUnsignedByte | Self::FixedUnsignedByte | Self::FixedUnsignedShort
    )
  }
  /// 未缩放基准尺寸 (OperandTypeInfoTraits::kUnscaledSize)
  #[inline]
  #[must_use]
  pub const fn unscaled_size(self) -> OperandSize {
    match self {
      Self::None => OperandSize::None,
      Self::ScalableSignedByte | Self::ScalableUnsignedByte | Self::FixedUnsignedByte => {
        OperandSize::Byte
      }
      Self::FixedUnsignedShort => OperandSize::Short,
    }
  }
}

/// 操作数类型全表 (原版 enum class OperandType : uint8_t; 顺序敏感 — 范围谓词依赖)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OperandType {
  /// 原版 kNone (None)
  None = 0,
  /// 原版 kFlag8 (FixedUnsignedByte)
  Flag8 = 1,
  /// 原版 kFlag16 (FixedUnsignedShort)
  Flag16 = 2,
  /// 原版 kIntrinsicId (FixedUnsignedByte)
  IntrinsicId = 3,
  /// 原版 kRuntimeId (FixedUnsignedShort)
  RuntimeId = 4,
  /// 原版 kNativeContextIndex (FixedUnsignedByte)
  NativeContextIndex = 5,
  /// 原版 kAbortReason (FixedUnsignedByte)
  AbortReason = 6,
  /// 原版 kEmbeddedFeedback (FixedUnsignedByte)
  EmbeddedFeedback = 7,
  /// 原版 kConstantPoolIndex (ScalableUnsignedByte)
  ConstantPoolIndex = 8,
  /// 原版 kFeedbackSlot (ScalableUnsignedByte)
  FeedbackSlot = 9,
  /// 原版 kContextSlot (ScalableUnsignedByte)
  ContextSlot = 10,
  /// 原版 kCoverageSlot (ScalableUnsignedByte)
  CoverageSlot = 11,
  /// 原版 kUImm (ScalableUnsignedByte)
  UImm = 12,
  /// 原版 kRegCount (ScalableUnsignedByte)
  RegCount = 13,
  /// 原版 kImm (ScalableSignedByte)
  Imm = 14,
  /// 原版 kReg (ScalableSignedByte)
  Reg = 15,
  /// 原版 kRegList (ScalableSignedByte)
  RegList = 16,
  /// 原版 kRegPair (ScalableSignedByte)
  RegPair = 17,
  /// 原版 kRegOut (ScalableSignedByte)
  RegOut = 18,
  /// 原版 kRegOutList (ScalableSignedByte)
  RegOutList = 19,
  /// 原版 kRegOutPair (ScalableSignedByte)
  RegOutPair = 20,
  /// 原版 kRegOutTriple (ScalableSignedByte)
  RegOutTriple = 21,
  /// 原版 kRegInOut (ScalableSignedByte)
  RegInOut = 22,
}

impl OperandType {
  /// 原版 kLast (COUNT_OPERAND_TYPES 求和 = 最后一个变体的值)
  pub const LAST: OperandType = Self::RegInOut;
  /// 操作数类型总数 (原版 BytecodeOperands::kOperandTypeCount = kLast + 1)
  pub const OPERAND_TYPE_COUNT: usize = 23;
  /// 对应元信息 (bytecode-traits.h OperandTraits::kOperandTypeInfo)
  #[inline]
  #[must_use]
  pub const fn type_info(self) -> OperandTypeInfo {
    OPERAND_TYPE_INFOS[self as usize]
  }
  /// 原版名 (bytecode-operands.cc OperandTypeToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::None => "None",
      Self::Flag8 => "Flag8",
      Self::Flag16 => "Flag16",
      Self::IntrinsicId => "IntrinsicId",
      Self::RuntimeId => "RuntimeId",
      Self::NativeContextIndex => "NativeContextIndex",
      Self::AbortReason => "AbortReason",
      Self::EmbeddedFeedback => "EmbeddedFeedback",
      Self::ConstantPoolIndex => "ConstantPoolIndex",
      Self::FeedbackSlot => "FeedbackSlot",
      Self::ContextSlot => "ContextSlot",
      Self::CoverageSlot => "CoverageSlot",
      Self::UImm => "UImm",
      Self::RegCount => "RegCount",
      Self::Imm => "Imm",
      Self::Reg => "Reg",
      Self::RegList => "RegList",
      Self::RegPair => "RegPair",
      Self::RegOut => "RegOut",
      Self::RegOutList => "RegOutList",
      Self::RegOutPair => "RegOutPair",
      Self::RegOutTriple => "RegOutTriple",
      Self::RegInOut => "RegInOut",
    }
  }

  #[inline]
  pub fn scale_for_operand(self, operand: u32) -> OperandScale {
    if BytecodeOperands::is_scalable_signed_byte(self) {
      BytecodeOperands::scale_for_signed_operand(operand as i32)
    } else if BytecodeOperands::is_scalable_unsigned_byte(self) {
      BytecodeOperands::scale_for_unsigned_operand(operand)
    } else {
      OperandScale::Single
    }
  }

  #[inline]
  pub fn size(self, scale: OperandScale) -> OperandSize {
    OPERAND_KIND_SIZES[scale.as_index()][self as usize]
  }
}

/// 每个操作数类型的元信息表 (OPERAND_TYPE_LIST 顺序)
pub static OPERAND_TYPE_INFOS: [OperandTypeInfo; OperandType::OPERAND_TYPE_COUNT] = [
  OperandTypeInfo::None,
  OperandTypeInfo::FixedUnsignedByte,
  OperandTypeInfo::FixedUnsignedShort,
  OperandTypeInfo::FixedUnsignedByte,
  OperandTypeInfo::FixedUnsignedShort,
  OperandTypeInfo::FixedUnsignedByte,
  OperandTypeInfo::FixedUnsignedByte,
  OperandTypeInfo::FixedUnsignedByte,
  OperandTypeInfo::ScalableUnsignedByte,
  OperandTypeInfo::ScalableUnsignedByte,
  OperandTypeInfo::ScalableUnsignedByte,
  OperandTypeInfo::ScalableUnsignedByte,
  OperandTypeInfo::ScalableUnsignedByte,
  OperandTypeInfo::ScalableUnsignedByte,
  OperandTypeInfo::ScalableSignedByte,
  OperandTypeInfo::ScalableSignedByte,
  OperandTypeInfo::ScalableSignedByte,
  OperandTypeInfo::ScalableSignedByte,
  OperandTypeInfo::ScalableSignedByte,
  OperandTypeInfo::ScalableSignedByte,
  OperandTypeInfo::ScalableSignedByte,
  OperandTypeInfo::ScalableSignedByte,
  OperandTypeInfo::ScalableSignedByte,
];

/// 非寄存器操作数 (原版 NON_REGISTER_OPERAND_TYPE_LIST, 共 15 种)
pub const NON_REGISTER_OPERAND_TYPES: [OperandType; 15] = [
  OperandType::None,
  OperandType::Flag8,
  OperandType::Flag16,
  OperandType::IntrinsicId,
  OperandType::RuntimeId,
  OperandType::NativeContextIndex,
  OperandType::AbortReason,
  OperandType::EmbeddedFeedback,
  OperandType::ConstantPoolIndex,
  OperandType::FeedbackSlot,
  OperandType::ContextSlot,
  OperandType::CoverageSlot,
  OperandType::UImm,
  OperandType::RegCount,
  OperandType::Imm,
];

/// 寄存器操作数 (原版 REGISTER_OPERAND_TYPE_LIST, 共 8 种)
pub const REGISTER_OPERAND_TYPES: [OperandType; 8] = [
  OperandType::Reg,
  OperandType::RegList,
  OperandType::RegPair,
  OperandType::RegOut,
  OperandType::RegOutList,
  OperandType::RegOutPair,
  OperandType::RegOutTriple,
  OperandType::RegInOut,
];

/// 寄存器输入操作数 (原版 REGISTER_INPUT_OPERAND_TYPE_LIST)
pub const REGISTER_INPUT_OPERAND_TYPES: [OperandType; 3] =
  [OperandType::Reg, OperandType::RegList, OperandType::RegPair];

/// 寄存器输出操作数 (原版 REGISTER_OUTPUT_OPERAND_TYPE_LIST)
pub const REGISTER_OUTPUT_OPERAND_TYPES: [OperandType; 4] = [
  OperandType::RegOut,
  OperandType::RegOutList,
  OperandType::RegOutPair,
  OperandType::RegOutTriple,
];

/// 累加器隐式读写语义位 (原版 enum class ImplicitRegisterUse : uint8_t)
/// 零依赖位标志: u8 新类型 + 命名常量 (对齐原版位布局)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImplicitRegisterUse(pub u8);

impl ImplicitRegisterUse {
  /// 无隐式累加器访问 (原版 kNone = 0)
  pub const NONE: Self = Self(0);
  /// 读累加器 (1 << 0)
  pub const READ_ACCUMULATOR: Self = Self(1 << 0);
  /// 写累加器 (1 << 1)
  pub const WRITE_ACCUMULATOR: Self = Self(1 << 1);
  /// 覆写 (旧值不可存活) (1 << 2)
  pub const CLOBBER_ACCUMULATOR: Self = Self(1 << 2);
  /// ShortStar 隐式写寄存器 (1 << 3)
  pub const WRITE_SHORT_STAR: Self = Self(1 << 3);
  /// 读 + 写 (原版 kReadWriteAccumulator)
  pub const READ_WRITE_ACCUMULATOR: Self =
    Self(Self::READ_ACCUMULATOR.0 | Self::WRITE_ACCUMULATOR.0);
  /// 读 + 覆写 (原版 kReadAndClobberAccumulator)
  pub const READ_AND_CLOBBER_ACCUMULATOR: Self =
    Self(Self::READ_ACCUMULATOR.0 | Self::CLOBBER_ACCUMULATOR.0);
  /// 读 + ShortStar 写 (原版 kReadAccumulatorWriteShortStar)
  pub const READ_ACCUMULATOR_WRITE_SHORT_STAR: Self =
    Self(Self::READ_ACCUMULATOR.0 | Self::WRITE_SHORT_STAR.0);

  /// 原始位值
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u8 {
    self.0
  }

  /// 是否包含全部指定位
  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }

  /// 是否有任一交叠位
  #[inline]
  #[must_use]
  pub const fn intersects(self, other: Self) -> bool {
    self.0 & other.0 != 0
  }

  /// 原版名 (bytecode-operands.cc ImplicitRegisterUseToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self.0 {
      0 => "None",
      1 => "ReadAccumulator",
      2 => "WriteAccumulator",
      4 => "ClobberAccumulator",
      8 => "WriteShortStar",
      5 => "ReadAndClobberAccumulator",
      3 => "ReadWriteAccumulator",
      9 => "ReadAccumulatorWriteShortStar",
      _ => "<组合>",
    }
  }
}

impl BitOr for ImplicitRegisterUse {
  type Output = Self;
  #[inline]
  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}

/// 操作数体系静态谓词集合 (原版 class BytecodeOperands : AllStatic)
pub struct BytecodeOperands;

impl BytecodeOperands {
  /// 操作数类型总数 = kLast + 1
  pub const OPERAND_TYPE_COUNT: usize = OperandType::OPERAND_TYPE_COUNT;
  /// 操作数刻度总数 (OPERAND_SCALE_COUNT 展开 = 3)
  pub const OPERAND_SCALE_COUNT: usize = 3;

  /// 刻度 -> 表索引 (原版 OperandScaleAsIndex)
  #[inline]
  #[must_use]
  pub const fn operand_scale_as_index(scale: OperandScale) -> usize {
    scale.as_index()
  }

  /// 是否读累加器
  #[inline]
  #[must_use]
  pub const fn reads_accumulator(use_: ImplicitRegisterUse) -> bool {
    use_.contains(ImplicitRegisterUse::READ_ACCUMULATOR)
  }

  /// 是否写累加器
  #[inline]
  #[must_use]
  pub const fn writes_accumulator(use_: ImplicitRegisterUse) -> bool {
    use_.contains(ImplicitRegisterUse::WRITE_ACCUMULATOR)
  }

  /// 是否覆写累加器
  #[inline]
  #[must_use]
  pub const fn clobbers_accumulator(use_: ImplicitRegisterUse) -> bool {
    use_.contains(ImplicitRegisterUse::CLOBBER_ACCUMULATOR)
  }

  /// 是否写或覆写累加器
  #[inline]
  #[must_use]
  pub const fn writes_or_clobbers_accumulator(use_: ImplicitRegisterUse) -> bool {
    use_.intersects(ImplicitRegisterUse(
      ImplicitRegisterUse::WRITE_ACCUMULATOR.0 | ImplicitRegisterUse::CLOBBER_ACCUMULATOR.0,
    ))
  }

  /// 是否隐式写寄存器 (ShortStar)
  #[inline]
  #[must_use]
  pub const fn writes_implicit_register(use_: ImplicitRegisterUse) -> bool {
    use_.contains(ImplicitRegisterUse::WRITE_SHORT_STAR)
  }

  /// 是否可缩放有符号字节操作数 (原版范围 kImm..=kRegInOut)
  #[inline]
  #[must_use]
  pub const fn is_scalable_signed_byte(t: OperandType) -> bool {
    (t as u8) >= OperandType::Imm as u8 && (t as u8) <= OperandType::RegInOut as u8
  }

  /// 是否可缩放无符号字节操作数 (原版范围 kConstantPoolIndex..=kRegCount)
  #[inline]
  #[must_use]
  pub const fn is_scalable_unsigned_byte(t: OperandType) -> bool {
    (t as u8) >= OperandType::ConstantPoolIndex as u8 && (t as u8) <= OperandType::RegCount as u8
  }

  #[inline]
  pub fn scale_for_signed_operand(v: i32) -> OperandScale {
    if v >= i8::MIN as i32 && v <= i8::MAX as i32 {
      OperandScale::Single
    } else if v >= i16::MIN as i32 && v <= i16::MAX as i32 {
      OperandScale::Double
    } else {
      OperandScale::Quadruple
    }
  }

  #[inline]
  pub fn scale_for_unsigned_operand(v: u32) -> OperandScale {
    if v <= u8::MAX as u32 {
      OperandScale::Single
    } else if v <= u16::MAX as u32 {
      OperandScale::Double
    } else {
      OperandScale::Quadruple
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(OperandType::OPERAND_TYPE_COUNT == 23);
  assert!(BytecodeOperands::OPERAND_SCALE_COUNT == 3);
  assert!(OperandType::Imm as u8 == 14);
  assert!(OperandType::RegInOut as u8 == 22);
  assert!(OperandType::ConstantPoolIndex as u8 == 8);
  assert!(OperandType::RegCount as u8 == 13);
  assert!(OperandScale::Quadruple as u8 == 4);
  assert!(OperandSize::Quad as u8 == 4);
  assert!(ImplicitRegisterUse::READ_WRITE_ACCUMULATOR.bits() == 3);
  assert!(ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR.bits() == 5);
  assert!(ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR.bits() == 9);
};
