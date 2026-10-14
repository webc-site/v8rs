//! 原版 v8/src/base/cpu/{cpu.h,cpu.cc,cpu-x86.cc,cpu-arm.cc} — CPU 特性检测
//!
//! 结构与原版 CPU 类 1:1: vendor_(13 字节 NUL 结尾) + 整型身份字段 + 全部
//! has_* 布尔位 + 惰性单例 GetInstance (OnceLock)。
//! 探测路径按宿主架构机械展开 (todo 一章: 先 x64 + arm64, 其余架构后置):
//! - x86_64: CPUID (leaf 0/1/7/7.1/0x24/0x80000000~/0x40000000)
//! - aarch64 macOS: 固定特性集 (原版 "ARM64 Macs always have ...") +
//!   sysctl hw.optional.arm.FEAT_CSSC
//! - aarch64 linux: getauxval(AT_HWCAP/AT_HWCAP2) 位判定
//!   (原版 hwcaps==0 时的 /proc/cpuinfo 回退路径登记不迁移, 现代 内核恒走 hwcap)
//! - 其余架构: 不探测 (保持 "Unknown" + 空特性, 原版对应分支为编译期裁剪)
//!
//! 原版 arm64 分支不设置 has_fpu_/has_neon_ 与缓存行宽 (V8 编译期假定),
//! 此处保持同款默认值 (bug-compat)。

use core::str::from_utf8;
use std::sync::OnceLock;

/// riscv MMU 模式 (原版 enum class RV_MMU_MODE)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RvMmuMode {
  RiscvSv39,
  RiscvSv48,
  RiscvSv57,
}

/// arm implementer 常量 (cpu.h:50-52)
pub const K_ARM: i32 = 0x41;
/// Nvidia
pub const K_NVIDIA: i32 = 0x4E;
/// Qualcomm
pub const K_QUALCOMM: i32 = 0x51;
/// Nvidia Denver 变体 (cpu.h:55)
pub const K_NVIDIA_DENVER: i32 = 0x0;

/// ARM part 码 (cpu.h:59-66)
pub const K_ARM_CORTEX_A5: i32 = 0xC05;
/// Cortex-A7
pub const K_ARM_CORTEX_A7: i32 = 0xC07;
/// Cortex-A8
pub const K_ARM_CORTEX_A8: i32 = 0xC08;
/// Cortex-A9
pub const K_ARM_CORTEX_A9: i32 = 0xC09;
/// Cortex-A12
pub const K_ARM_CORTEX_A12: i32 = 0xC0C;
/// Cortex-A15
pub const K_ARM_CORTEX_A15: i32 = 0xC0F;
/// Denver V10
pub const K_NVIDIA_DENVER_V10: i32 = 0x002;

/// 未知缓存行宽 (cpu.h:76)
pub const K_UNKNOWN_CACHE_LINE_SIZE: i32 = 0;
/// 未知虚拟地址位数 (cpu.h:115)
pub const K_UNKNOWN_NUM_VIRTUAL_ADDRESS_BITS: i32 = 0;
/// 未知 RVV 向量长度 (cpu.h:157)
pub const K_UNKNOWN_VLEN: u32 = 0;

