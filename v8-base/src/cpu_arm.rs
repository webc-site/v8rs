// 由 tool/src/base/cpu/cpu-arm.cc.js 从 v8/src/base/cpu/cpu-arm.cc 生成; 修订就地改, 复跑覆盖
// ARM HWCAP 特征位 (arm64 目标块; 32 位 ARM 块登记不产码)

/// arm64 HWCAP 位 (原版 getauxval(AT_HWCAP) 掩码, getauxval(AT_HWCAP) 定义于 cpu-arm.cc)
pub const HWCAP_ARM64: [(&str, u64); 37] = [
  ("HWCAP_FP", 0x1),
  ("HWCAP_ASIMD", 0x2),
  ("HWCAP_EVTSTRM", 0x4),
  ("HWCAP_AES", 0x8),
  ("HWCAP_PMULL", 0x10),
  ("HWCAP_SHA1", 0x20),
  ("HWCAP_SHA2", 0x40),
  ("HWCAP_CRC32", 0x80),
  ("HWCAP_ATOMICS", 0x100),
  ("HWCAP_FPHP", 0x200),
  ("HWCAP_ASIMDHP", 0x400),
  ("HWCAP_CPUID", 0x800),
  ("HWCAP_ASIMDRDM", 0x1000),
  ("HWCAP_JSCVT", 0x2000),
  ("HWCAP_FCMA", 0x4000),
  ("HWCAP_LRCPC", 0x8000),
  ("HWCAP_DCPOP", 0x10000),
  ("HWCAP_SHA3", 0x20000),
  ("HWCAP_SM3", 0x40000),
  ("HWCAP_SM4", 0x80000),
  ("HWCAP_ASIMDDP", 0x100000),
  ("HWCAP_SHA512", 0x200000),
  ("HWCAP_SVE", 0x400000),
  ("HWCAP_ASIMDFHM", 0x800000),
  ("HWCAP_DIT", 0x1000000),
  ("HWCAP_USCAT", 0x2000000),
  ("HWCAP_ILRCPC", 0x4000000),
  ("HWCAP_FLAGM", 0x8000000),
  ("HWCAP_SSBS", 0x10000000),
  ("HWCAP_SB", 0x20000000),
  ("HWCAP_PACA", 0x40000000),
  ("HWCAP_PACG", 0x80000000),
  ("HWCAP2_SVEBITPERM", 0x10),
  ("HWCAP2_MTE", 0x40000),
  ("HWCAP2_CSSC", 0x400000000),
  ("HWCAP2_MOPS", 0x80000000000),
  ("HWCAP2_HBC", 0x100000000000),
];

/// 32 位 ARM HWCAP/HWCAP2 块 (非目标架构, 登记不产码)
pub const ARM32_CAP_NAMES_REGISTERED: [&str; 22] = [
  "HWCAP_SWP",
  "HWCAP_HALF",
  "HWCAP_THUMB",
  "HWCAP_26BIT",
  "HWCAP_FAST_MULT",
  "HWCAP_FPA",
  "HWCAP_VFP",
  "HWCAP_EDSP",
  "HWCAP_JAVA",
  "HWCAP_IWMMXT",
  "HWCAP_CRUNCH",
  "HWCAP_THUMBEE",
  "HWCAP_NEON",
  "HWCAP_VFPv3",
  "HWCAP_VFPv3D16",
  "HWCAP_TLS",
  "HWCAP_VFPv4",
  "HWCAP_IDIVA",
  "HWCAP_IDIVT",
  "HWCAP_VFPD32",
  "HWCAP_IDIV",
  "HWCAP_LPAE",
];

/// 按名查 arm64 位值
#[inline]
#[must_use]
pub fn hwcap(name: &str) -> Option<u64> {
  HWCAP_ARM64.iter().find(|(n, _)| *n == name).map(|(_, v)| *v)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(HWCAP_ARM64.len() == 37);
  assert!(HWCAP_ARM64[0].0 == "HWCAP_FP" && HWCAP_ARM64[0].1 == 0x1);
  assert!(hwcap("HWCAP_ASIMD") == Some(0x2));
};
