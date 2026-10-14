// 由 tool/src/codegen/cpu-features.h.js 从 v8/src/codegen/cpu-features.h;src/codegen/flush-instruction-cache.{h,cc} 生成; 修订就地改, 复跑覆盖
// CpuFeature 枚举 (x64/arm64 展开) + CpuFeatures 静态面骨架 + FlushInstructionCache 签名

// 平台分支: x64 与 arm64 两目标分别展开为独立枚举;
// arm/ia32/mips64/loong64/ppc/s390/riscv 分支未采用, 在此登记跳过。

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// x64/ia32 目标 CpuFeature (原版 #if V8_TARGET_ARCH_IA32 || X64 分支, 20 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CpuFeatureX64 {
  /// SSE4_2
  Sse4_2 = 0,
  /// SSE4_1
  Sse4_1 = 1,
  /// SSSE3
  Ssse3 = 2,
  /// SSE3
  Sse3 = 3,
  /// SAHF
  Sahf = 4,
  /// AVX
  Avx = 5,
  /// AVX2
  Avx2 = 6,
  /// AVX_VNNI
  AvxVnni = 7,
  /// AVX_VNNI_INT8
  AvxVnniInt8 = 8,
  /// FMA3
  Fma3 = 9,
  /// BMI1
  Bmi1 = 10,
  /// BMI2
  Bmi2 = 11,
  /// LZCNT
  Lzcnt = 12,
  /// POPCNT
  Popcnt = 13,
  /// INTEL_ATOM
  IntelAtom = 14,
  /// INTEL_JCC_ERRATUM_MITIGATION
  IntelJccErratumMitigation = 15,
  /// CETSS
  Cetss = 16,
  /// F16C
  F16C = 17,
  /// APX_F
  ApxF = 18,
  /// AVX10_1
  Avx10_1 = 19,
  /// NUMBER_OF_CPU_FEATURES 哨兵
  NumberOfCpuFeatures = 20,
}

impl CpuFeatureX64 {
  pub const SSE4_2: Self = Self::Sse4_2;
  pub const SSE4_1: Self = Self::Sse4_1;
  pub const SSSE3: Self = Self::Ssse3;
  pub const SSE3: Self = Self::Sse3;
  pub const SAHF: Self = Self::Sahf;
  pub const AVX: Self = Self::Avx;
  pub const AVX2: Self = Self::Avx2;
  pub const AVX_VNNI: Self = Self::AvxVnni;
  pub const AVX_VNNI_INT8: Self = Self::AvxVnniInt8;
  pub const FMA3: Self = Self::Fma3;
  pub const BMI1: Self = Self::Bmi1;
  pub const BMI2: Self = Self::Bmi2;
  pub const LZCNT: Self = Self::Lzcnt;
  pub const POPCNT: Self = Self::Popcnt;
  pub const INTEL_ATOM: Self = Self::IntelAtom;
  pub const INTEL_JCC_ERRATUM_MITIGATION: Self = Self::IntelJccErratumMitigation;
  pub const CETSS: Self = Self::Cetss;
  pub const F16_C: Self = Self::F16C;
  pub const F16C: Self = Self::F16C;
  pub const APX_F: Self = Self::ApxF;
  pub const AVX10_1: Self = Self::Avx10_1;
  pub const NUMBER_OF_CPU_FEATURES: Self = Self::NumberOfCpuFeatures;
}

/// arm64 目标 CpuFeature (原版 #elif V8_TARGET_ARCH_ARM64 分支, 11 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CpuFeatureArm64 {
  /// JSCVT
  Jscvt = 0,
  /// DOTPROD
  Dotprod = 1,
  /// LSE
  Lse = 2,
  /// PMULL1Q
  Pmull1Q = 3,
  /// FP16
  Fp16 = 4,
  /// SHA3
  Sha3 = 5,
  /// HBC
  Hbc = 6,
  /// CSSC
  Cssc = 7,
  /// MOPS
  Mops = 8,
  /// SVE
  Sve = 9,
  /// SVEBITPERM
  Svebitperm = 10,
  /// NUMBER_OF_CPU_FEATURES 哨兵
  NumberOfCpuFeatures = 11,
}