/// CPU — 原版 class CPU final 的字段级对照
#[derive(Debug, Clone)]
pub struct Cpu {
  pub(crate) vendor_: [u8; 13],
  pub(crate) stepping_: i32,
  pub(crate) model_: i32,
  pub(crate) ext_model_: i32,
  pub(crate) family_: i32,
  pub(crate) ext_family_: i32,
  pub(crate) type_: i32,
  pub(crate) implementer_: i32,
  pub(crate) architecture_: i32,
  pub(crate) variant_: i32,
  pub(crate) part_: i32,
  pub(crate) icache_line_size_: i32,
  pub(crate) dcache_line_size_: i32,
  pub(crate) num_virtual_address_bits_: i32,
  pub(crate) has_fpu_: bool,
  pub(crate) has_cmov_: bool,
  pub(crate) has_sahf_: bool,
  pub(crate) has_mmx_: bool,
  pub(crate) has_sse_: bool,
  pub(crate) has_sse2_: bool,
  pub(crate) has_sse3_: bool,
  pub(crate) has_ssse3_: bool,
  pub(crate) has_sse41_: bool,
  pub(crate) has_sse42_: bool,
  pub(crate) is_atom_: bool,
  pub(crate) has_intel_jcc_erratum_: bool,
  pub(crate) has_cetss_: bool,
  pub(crate) has_osxsave_: bool,
  pub(crate) has_avx_: bool,
  pub(crate) has_avx2_: bool,
  pub(crate) has_avx_vnni_: bool,
  pub(crate) has_avx_vnni_int8_: bool,
  pub(crate) has_fma3_: bool,
  pub(crate) has_f16c_: bool,
  pub(crate) has_bmi1_: bool,
  pub(crate) has_bmi2_: bool,
  pub(crate) has_lzcnt_: bool,
  pub(crate) has_popcnt_: bool,
  pub(crate) has_apx_f_: bool,
  pub(crate) has_avx10_1_: bool,
  pub(crate) has_idiva_: bool,
  pub(crate) has_neon_: bool,
  pub(crate) has_thumb2_: bool,
  pub(crate) has_vfp_: bool,
  pub(crate) has_vfp3_: bool,
  pub(crate) has_vfp3_d32_: bool,
  pub(crate) has_jscvt_: bool,
  pub(crate) has_dot_prod_: bool,
  pub(crate) has_lse_: bool,
  pub(crate) has_mte_: bool,
  pub(crate) has_sha3_: bool,
  pub(crate) has_pmull1q_: bool,
  pub(crate) has_fp16_: bool,
  pub(crate) has_hbc_: bool,
  pub(crate) has_cssc_: bool,
  pub(crate) has_mops_: bool,
  pub(crate) has_sve_: bool,
  pub(crate) has_svebitperm_: bool,
  pub(crate) is_fp64_mode_: bool,
  pub(crate) has_non_stop_time_stamp_counter_: bool,
  pub(crate) is_running_in_vm_: bool,
  pub(crate) has_msa_: bool,
  pub(crate) riscv_mmu_: RvMmuMode,
  pub(crate) vlen_: u32,
  pub(crate) has_rvv_: bool,
  pub(crate) has_zba_: bool,
  pub(crate) has_zbb_: bool,
  pub(crate) has_zbs_: bool,
  pub(crate) has_zfa_: bool,
  pub(crate) has_zfh_: bool,
  pub(crate) has_zvfh_: bool,
  pub(crate) has_rvc_: bool,
  pub(crate) has_lsx_: bool,
  pub(crate) has_lasx_: bool,
}

impl Default for Cpu {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl Cpu {
  /// CPU(): vendor 置 "Unknown" 后进入 DetectFeatures (cpu.cc:215-225)
  pub fn new() -> Self {
    let mut cpu = Self::unknown();
    cpu.detect_features();
    cpu
  }

