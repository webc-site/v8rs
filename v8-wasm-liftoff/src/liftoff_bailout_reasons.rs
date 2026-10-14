// 由 tool/src/wasm/baseline/liftoff-bailout-reasons.h.js 从 v8/src/wasm/baseline/liftoff-bailout-reasons.h 生成; 修订就地改, 复跑覆盖
// Liftoff 退出原因枚举与名称表 (直方图 V8.LiftoffBailoutReasons 对齐, 序号敏感)

/// Liftoff 放弃编译原因 (对应 enum LiftoffBailoutReason : int8_t)
/// 序号与 chromium 直方图 "V8.LiftoffBailoutReasons" 对齐, 禁重排;
/// kOtherReason=20 前留有前向兼容空档。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum LiftoffBailoutReason {
  /// kNoReason
  NoReason = 0,
  /// kDecodeError
  DecodeError = 1,
  /// kUnsupportedArchitecture
  UnsupportedArchitecture = 2,
  /// kMissingCPUFeature
  MissingCpuFeature = 3,
  /// kComplexOperation
  ComplexOperation = 4,
  /// kSimd
  Simd = 5,
  /// kRefTypes
  RefTypes = 6,
  /// kExceptionHandling
  ExceptionHandling = 7,
  /// kMultiValue
  MultiValue = 8,
  /// kTailCall
  TailCall = 9,
  /// kAtomics
  Atomics = 10,
  /// kBulkMemory
  BulkMemory = 11,
  /// kNonTrappingFloatToInt
  NonTrappingFloatToInt = 12,
  /// kGC
  Gc = 13,
  /// kRelaxedSimd
  RelaxedSimd = 14,
  /// kWasmfx
  Wasmfx = 15,
  /// kOtherReason
  OtherReason = 20,
  /// kNumBailoutReasons (哨兵)
  NumBailoutReasons = 21,
}

impl LiftoffBailoutReason {
  /// 对应 LiftoffBailoutReasonToString
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::NoReason => "no bailout",
      Self::DecodeError => "decode error",
      Self::UnsupportedArchitecture => "unsupported architecture",
      Self::MissingCpuFeature => "missing CPU feature",
      Self::ComplexOperation => "complex operation",
      Self::Simd => "simd unsupported",
      Self::RefTypes => "reftypes unsupported",
      Self::ExceptionHandling => "exception handling unsupported",
      Self::MultiValue => "multi-value unsupported",
      Self::TailCall => "tail call unsupported",
      Self::Atomics => "atomics unsupported",
      Self::BulkMemory => "bulk memory unsupported",
      Self::NonTrappingFloatToInt => "non-trapping float-to-int unsupported",
      Self::Gc => "gc unsupported",
      Self::RelaxedSimd => "relaxed simd unsupported",
      Self::Wasmfx => "wasmfx unsupported",
      Self::OtherReason => "other reason",
      Self::NumBailoutReasons => "num bailout reasons",
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  const fn str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
      return false;
    }
    let mut i = 0;
    while i < a.len() {
      if a[i] != b[i] {
        return false;
      }
      i += 1;
    }
    true
  }

  assert!(LiftoffBailoutReason::NoReason as i8 == 0);
  assert!(LiftoffBailoutReason::DecodeError as i8 == 1);
  assert!(LiftoffBailoutReason::Gc as i8 == 13);
  assert!(LiftoffBailoutReason::Wasmfx as i8 == 15);
  assert!(LiftoffBailoutReason::OtherReason as i8 == 20);
  assert!(LiftoffBailoutReason::NumBailoutReasons as i8 == 21);
  assert!(str_eq(
    LiftoffBailoutReason::NoReason.to_str(),
    "no bailout"
  ));
  assert!(str_eq(
    LiftoffBailoutReason::OtherReason.to_str(),
    "other reason"
  ));
};
