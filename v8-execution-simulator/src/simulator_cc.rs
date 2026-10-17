// 由 tool/src/execution/simulator-cc.js 从 v8/src/execution/simulator-base.cc, src/execution/arm64/simulator-logic-arm64.cc 生成; 修订就地改, 复跑覆盖
// SimulatorBase 生命周期 + simulator-logic 指令语义清单 (规则面基线, 函数体 todo)

//! SimulatorBase 与 arm64 指令语义模拟器的规则面。
//!
//! simulator-base.cc: 进程级单例协议 (重定向链/指令缓存, 互斥锁四件套);
//! simulator-logic-arm64.cc: Visit 分发 + FP half 桥接, 函数体归语义轮填体。

// ══ SimulatorBase (simulator-base.cc) ══

use std::sync::atomic::{AtomicBool, Ordering};

/// 静态成员四件套: redirection_mutex / redirection / i_cache_mutex / i_cache
/// (cc:14-25; Rust 版收敛为 OnceLock 进程单例, 字段保留语义名)
pub const REDIRECTION_MUTEX: &str = "redirection_mutex_";
pub const REDIRECTION: &str = "redirection_";
pub const I_CACHE_MUTEX: &str = "i_cache_mutex_";
pub const I_CACHE: &str = "i_cache_";

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// InitializeOncePerProcess (cc:27-38): 首次调用建锁 + CustomMatcherHashMap(ICacheMatch)
pub fn initialize_once_per_process() {
  INITIALIZED.store(true, Ordering::SeqCst);
}

/// GlobalTearDown (cc:40-58): 删锁/删重定向链/逐 CachePage 释放 i_cache
pub fn global_tear_down() {
  INITIALIZED.store(false, Ordering::SeqCst);
}

// ══ half 桥接 (simulator-logic-arm64.cc:16-49) ══

/// FP16 位模式: 默认 NaN 与 quiet-NaN 掩码 (kFP16DefaultNaN=0x7E00, kHQuietNanMask=1<<9)
pub const K_FP16_DEFAULT_NAN: u16 = 0x7E00;
pub const K_H_QUIET_NAN_MASK: u16 = 0x200;

/// half(float16) 桥接: fp16_ieee_from_fp32_value / fp16_ieee_to_fp32_value (cc:20-21/24)
pub struct Half {
  pub bits: u16,
}

impl Half {
  /// half(float) — fp32 → fp16 位模式 (fp16_ieee_from_fp32_value)
  pub fn from_f32(v: f32) -> Self {
    let w = v.to_bits();
    let shl1_w = w.wrapping_add(w);
    let sign = w & 0x8000_0000;
    // 溢出/下溢基准 abs((v*2^112)*2^-110); 指数偏置钳制到 [0x71000000,..]
    let mut base = (v * f32::from_bits(0x7780_0000) * f32::from_bits(0x0880_0000)).abs();
    let mut bias = shl1_w & 0xFF00_0000;
    if bias < 0x7100_0000 {
      bias = 0x7100_0000;
    }
    base += f32::from_bits((bias >> 1) + 0x0780_0000);
    let bits = base.to_bits();
    let exp_bits = (bits >> 13) & 0x0000_7C00;
    let mantissa_bits = bits & 0x0000_0FFF;
    // 原值为 NaN (shl1_w>0xFF000000) → 落 quiet-NaN, 否则拼指数尾数
    let result = if shl1_w > 0xFF00_0000 {
      0x7E00
    } else {
      exp_bits + mantissa_bits
    };
    Self {
      bits: ((sign >> 16) | result) as u16,
    }
  }
  /// operator float() — fp16 位模式 → fp32 (fp16_ieee_to_fp32_value)
  pub fn to_f32(self) -> f32 {
    let w = (self.bits as u32) << 16;
    let sign = w & 0x8000_0000;
    let two_w = w.wrapping_add(w);
    // 规格化: 抬指数到 fp32 位段再乘 2^-112
    let normalized = f32::from_bits((two_w >> 4) + 0x7000_0000) * f32::from_bits(0x0780_0000);
    // 非规格化/零: 尾数并入偏置 126 的 fp32, 再减 magic_bias 0.5
    let denormalized = f32::from_bits((two_w >> 17) | (126u32 << 23)) - 0.5f32;
    let base_bits = if two_w < 0x0800_0000 {
      denormalized.to_bits()
    } else {
      normalized.to_bits()
    };
    f32::from_bits(sign | base_bits)
  }
  /// operator-() — 符号位翻转 (cc:27 bits ^ 0x8000)
  pub fn neg(self) -> Self {
    Self {
      bits: self.bits ^ 0x8000,
    }
  }
  /// ToQuietNaN (cc:35-37 bits | kHQuietNanMask)
  pub fn to_quiet_nan(self) -> Self {
    Self {
      bits: self.bits | K_H_QUIET_NAN_MASK,
    }
  }
}