  /// 原版成员默认初始化 (cpu.h:169-242)
  fn unknown() -> Self {
    let mut vendor_ = [0u8; 13];
    vendor_[..8].copy_from_slice(b"Unknown\0");
    Self {
      vendor_,
      stepping_: 0,
      model_: 0,
      ext_model_: 0,
      family_: 0,
      ext_family_: 0,
      type_: 0,
      implementer_: 0,
      architecture_: 0,
      variant_: -1,
      part_: 0,
      icache_line_size_: K_UNKNOWN_CACHE_LINE_SIZE,
      dcache_line_size_: K_UNKNOWN_CACHE_LINE_SIZE,
      num_virtual_address_bits_: K_UNKNOWN_NUM_VIRTUAL_ADDRESS_BITS,
      has_fpu_: false,
      has_cmov_: false,
      has_sahf_: false,
      has_mmx_: false,
      has_sse_: false,
      has_sse2_: false,
      has_sse3_: false,
      has_ssse3_: false,
      has_sse41_: false,
      has_sse42_: false,
      is_atom_: false,
      has_intel_jcc_erratum_: false,
      has_cetss_: false,
      has_osxsave_: false,
      has_avx_: false,
      has_avx2_: false,
      has_avx_vnni_: false,
      has_avx_vnni_int8_: false,
      has_fma3_: false,
      has_f16c_: false,
      has_bmi1_: false,
      has_bmi2_: false,
      has_lzcnt_: false,
      has_popcnt_: false,
      has_apx_f_: false,
      has_avx10_1_: false,
      has_idiva_: false,
      has_neon_: false,
      has_thumb2_: false,
      has_vfp_: false,
      has_vfp3_: false,
      has_vfp3_d32_: false,
      has_jscvt_: false,
      has_dot_prod_: false,
      has_lse_: false,
      has_mte_: false,
      has_sha3_: false,
      has_pmull1q_: false,
      has_fp16_: false,
      has_hbc_: false,
      has_cssc_: false,
      has_mops_: false,
      has_sve_: false,
      has_svebitperm_: false,
      is_fp64_mode_: false,
      has_non_stop_time_stamp_counter_: false,
      is_running_in_vm_: false,
      has_msa_: false,
      riscv_mmu_: RvMmuMode::RiscvSv48,
      vlen_: K_UNKNOWN_VLEN,
      has_rvv_: false,
      has_zba_: false,
      has_zbb_: false,
      has_zbs_: false,
      has_zfa_: false,
      has_zfh_: false,
      has_zvfh_: false,
      has_rvc_: false,
      has_lsx_: false,
      has_lasx_: false,
    }
  }

  /// static const CPU& CPU::GetInstance() — 惰性单例 (cpu.cc:227-230)
  pub fn get_instance() -> &'static Self {
    static INSTANCE: OnceLock<Cpu> = OnceLock::new();
    INSTANCE.get_or_init(Self::new)
  }

  // ---- 访问器 (cpu.h:40-156 逐一对照) ----

