// 由 tool/src/bigint/bigint-internal.h.js 从 v8/src/bigint/bigint-internal.{h,cc} 生成; 修订就地改, 复跑覆盖
// BigInt 处理器内部协议

/// BigInt 处理器内部常量 (原版 bigint-internal.h)
pub mod bigint_internal_const {
  /// 工作量阈值: 累计约一次 mul 指令计数的上限, 达到即查中断
  /// (目标粒度 10-100ms; 原版 kWorkEstimateThreshold)
  pub const WORK_ESTIMATE_THRESHOLD: usize = 5000000;

  /// 位数上限 = UINT32_MAX / kDigitBits, 64 位目标 kDigitBits=64 (原版 kMaxNumDigits)
  pub const MAX_NUM_DIGITS: u32 = 67108863;

  /// kDigitBits (64 位目标)
  pub const DIGIT_BITS: u32 = 64;

  /// FromString 分派族 (对数: 4)
  pub const FROM_STRING_FAMILY: [&str; 4] = ["FromString", "FromStringClassic", "FromStringLarge", "FromStringBasePowerOfTwo"];
}

/// 处理器中断状态 (原版 Status)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
  Ok = 0,
  Interrupted = 1,
}

/// ProcessorImpl 公开算子面 (原版 bigint-internal.h 头侧声明, 25 项, 顺序登记)
pub const PROCESSOR_DECL_METHODS: [&str; 25] = ["Multiply", "MultiplyLarge", "MultiplyKaratsuba", "KaratsubaStart", "KaratsubaChunk", "KaratsubaMain", "DivideSchoolbook", "DivideBurnikelZiegler", "CachedMod_MakeInverse", "MultiplyToomCook", "Toom3Main", "MultiplyFFT", "DivideBarrett", "DivideBarrett", "Invert", "InvertBasecase", "InvertNewton", "ToString", "ToStringImpl", "FromString", "FromStringClassic", "FromStringLarge", "FromStringBasePowerOfTwo", "should_terminate", "AddWorkEstimate"];

/// 中断协议签名骨架 (语义见 crate 内手写 bigint_internal.rs)
pub mod bigint_internal_skel {
  /// 工作量累计: 达 WORK_ESTIMATE_THRESHOLD 清零并查平台中断 (原版 AddWorkEstimate)
  pub fn add_work_estimate(_estimate: usize) -> bool {
    unimplemented!("骨架")
  }

  /// 字符串转 BigInt 总分派 (原版 FromString; 按 radix/长度选 Classic/Large/PowerOfTwo)
  pub fn from_string(_z: &mut [u64], _accumulator: &mut usize) {
    unimplemented!("骨架")
  }
}
