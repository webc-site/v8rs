// 由 tool/src/objects/deoptimization-data.h.js 从 v8/src/objects/deoptimization-data.{h,cc} 生成; 修订就地改, 复跑覆盖
// 去优化字面量类别枚举
//! DeoptimizationData: 去优化字面量类别。
//! 翻译表编解码语义面 (TranslationOpcode/翻译数组) 登记跳过, 由 Developer 就地实现。

/// 去优化字面量类别 (原版 enum class DeoptimizationLiteralKind : uint8_t, 共 10 条目)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeoptimizationLiteralKind {
  /// kObject
  Object = 0,
  /// kNumber
  Number = 1,
  /// kSignedBigInt64
  SignedBigInt64 = 2,
  /// kUnsignedBigInt64
  UnsignedBigInt64 = 3,
  /// kInvalid
  Invalid = 4,
  /// kWasmI31Ref
  WasmI31Ref = 5,
  /// kWasmInt32
  WasmInt32 = 6,
  /// kWasmFloat32
  WasmFloat32 = 7,
  /// kWasmFloat64
  WasmFloat64 = 8,
}

impl DeoptimizationLiteralKind {
  /// kWasmInt64 (别名 = kSignedBigInt64)
  pub const WASM_INT64: Self = Self::SignedBigInt64;

  /// 非别名声目数 (别名 kWasmInt64 复用 kSignedBigInt64 值)。
  pub const COUNT: usize = 9;

  /// 数值反查 (别名值命中共有条目)。
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::Object),
      1 => Some(Self::Number),
      2 => Some(Self::SignedBigInt64),
      3 => Some(Self::UnsignedBigInt64),
      4 => Some(Self::Invalid),
      5 => Some(Self::WasmI31Ref),
      6 => Some(Self::WasmInt32),
      7 => Some(Self::WasmFloat32),
      8 => Some(Self::WasmFloat64),
      _ => None,
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(DeoptimizationLiteralKind::Object as u8 == 0);
  assert!(DeoptimizationLiteralKind::Invalid as u8 == 4);
  assert!(
    DeoptimizationLiteralKind::WASM_INT64 as u8 == DeoptimizationLiteralKind::SignedBigInt64 as u8
  );
};