impl CpuFeatureArm64 {
  pub const JSCVT: Self = Self::Jscvt;
  pub const DOTPROD: Self = Self::Dotprod;
  pub const LSE: Self = Self::Lse;
  pub const PMULL1_Q: Self = Self::Pmull1Q;
  pub const PMULL1Q: Self = Self::Pmull1Q;
  pub const FP16: Self = Self::Fp16;
  pub const SHA3: Self = Self::Sha3;
  pub const HBC: Self = Self::Hbc;
  pub const CSSC: Self = Self::Cssc;
  pub const MOPS: Self = Self::Mops;
  pub const SVE: Self = Self::Sve;
  pub const SVEBITPERM: Self = Self::Svebitperm;
  pub const NUMBER_OF_CPU_FEATURES: Self = Self::NumberOfCpuFeatures;
}

// 原版 static_assert(NUMBER_OF_CPU_FEATURES <= kBitsPerInt) 的 u32 载体前提

/// 原版 CpuFeatures::supported_ (base::EnumSet<CpuFeatureSet>) 的位掩码载体:
/// 两目标架构哨兵 20/11 均 < 32, u32 满足原版 static_assert(<= kBitsPerInt)。
static SUPPORTED: AtomicU32 = AtomicU32::new(0);
/// 原版 CpuFeatures::initialized_
static INITIALIZED: AtomicBool = AtomicBool::new(false);
/// 原版 icache_line_size_ (ProbeImpl 平台侧写入)
static ICACHE_LINE_SIZE: AtomicU32 = AtomicU32::new(0);
/// 原版 dcache_line_size_ (ProbeImpl 平台侧写入)
static DCACHE_LINE_SIZE: AtomicU32 = AtomicU32::new(0);

/// 探测最大特征码上限 (u32 容量 32 项)
pub const K_MAX_PROBED_CPU_FEATURES: u8 = 32;

/// CpuFeatures 静态面 (AllStatic; 支持集必须先 Probe 再用):
/// 例: if is_supported(SSE3 as u8) { CpuFeatureScope ... }
pub struct CpuFeatures;

impl CpuFeatures {
  /// 是否已 Probe 初始化 (原版 Probe 内 initialized_ 幂等门, 供上层查询)
  /// v8: src/codegen/cpu-features.h:137-142
  #[must_use]
  pub fn initialized() -> bool {
    INITIALIZED.load(Ordering::Relaxed)
  }

  /// 原版 Probe(cross_compile): 触发各架构特化 ProbeImpl
  /// v8: src/codegen/cpu-features.h:133
  pub fn probe(cross_compile: bool) {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
      return;
    }
    ICACHE_LINE_SIZE.store(64, Ordering::Relaxed);
    DCACHE_LINE_SIZE.store(64, Ordering::Relaxed);

    if cross_compile {
      return;
    }

    #[cfg(target_arch = "x86_64")]
    {
      let mut mask = 0u32;
      mask |= 1u32 << (CpuFeatureX64::SSE4_1 as u8);
      mask |= 1u32 << (CpuFeatureX64::SSE4_2 as u8);
      mask |= 1u32 << (CpuFeatureX64::POPCNT as u8);
      mask |= 1u32 << (CpuFeatureX64::SAHF as u8);
      if std::is_x86_feature_detected!("avx") {
        mask |= 1u32 << (CpuFeatureX64::AVX as u8);
      }
      if std::is_x86_feature_detected!("avx2") {
        mask |= 1u32 << (CpuFeatureX64::AVX2 as u8);
      }
      if std::is_x86_feature_detected!("fma") {
        mask |= 1u32 << (CpuFeatureX64::FMA3 as u8);
      }
      if std::is_x86_feature_detected!("bmi1") {
        mask |= 1u32 << (CpuFeatureX64::BMI1 as u8);
      }
      if std::is_x86_feature_detected!("bmi2") {
        mask |= 1u32 << (CpuFeatureX64::BMI2 as u8);
      }
      if std::is_x86_feature_detected!("lzcnt") {
        mask |= 1u32 << (CpuFeatureX64::LZCNT as u8);
      }
      SUPPORTED.fetch_or(mask, Ordering::Relaxed);
    }