  /// x86 CPUID 厂商串 ("GenuineIntel" / "AuthenticAMD" / "Unknown")
  #[must_use]
  pub fn vendor(&self) -> &str {
    let end = self.vendor_.iter().position(|&c| c == 0).unwrap_or(13);
    from_utf8(&self.vendor_[..end]).unwrap_or("")
  }
  /// stepping()
  #[must_use]
  pub const fn stepping(&self) -> i32 {
    self.stepping_
  }
  /// model()
  #[must_use]
  pub const fn model(&self) -> i32 {
    self.model_
  }
  /// ext_model()
  #[must_use]
  pub const fn ext_model(&self) -> i32 {
    self.ext_model_
  }
  /// family()
  #[must_use]
  pub const fn family(&self) -> i32 {
    self.family_
  }
  /// ext_family()
  #[must_use]
  pub const fn ext_family(&self) -> i32 {
    self.ext_family_
  }
  /// type()
  #[must_use]
  pub const fn r#type(&self) -> i32 {
    self.type_
  }
  /// arm implementer
  #[must_use]
  pub const fn implementer(&self) -> i32 {
    self.implementer_
  }
  /// arm architecture
  #[must_use]
  pub const fn architecture(&self) -> i32 {
    self.architecture_
  }
  /// arm variant
  #[must_use]
  pub const fn variant(&self) -> i32 {
    self.variant_
  }
  /// arm part
  #[must_use]
  pub const fn part(&self) -> i32 {
    self.part_
  }
  /// has_fpu()
  #[must_use]
  pub const fn has_fpu(&self) -> bool {
    self.has_fpu_
  }
  /// icache_line_size()
  #[must_use]
  pub const fn icache_line_size(&self) -> i32 {
    self.icache_line_size_
  }
  /// dcache_line_size()
  #[must_use]
  pub const fn dcache_line_size(&self) -> i32 {
    self.dcache_line_size_
  }
  /// has_cmov()
  #[must_use]
  pub const fn has_cmov(&self) -> bool {
    self.has_cmov_
  }
  /// has_sahf()
  #[must_use]
  pub const fn has_sahf(&self) -> bool {
    self.has_sahf_
  }
  /// has_mmx()
  #[must_use]
  pub const fn has_mmx(&self) -> bool {
    self.has_mmx_
  }
  /// has_sse()
  #[must_use]
  pub const fn has_sse(&self) -> bool {
    self.has_sse_
  }
  /// has_sse2()
  #[must_use]
  pub const fn has_sse2(&self) -> bool {
    self.has_sse2_
  }
  /// has_sse3()
  #[must_use]
  pub const fn has_sse3(&self) -> bool {
    self.has_sse3_
  }
  /// has_ssse3()
  #[must_use]
  pub const fn has_ssse3(&self) -> bool {
    self.has_ssse3_
  }
  /// has_sse41()
  #[must_use]
  pub const fn has_sse41(&self) -> bool {
    self.has_sse41_
  }
  /// has_sse42()
  #[must_use]
  pub const fn has_sse42(&self) -> bool {
    self.has_sse42_
  }
  /// has_osxsave()
  #[must_use]
  pub const fn has_osxsave(&self) -> bool {
    self.has_osxsave_
  }
  /// has_avx()
  #[must_use]
  pub const fn has_avx(&self) -> bool {
    self.has_avx_
  }
  /// has_avx2()
  #[must_use]
  pub const fn has_avx2(&self) -> bool {
    self.has_avx2_
  }
  /// has_avx_vnni()
  #[must_use]
  pub const fn has_avx_vnni(&self) -> bool {
    self.has_avx_vnni_
  }
  /// has_avx_vnni_int8()
  #[must_use]
  pub const fn has_avx_vnni_int8(&self) -> bool {
    self.has_avx_vnni_int8_
  }
  /// has_fma3()
  #[must_use]
  pub const fn has_fma3(&self) -> bool {
    self.has_fma3_
  }
  /// has_f16c()
  #[must_use]
  pub const fn has_f16c(&self) -> bool {
    self.has_f16c_
  }
  /// has_bmi1()
  #[must_use]
  pub const fn has_bmi1(&self) -> bool {
    self.has_bmi1_
  }
  /// has_bmi2()
  #[must_use]
  pub const fn has_bmi2(&self) -> bool {
    self.has_bmi2_
  }
  /// has_lzcnt()
  #[must_use]
  pub const fn has_lzcnt(&self) -> bool {
    self.has_lzcnt_
  }
  /// has_popcnt()
  #[must_use]
  pub const fn has_popcnt(&self) -> bool {
    self.has_popcnt_
  }
  /// has_apx_f()
  #[must_use]
  pub const fn has_apx_f(&self) -> bool {
    self.has_apx_f_
  }
  /// has_avx10_1()
  #[must_use]
  pub const fn has_avx10_1(&self) -> bool {
    self.has_avx10_1_
  }
  /// is_atom()
  #[must_use]
  pub const fn is_atom(&self) -> bool {
    self.is_atom_
  }
  /// has_intel_jcc_erratum()
  #[must_use]
  pub const fn has_intel_jcc_erratum(&self) -> bool {
    self.has_intel_jcc_erratum_
  }
  /// has_cetss()
  #[must_use]
  pub const fn has_cetss(&self) -> bool {
    self.has_cetss_
  }
  /// has_non_stop_time_stamp_counter()
  #[must_use]
  pub const fn has_non_stop_time_stamp_counter(&self) -> bool {
    self.has_non_stop_time_stamp_counter_
  }
  /// is_running_in_vm()
  #[must_use]
  pub const fn is_running_in_vm(&self) -> bool {
    self.is_running_in_vm_
  }
  /// exposes_num_virtual_address_bits()
  #[must_use]
  pub const fn exposes_num_virtual_address_bits(&self) -> bool {
    self.num_virtual_address_bits_ != K_UNKNOWN_NUM_VIRTUAL_ADDRESS_BITS
  }
  /// num_virtual_address_bits() (原版 DCHECK(exposes))
  #[must_use]
  pub const fn num_virtual_address_bits(&self) -> i32 {
    debug_assert!(self.exposes_num_virtual_address_bits());
    self.num_virtual_address_bits_
  }
  /// has_idiva()
  #[must_use]
  pub const fn has_idiva(&self) -> bool {
    self.has_idiva_
  }
  /// has_neon()
  #[must_use]
  pub const fn has_neon(&self) -> bool {
    self.has_neon_
  }
  /// has_thumb2()
  #[must_use]
  pub const fn has_thumb2(&self) -> bool {
    self.has_thumb2_
  }
  /// has_vfp()
  #[must_use]
  pub const fn has_vfp(&self) -> bool {
    self.has_vfp_
  }
  /// has_vfp3()
  #[must_use]
  pub const fn has_vfp3(&self) -> bool {
    self.has_vfp3_
  }
  /// has_vfp3_d32()
  #[must_use]
  pub const fn has_vfp3_d32(&self) -> bool {
    self.has_vfp3_d32_
  }
  /// has_jscvt()
  #[must_use]
  pub const fn has_jscvt(&self) -> bool {
    self.has_jscvt_
  }
  /// has_dot_prod()
  #[must_use]
  pub const fn has_dot_prod(&self) -> bool {
    self.has_dot_prod_
  }
  /// has_lse()
  #[must_use]
  pub const fn has_lse(&self) -> bool {
    self.has_lse_
  }
  /// has_mte()
  #[must_use]
  pub const fn has_mte(&self) -> bool {
    self.has_mte_
  }
  /// has_sha3()
  #[must_use]
  pub const fn has_sha3(&self) -> bool {
    self.has_sha3_
  }
  /// has_pmull1q()
  #[must_use]
  pub const fn has_pmull1q(&self) -> bool {
    self.has_pmull1q_
  }
  /// has_fp16()
  #[must_use]
  pub const fn has_fp16(&self) -> bool {
    self.has_fp16_
  }
  /// has_hbc()
  #[must_use]
  pub const fn has_hbc(&self) -> bool {
    self.has_hbc_
  }
  /// has_cssc()
  #[must_use]
  pub const fn has_cssc(&self) -> bool {
    self.has_cssc_
  }
  /// has_mops()
  #[must_use]
  pub const fn has_mops(&self) -> bool {
    self.has_mops_
  }
  /// has_sve()
  #[must_use]
  pub const fn has_sve(&self) -> bool {
    self.has_sve_
  }
  /// has_svebitperm()
  #[must_use]
  pub const fn has_svebitperm(&self) -> bool {
    self.has_svebitperm_
  }
  /// is_fp64_mode() (mips)
  #[must_use]
  pub const fn is_fp64_mode(&self) -> bool {
    self.is_fp64_mode_
  }
  /// has_msa() (mips)
  #[must_use]
  pub const fn has_msa(&self) -> bool {
    self.has_msa_
  }
  /// vlen() (riscv)
  #[must_use]
  pub const fn vlen(&self) -> u32 {
    self.vlen_
  }
  /// has_rvv() (riscv)
  #[must_use]
  pub const fn has_rvv(&self) -> bool {
    self.has_rvv_
  }
  /// has_zba() (riscv)
  #[must_use]
  pub const fn has_zba(&self) -> bool {
    self.has_zba_
  }
  /// has_zbb() (riscv)
  #[must_use]
  pub const fn has_zbb(&self) -> bool {
    self.has_zbb_
  }
  /// has_zbs() (riscv)
  #[must_use]
  pub const fn has_zbs(&self) -> bool {
    self.has_zbs_
  }
  /// has_zfa() (riscv)
  #[must_use]
  pub const fn has_zfa(&self) -> bool {
    self.has_zfa_
  }
  /// has_rvc() (riscv)
  #[must_use]
  pub const fn has_rvc(&self) -> bool {
    self.has_rvc_
  }
  /// has_zfh() (riscv)
  #[must_use]
  pub const fn has_zfh(&self) -> bool {
    self.has_zfh_
  }
  /// has_zvfh() (riscv)
  #[must_use]
  pub const fn has_zvfh(&self) -> bool {
    self.has_zvfh_
  }
  /// riscv_mmu()
  #[must_use]
  pub const fn riscv_mmu(&self) -> RvMmuMode {
    self.riscv_mmu_
  }
  /// has_lsx() (loong64)
  #[must_use]
  pub const fn has_lsx(&self) -> bool {
    self.has_lsx_
  }
  /// has_lasx() (loong64)
  #[must_use]
  pub const fn has_lasx(&self) -> bool {
    self.has_lasx_
  }

  /// void CPU::DetectFeatures() — 按宿主架构分派
  fn detect_features(&mut self) {
    #[cfg(target_arch = "x86_64")]
    self.detect_x86();
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    self.detect_arm64_darwin();
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    self.detect_arm64_linux();
    #[cfg(not(any(
      target_arch = "x86_64",
      all(target_arch = "aarch64", target_os = "macos"),
      all(target_arch = "aarch64", target_os = "linux")
    )))]
    {
      // 其余架构 (arm32/mips64/riscv64/loong64/ia32): 原版为编译期选
      // cpu-<arch>.cc, 本快照后置 (todo: 先 x64+arm64); 保持 Unknown 默认值
    }
  }

  /// cpu-x86.cc:85-251 — CPUID 全量探测
  #[cfg(target_arch = "x86_64")]
  fn detect_x86(&mut self) {
    use core::arch::x86_64::__cpuid_count;
    // [eax, ebx, ecx, edx], 对应原版 int cpu_info[4]
    let cpuid4 = |leaf: u32, sub: u32| {
      let r = __cpuid_count(leaf, sub);
      [r.eax, r.ebx, r.ecx, r.edx]
    };

    // leaf 0: 最大有效 leaf + 厂商串 (原版 swap(ecx,edx) 后 memcpy 12 字节
    // ⇒ 字节序为 ebx|edx|ecx)
    let info0 = cpuid4(0, 0);
    let num_ids = info0[0];
    self.vendor_[0..4].copy_from_slice(&info0[1].to_le_bytes());
    self.vendor_[4..8].copy_from_slice(&info0[3].to_le_bytes());
    self.vendor_[8..12].copy_from_slice(&info0[2].to_le_bytes());
    self.vendor_[12] = 0;

    if num_ids > 0 {
      let info = cpuid4(1, 0);
      let mut info70 = [0u32; 4];
      let mut info71 = [0u32; 4];
      if num_ids >= 7 {
        info70 = cpuid4(7, 0);
        // leaf 7 支持的子 leaf 上限
        if info70[0] >= 1 {
          info71 = cpuid4(7, 1);
        }
      }

      self.stepping_ = (info[0] & 0xF) as i32;
      self.model_ = (((info[0] >> 4) & 0xF) + ((info[0] >> 12) & 0xF0)) as i32;
      self.family_ = ((info[0] >> 8) & 0xF) as i32;
      self.type_ = ((info[0] >> 12) & 0x3) as i32;
      self.ext_model_ = ((info[0] >> 16) & 0xF) as i32;
      self.ext_family_ = ((info[0] >> 20) & 0xFF) as i32;
      self.has_fpu_ = (info[3] & 0x0000_0001) != 0;
      self.has_cmov_ = (info[3] & 0x0000_8000) != 0;
      self.has_mmx_ = (info[3] & 0x0080_0000) != 0;
      self.has_sse_ = (info[3] & 0x0200_0000) != 0;
      self.has_sse2_ = (info[3] & 0x0400_0000) != 0;
      self.has_sse3_ = (info[2] & 0x0000_0001) != 0;
      self.has_ssse3_ = (info[2] & 0x0000_0200) != 0;
      self.has_sse41_ = (info[2] & 0x0008_0000) != 0;
      self.has_sse42_ = (info[2] & 0x0010_0000) != 0;
      self.has_popcnt_ = (info[2] & 0x0080_0000) != 0;
      self.has_osxsave_ = (info[2] & 0x0800_0000) != 0;
      self.has_avx_ = (info[2] & 0x1000_0000) != 0;
      self.has_avx2_ = (info70[1] & 0x0000_0020) != 0;
      self.has_avx_vnni_ = (info71[0] & 0x0000_0010) != 0;
      self.has_avx_vnni_int8_ = (info71[3] & 0x0000_0010) != 0;
      self.has_fma3_ = (info[2] & 0x0000_1000) != 0;
      self.has_f16c_ = (info[2] & 0x2000_0000) != 0;
      self.has_apx_f_ = (info71[3] & 0x0020_0000) != 0;
      if (info71[3] & 0x0008_0000) != 0 && num_ids >= 0x24 {
        let info24 = cpuid4(0x24, 0);
        self.has_avx10_1_ = (info24[1] & 0xFF) >= 1;
        debug_assert!(imply(self.has_avx10_1_, (info24[1] & 0x70000) == 0x70000));
      }
      // CET shadow stack
      self.has_cetss_ = (info70[2] & 0x0000_0080) != 0;
      // Hypervisor Present Bit: ECX bit31 of leaf 1
      self.is_running_in_vm_ = (info[2] & 0x8000_0000) != 0;

      if self.family_ == 0x6 {
        // Atom 型号族 (cpu-x86.cc:157-169)
        self.is_atom_ = matches!(
          self.model_,
          0x1C | 0x26 | 0x36 | 0x27 | 0x35 | 0x37 | 0x4A | 0x4D | 0x4C | 0x6E
        );
        // Intel JCC erratum 受影响型号 (cpu-x86.cc:172-197)
        self.has_intel_jcc_erratum_ = match self.model_ {
          0x4E => self.stepping_ == 0x3,
          0x55 => matches!(self.stepping_, 0x4 | 0x7),
          0x5E => self.stepping_ == 0x3,
          0x8E => matches!(self.stepping_, 0x9..=0xC),
          0x9E => matches!(self.stepping_, 0x9..=0xB | 0xD),
          0xA6 => self.stepping_ == 0x0,
          0xAE => self.stepping_ == 0xA,
          _ => false,
        };
      }
    }

    // VEX 编码 GPR 指令的独立特性位
    if num_ids >= 7 {
      let info = cpuid4(7, 0);
      self.has_bmi1_ = (info[1] & 0x0000_0008) != 0;
      self.has_bmi2_ = (info[1] & 0x0000_0100) != 0;
    }

    // 扩展 leaf
    let info = cpuid4(0x8000_0000, 0);
    let num_ext_ids = info[0];

    if num_ext_ids > 0x8000_0000 {
      let info = cpuid4(0x8000_0001, 0);
      self.has_lzcnt_ = (info[2] & 0x0000_0020) != 0;
      // SAHF 须在长模式探测
      self.has_sahf_ = (info[2] & 0x0000_0001) != 0;
    }

    // 不停时戳计数器
    const NON_STOP_TSC_LEAF: u32 = 0x8000_0007;
    if num_ext_ids >= NON_STOP_TSC_LEAF {
      let info = cpuid4(NON_STOP_TSC_LEAF, 0);
      self.has_non_stop_time_stamp_counter_ = (info[3] & (1 << 8)) != 0;
    }

    const VIRTUAL_PHYSICAL_ADDRESS_BITS: u32 = 0x8000_0008;
    if num_ext_ids >= VIRTUAL_PHYSICAL_ADDRESS_BITS {
      let info = cpuid4(VIRTUAL_PHYSICAL_ADDRESS_BITS, 0);
      self.num_virtual_address_bits_ = ((info[0] >> 8) & 0xFF) as i32;
    }

    // Hyper-V 变体 TSC 视作恒定 (cpu-x86.cc:233-249)
    if !self.has_non_stop_time_stamp_counter_ && self.is_running_in_vm_ {
      let info_hv = cpuid4(0x4000_0000, 0);
      if info_hv[1] == 0x7263_694D && // "Micr"
         info_hv[2] == 0x666F_736F && // "osof"
         info_hv[3] == 0x7648_2074
      {
        // "t Hv"
        self.has_non_stop_time_stamp_counter_ = true;
      }
    }
  }

  /// cpu-arm.cc:404-471 — ARM64 macOS:
  /// 非 iOS 的 ARM64 Mac 恒有 jscvt/dotprod/lse/pmull1q/fp16/sha3;
  /// CSSC 经 sysctl 探测
  #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
  fn detect_arm64_darwin(&mut self) {
    use core::mem::size_of;
    use std::ptr::null_mut;
    self.has_jscvt_ = true;
    self.has_dot_prod_ = true;
    self.has_lse_ = true;
    self.has_pmull1q_ = true;
    self.has_fp16_ = true;
    self.has_sha3_ = true;

    let mut feat_cssc: i64 = 0;
    let mut feat_cssc_size = size_of::<i64>();
    // SAFETY: name 为合法 C 字符串; 出参指针与大小匹配且在作用域内
    let rc = unsafe {
      libc::sysctlbyname(
        c"hw.optional.arm.FEAT_CSSC".as_ptr(),
        (&mut feat_cssc as *mut i64).cast(),
        &mut feat_cssc_size,
        null_mut(),
        0,
      )
    };
    self.has_cssc_ = rc != -1 && feat_cssc != 0;
  }

  /// cpu-arm.cc:366-395 — ARM64 Linux: ELF hwcaps 位判定
  #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
  fn detect_arm64_linux(&mut self) {
    // <uapi/asm/hwcap.h>
    const HWCAP_ATOMICS: u64 = 1 << 8;
    const HWCAP_FPHP: u64 = 1 << 9;
    const HWCAP_PMULL: u64 = 1 << 4;
    const HWCAP_JSCVT: u64 = 1 << 13;
    const HWCAP_SHA3: u64 = 1 << 17;
    const HWCAP_ASIMDDP: u64 = 1 << 20;
    const HWCAP_SVE: u64 = 1 << 22;
    const HWCAP2_SVEBITPERM: u64 = 1 << 4;
    const HWCAP2_MTE: u64 = 1 << 18;
    const HWCAP2_CSSC: u64 = 1 << 34;
    const HWCAP2_MOPS: u64 = 1 << 43;
    const HWCAP2_HBC: u64 = 1 << 44;

    // SAFETY: getauxval 为纯读取 auxv 的库函数
    let (hwcaps, hwcaps2) = unsafe {
      (
        libc::getauxval(libc::AT_HWCAP) as u64,
        libc::getauxval(libc::AT_HWCAP2) as u64,
      )
    };
    self.has_cssc_ = (hwcaps2 & HWCAP2_CSSC) != 0;
    self.has_mte_ = (hwcaps2 & HWCAP2_MTE) != 0;
    self.has_hbc_ = (hwcaps2 & HWCAP2_HBC) != 0;
    self.has_mops_ = (hwcaps2 & HWCAP2_MOPS) != 0;
    self.has_svebitperm_ = (hwcaps2 & HWCAP2_SVEBITPERM) != 0;
    if hwcaps != 0 {
      self.has_jscvt_ = (hwcaps & HWCAP_JSCVT) != 0;
      self.has_dot_prod_ = (hwcaps & HWCAP_ASIMDDP) != 0;
      self.has_lse_ = (hwcaps & HWCAP_ATOMICS) != 0;
      self.has_pmull1q_ = (hwcaps & HWCAP_PMULL) != 0;
      self.has_fp16_ = (hwcaps & HWCAP_FPHP) != 0;
      self.has_sha3_ = (hwcaps & HWCAP_SHA3) != 0;
      self.has_sve_ = (hwcaps & HWCAP_SVE) != 0;
    }
    // hwcaps == 0 时的 /proc/cpuinfo "Features" 回退路径不迁移 (登记):
    // 现代 内核 auxv 恒可用, 原版回退仅覆盖极老 用户态
  }
}

/// DCHECK_IMPLIES(a, b) ⇒ a → b
#[cfg(target_arch = "x86_64")]
const fn imply(a: bool, b: bool) -> bool {
  !a || b
}
