// 由 tool/src/codegen/code-stub-assembler.h.js 从 v8/src/codegen/code-stub-assembler.{h,-inl.h,cc} 生成; 修订就地改, 复跑覆盖
// CSA 枚举与骨架

// 语义面: CSA 896+ 个 TNode 原语 (cc 22156 行)
// 登记跳过, 由 Developer 按原语表分批补齐。此处产出规则枚举与宏族登记。

/// 分配标志 (原 enum class AllocationFlag : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AllocationFlag {
  None = 0,
  DoubleAlignment = 1,
  Pretenured = 1 << 1,
}

/// 位集别名 (原 base::Flags<AllocationFlag>)
pub type AllocationFlags = u8;

/// 松弛追踪模式 (原 enum SlackTrackingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SlackTrackingMode {
  /// kWithSlackTracking
  WithSlackTracking = 0,
  /// kNoSlackTracking
  NoSlackTracking = 1,
  /// kDontInitializeInObjectProperties
  DontInitializeInObjectProperties = 2,
}

/// 数组拷贝选择标志 (原 enum class ExtractFixedArrayFlag, 位集)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtractFixedArrayFlag(pub u8);

impl ExtractFixedArrayFlag {
  /// 原 kFixedArrays = 1
  pub const FIXED_ARRAYS: Self = Self(1);
  /// 原 kFixedDoubleArrays = 2
  pub const FIXED_DOUBLE_ARRAYS: Self = Self(2);
  /// 原 kDontCopyCOW = 4
  pub const DONT_COPY_COW: Self = Self(4);
  /// 原 kAllFixedArrays = kFixedArrays | kFixedDoubleArrays
  pub const ALL_FIXED_ARRAYS: Self = Self(Self::FIXED_ARRAYS.0 | Self::FIXED_DOUBLE_ARRAYS.0);
  /// 原 kAllFixedArraysDontCopyCOW = kAllFixedArrays | kDontCopyCOW
  pub const ALL_FIXED_ARRAYS_DONT_COPY_COW: Self =
    Self(Self::ALL_FIXED_ARRAYS.0 | Self::DONT_COPY_COW.0);

  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    (self.0 & other.0) == other.0
  }
}

/// 洞值转换模式 (原 enum class HoleConversionMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HoleConversionMode {
  /// kDontConvert
  DontConvert = 0,
  /// kConvertToUndefined
  ConvertToUndefined = 1,
}

// 原生成宏族 (展开为方法矩阵):
// - PARAMETER_BINOP
// - PARAMETER_BINOP
// - PARAMETER_BINOP
// - HEAP_CONSTANT_ACCESSOR
// - HEAP_CONSTANT_ACCESSOR
// - HEAP_CONSTANT_TEST
// - SMI_ARITHMETIC_BINOP
// - SMI_COMPARISON_OP
// - BINT_COMPARISON_OP
// - BINT_COMPARISON_OP

/// 原 class CodeStubAssembler 骨架 (原语方法 896 处 TNode 签名)
pub struct CodeStubAssembler;

// —— 提取不变式断言 ——
const _: () = {
  assert!(AllocationFlag::Pretenured as u8 == 2);
  assert!(ExtractFixedArrayFlag::ALL_FIXED_ARRAYS.0 == 3);
};