// ══ 指令语义助手面 (simulator-logic-arm64.cc, 共 313 个 Simulator:: 方法) ══

/// 语义方法名全表 (函数体归填体轮, 表本身供覆盖率与分派骨架对数)
pub const LOGIC_METHODS: &[&str] = &[
  "FixedToDouble",
  "UFixedToDouble",
  "FixedToFloat",
  "UFixedToFloat",
  "FixedToFloat16",
  "UFixedToFloat16",
  "FPToDouble",
  "FPToFloat",
  "FPToFloat16",
  "FPToFloat16",
  "FPToFloat",
  "ld1",
  "ld1",
  "ld1r",
  "ld2",
  "ld2",
  "ld2r",
  "ld3",
  "ld3",
  "ld3r",
  "ld4",
  "ld4",
  "ld4r",
  "st1",
  "st1",
  "st2",
  "st2",
  "st3",
  "st3",
  "st4",
  "st4",
  "cmp",
  "cmp",
  "cmptst",
  "add",
  "addp",
  "mla",
  "mls",
  "mul",
  "mul",
  "mla",
  "mls",
  "smull",
  "smull2",
  "umull",
  "umull2",
  "smlal",
  "smlal2",
  "umlal",
  "umlal2",
  "smlsl",
  "smlsl2",
  "umlsl",
  "umlsl2",
  "sqdmull",
  "sqdmull2",
  "sqdmlal",
  "sqdmlal2",
  "sqdmlsl",
  "sqdmlsl2",
  "sqdmulh",
  "sqrdmulh",
  "PolynomialMult",
  "pmul",
  "pmull",
  "pmull2",
  "sub",
  "and_",
  "orr",
  "orn",
  "eor",
  "bic",
  "bic",
  "bif",
  "bit",
  "bsl",
  "SMinMax",
  "smax",
  "smin",
  "SMinMaxP",
  "smaxp",
  "sminp",
  "addp",
  "addv",
  "saddlv",
  "uaddlv",
  "SMinMaxV",
  "smaxv",
  "sminv",
  "UMinMax",
  "umax",
  "umin",
  "UMinMaxP",
  "umaxp",
  "uminp",
  "UMinMaxV",
  "umaxv",
  "uminv",
  "shl",
  "sshll",
  "sshll2",
  "shll",
  "shll2",
  "ushll",
  "ushll2",
  "sli",
  "sqshl",
  "uqshl",
  "sqshlu",
  "sri",
  "ushr",
  "sshr",
  "ssra",
  "usra",
  "srsra",
  "ursra",
  "cls",
  "clz",
  "cnt",
  "sshl",
  "ushl",
  "neg",
  "suqadd",
  "usqadd",
  "abs",
  "ExtractNarrow",
  "xtn",
  "sqxtn",
  "sqxtun",
  "uqxtn",
  "AbsDiff",
  "saba",
  "uaba",
  "not_",
  "rbit",
  "rev",
  "rev16",
  "rev32",
  "rev64",
  "addlp",
  "saddlp",
  "uaddlp",
  "sadalp",
  "uadalp",
  "ror",
  "ext",
  "dup_element",
  "dup_immediate",
  "ins_element",
  "ins_immediate",
  "movi",
  "mvni",
  "orr",
  "uxtl",
  "sxtl",
  "uxtl2",
  "sxtl2",
  "shrn",
  "shrn2",
  "rshrn",
  "rshrn2",
  "Table",
  "tbl",
  "tbl",
  "tbl",
  "tbl",
  "tbx",
  "tbx",
  "tbx",
  "tbx",
  "uqshrn",
  "uqshrn2",
  "uqrshrn",
  "uqrshrn2",
  "sqshrn",
  "sqshrn2",
  "sqrshrn",
  "sqrshrn2",
  "sqshrun",
  "sqshrun2",
  "sqrshrun",
  "sqrshrun2",
  "uaddl",
  "uaddl2",
  "uaddw",
  "uaddw2",
  "saddl",
  "saddl2",
  "saddw",
  "saddw2",
  "usubl",
  "usubl2",
  "usubw",
  "usubw2",
  "ssubl",
  "ssubl2",
  "ssubw",
  "ssubw2",
  "uabal",
  "uabal2",
  "sabal",
  "sabal2",
  "uabdl",
  "uabdl2",
  "sabdl",
  "sabdl2",
  "umull",
  "umull2",
  "smull",
  "smull2",
  "umlsl",
  "umlsl2",
  "smlsl",
  "smlsl2",
  "umlal",
  "umlal2",
  "smlal",
  "smlal2",
  "sqdmlal",
  "sqdmlal2",
  "sqdmlsl",
  "sqdmlsl2",
  "sqdmull",
  "sqdmull2",
  "sqrdmulh",
  "sqdmulh",
  "addhn",
  "addhn2",
  "raddhn",
  "raddhn2",
  "subhn",
  "subhn2",
  "rsubhn",
  "rsubhn2",
  "trn1",
  "trn2",
  "zip1",
  "zip2",
  "uzp1",
  "uzp2",
  "FPAdd",
  "FPSub",
  "FPMul",
  "FPMulx",
  "FPMulAdd",
  "FPDiv",
  "FPSqrt",
  "FPMax",
  "FPMaxNM",
  "FPMin",
  "FPMinNM",
  "FPRecipStepFused",
  "FPRSqrtStepFused",
  "FPRoundInt",
  "FPToInt16",
  "FPToInt32",
  "FPToInt64",
  "FPToUInt16",
  "FPToUInt32",
  "FPToUInt64",
  "fnmul",
  "frecps",
  "frecps",
  "frsqrts",
  "FPToFixedJS",
  "frsqrts",
  "fcmp",
  "fcmp",
  "fcmp_zero",
  "fabscmp",
  "fmla",
  "fmla",
  "fmls",
  "fmls",
  "fneg",
  "fneg",
  "fabs_",
  "fabs_",
  "fabd",
  "fsqrt",
  "FMinMaxV",
  "fmaxv",
  "fminv",
  "fmaxnmv",
  "fminnmv",
  "fmul",
  "fmla",
  "fmls",
  "fmulx",
  "frint",
  "fcvts",
  "fcvtu",
  "fcvtl",
  "fcvtl2",
  "fcvtn",
  "fcvtn2",
  "fcvtxn",
  "fcvtxn2",
  "recip_sqrt_estimate",
  "FPRecipSqrtEstimate",
  "frsqrte",
  "FPRecipEstimate",
  "frecpe",
  "ursqrte",
  "recip_estimate",
  "urecpe",
  "frecpx",
  "frecpx",
  "scvtf",
  "ucvtf",
  "dot",
  "sdot",
  "bgrp",
];