    #[cfg(target_arch = "aarch64")]
    {
      let mut mask = 0u32;
      mask |= 1u32 << (CpuFeatureArm64::Fp16 as u8);
      mask |= 1u32 << (CpuFeatureArm64::Dotprod as u8);
      mask |= 1u32 << (CpuFeatureArm64::Lse as u8);
      SUPPORTED.fetch_or(mask, Ordering::Relaxed);
    }
  }

  /// 原版 IsSupported(f): 查位掩码
  /// v8: src/codegen/cpu-features.h:135
  #[must_use]
  pub fn is_supported(f: u8) -> bool {
    debug_assert!(
      f < K_MAX_PROBED_CPU_FEATURES,
      "特征码越出 EnumSet u32 载体容量"
    );
    (SUPPORTED.load(Ordering::Relaxed) & (1u32 << (f & 31))) != 0
  }

  /// 原版 SupportedFeatures(): 交出支持集
  /// v8: src/codegen/cpu-features.h:145
  #[must_use]
  pub fn supported_features() -> u32 {
    SUPPORTED.load(Ordering::Relaxed)
  }

  /// 原版 Supported(f): 支持则交出位掩码, 否则回退为 0 (Probe 阶段自举用)
  /// v8: src/codegen/cpu-features.h:148
  #[must_use]
  pub fn supported(f: u8) -> u32 {
    let mask = 1u32 << (f & 31);
    if (SUPPORTED.load(Ordering::Relaxed) & mask) != 0 {
      mask
    } else {
      0
    }
  }

  /// 原版 SetSupported(f): supported_.Add(f)
  /// v8: src/codegen/cpu-features.h:150
  pub fn set_supported(f: u8) {
    debug_assert!(
      f < K_MAX_PROBED_CPU_FEATURES,
      "特征码越出 EnumSet u32 载体容量"
    );
    SUPPORTED.fetch_or(1u32 << (f & 31), Ordering::Relaxed);
  }

  /// 原版 SetUnsupported(f): supported_.Remove(f)
  /// v8: src/codegen/cpu-features.h:153
  pub fn set_unsupported(f: u8) {
    debug_assert!(
      f < K_MAX_PROBED_CPU_FEATURES,
      "特征码越出 EnumSet u32 载体容量"
    );
    SUPPORTED.fetch_and(!(1u32 << (f & 31)), Ordering::Relaxed);
  }

  /// 原版 SupportsSimd128(): 各架构 cpu-features-*.cc/h 特化实现
  /// v8: src/codegen/cpu-features.h:156
  #[must_use]
  pub fn supports_simd128() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
      Self::is_supported(CpuFeatureX64::SSE4_1 as u8)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
      true
    }
  }

  /// 原版 SupportsOptimizer(): 各架构特化谓词
  /// v8: src/codegen/cpu-features.h:158
  #[must_use]
  pub fn supports_optimizer() -> bool {
    true
  }

  /// 原版 icache_line_size(): DCHECK_NE(icache_line_size_, 0) 后交出静态值
  /// v8: src/codegen/cpu-features.h:160-163
  #[must_use]
  pub fn icache_line_size() -> u32 {
    let size = ICACHE_LINE_SIZE.load(Ordering::Relaxed);
    debug_assert!(
      size != 0,
      "原版 DCHECK_NE(icache_line_size_, 0): Probe 前不可用"
    );
    if size != 0 { size } else { 64 }
  }

  /// 原版 dcache_line_size(): DCHECK_NE(dcache_line_size_, 0) 后交出静态值
  /// v8: src/codegen/cpu-features.h:165-168
  #[must_use]
  pub fn dcache_line_size() -> u32 {
    let size = DCACHE_LINE_SIZE.load(Ordering::Relaxed);
    debug_assert!(
      size != 0,
      "原版 DCHECK_NE(dcache_line_size_, 0): Probe 前不可用"
    );
    if size != 0 { size } else { 64 }
  }

  /// 原版 FlushICache(start, size): 平台实现 (arm sys_cacheflush 等)
  /// v8: src/codegen/cpu-features.h:181
  pub fn flush_i_cache(start: *mut u8, size: usize) {
    crate::flush_instruction_cache(start, size);
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CpuFeatureX64::NUMBER_OF_CPU_FEATURES as u8 == 20);
  assert!(CpuFeatureArm64::NUMBER_OF_CPU_FEATURES as u8 == 11);
  assert!(CpuFeatureX64::SSE4_2 as u8 == 0);
  assert!(CpuFeatureArm64::JSCVT as u8 == 0);
};
