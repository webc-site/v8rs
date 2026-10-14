// 由 tool/src/bigint/mul-karatsuba.cc.js 从 v8/src/bigint/mul-{karatsuba,toom,fft}.cc (+bigint-inl.h config) 生成; 修订就地改, 复跑覆盖
// 乘法三族阈值与定义头清单

/// 乘法算法分派阈值 (原版 bigint-inl.h namespace config, 按平台展开; 64 位为当前目标)
pub mod mul_config {
  /// schoolbook → karatsuba 界 (平台无关单一值)
  pub const KARATSUBA_THRESHOLD: u32 = 34;
  /// karatsuba → toom 界 (x64 上 200-400 位区间两者接近)
  pub const TOOM_THRESHOLD: u32 = 210;
  /// toom → fft 界 (32 位 1100 / 64 位 720)
  pub const FFT_THRESHOLD_32: u32 = 1100;
  pub const FFT_THRESHOLD_64: u32 = 720;
  /// fft 内层界 (内外两层 fft)
  pub const FFT_INNER_THRESHOLD: u32 = 200;

  /// 当前目标 (64 位) 生效组
  pub const ACTIVE: [&str; 4] = ["karatsuba=34", "toom=210", "fft=720", "fft_inner=200"];
}

/// FFT 非对称分块阈值 (原版 mul-fft.cc kAsymmetricChunkingThreshold)
pub const FFT_ASYMMETRIC_CHUNKING_THRESHOLD: u32 = 100;

// mul-karatsuba.cc 定义头清单 (7 项):
// uint32_t RoundUpLen(uint32_t len)
// uint32_t KaratsubaLength(uint32_t n)
// void KaratsubaSubtractionHelper(RWDigits result, Digits X, Digits Y, int* sign)
// void ProcessorImpl::MultiplyKaratsuba(RWDigits Z, Digits X, Digits Y)
// void ProcessorImpl::KaratsubaStart(RWDigits Z, Digits X, Digits Y, RWDigits scratch, uint32_t k)
// void ProcessorImpl::KaratsubaChunk(RWDigits Z, Digits X, Digits Y, RWDigits scratch)
// void ProcessorImpl::KaratsubaMain(RWDigits Z, Digits X, Digits Y, RWDigits scratch, uint32_t n)

// mul-toom.cc 定义头清单 (5 项):
// void TimesTwo(RWDigits X)
// void DivideByTwo(RWDigits X)
// void DivideByThree(RWDigits X)
// void ProcessorImpl::Toom3Main(RWDigits Z, Digits X, Digits Y)
// void ProcessorImpl::MultiplyToomCook(RWDigits Z, Digits X, Digits Y)

// mul-fft.cc 定义头清单 (24 项):
// void ModFn_Helper(digit_t* x, uint32_t len, signed_digit_t high)
// void ModFn(digit_t* x, uint32_t len)
// void ModFnDoubleWidth(digit_t* dest, const digit_t* src, uint32_t len)
// void SumDiff(digit_t* sum, digit_t* diff, const digit_t* a, const digit_t* b, uint32_t len)
// void ShiftModFn_Large(digit_t* result, const digit_t* input, uint32_t digit_shift, int bits_shift, uint32_t K)
// void ShiftModFn(digit_t* result, const digit_t* input, int power_of_two, uint32_t K, uint32_t zero_above = 0x7FFFFFFF)
// void ComputeParameters(uint32_t N, int m, Parameters* params)
// void ComputeParameters_Inner(uint32_t N, Parameters* params)
// uint32_t PredictInnerK(uint32_t N)
// bool ShouldDecrementM(const Parameters& current, const Parameters& next, const Parameters& after_next)
// int GetParameters(uint32_t N, Parameters* params)
// void FFTContainer::Start_Default(Digits X, uint32_t chunk_size, int theta, int omega)
// void FFTContainer::Start(Digits X, uint32_t chunk_size, int theta, int omega)
// void FFTContainer::FFT_ReturnShuffledThreadsafe(uint32_t start, uint32_t len, int omega, digit_t* temp)
// void FFTContainer::FFT_Recurse(uint32_t start, uint32_t half, int omega, digit_t* temp)
// void FFTContainer::BackwardFFT(uint32_t start, uint32_t len, int omega)
// void FFTContainer::BackwardFFT_Threadsafe(uint32_t start, uint32_t len, int omega, digit_t* temp)
// void FFTContainer::NormalizeAndRecombine(int omega, int m, RWDigits Z, uint32_t chunk_size)
// bool ShouldBeNegative(const digit_t* x, uint32_t xlen, digit_t threshold, int s)
// void FFTContainer::CounterWeightAndRecombine(int theta, int m, RWDigits Z, uint32_t s)
// void MultiplyFFT_Inner(RWDigits Z, Digits X, Digits Y, const Parameters& params, ProcessorImpl* processor)
// void FFTContainer::DoPointwiseMultiplication(const FFTContainer& other, uint32_t start, uint32_t end, digit_t* temp)
// void FFTContainer::PointwiseMultiply(const FFTContainer& other)
// void ProcessorImpl::MultiplyFFT(RWDigits Z, Digits X, Digits Y)
