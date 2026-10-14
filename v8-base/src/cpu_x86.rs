// 由 tool/src/base/cpu/cpu-x86.cc.js 从 v8/src/base/cpu/cpu-x86.cc 生成; 修订就地改, 复跑覆盖
// x64 CPUID 特征位表 (V8_HOST_ARCH_X64 分支; IA32/MSVC/PIC 汇编变体登记不产码)

/// CPUID 特征位表 (原版 has_<name>_ 位测, 列: 特征|寄存器|下标|掩码)
/// 寄存器来源: cpu_info=leaf1 EAX 页, cpu_info70=leaf7, cpu_info71=leaf7 子页1, cpu_info24=leaf0x24
pub const CPUID_FEATURES: [(&str, &str, u32, u32); 24] = [
  ("fpu", "cpu_info", 3, 0x00000001),
  ("cmov", "cpu_info", 3, 0x00008000),
  ("mmx", "cpu_info", 3, 0x00800000),
  ("sse", "cpu_info", 3, 0x02000000),
  ("sse2", "cpu_info", 3, 0x04000000),
  ("sse3", "cpu_info", 2, 0x00000001),
  ("ssse3", "cpu_info", 2, 0x00000200),
  ("sse41", "cpu_info", 2, 0x00080000),
  ("sse42", "cpu_info", 2, 0x00100000),
  ("popcnt", "cpu_info", 2, 0x00800000),
  ("osxsave", "cpu_info", 2, 0x08000000),
  ("avx", "cpu_info", 2, 0x10000000),
  ("avx2", "cpu_info70", 1, 0x00000020),
  ("avx_vnni", "cpu_info71", 0, 0x00000010),
  ("avx_vnni_int8", "cpu_info71", 3, 0x00000010),
  ("fma3", "cpu_info", 2, 0x00001000),
  ("f16c", "cpu_info", 2, 0x20000000),
  ("apx_f", "cpu_info71", 3, 0x00200000),
  ("cetss", "cpu_info70", 2, 0x00000080),
  ("bmi1", "cpu_info", 1, 0x00000008),
  ("bmi2", "cpu_info", 1, 0x00000100),
  ("lzcnt", "cpu_info", 2, 0x00000020),
  ("sahf", "cpu_info", 2, 0x00000001),
  ("non_stop_time_stamp_counter", "cpu_info", 3, (1 << 8)),
];

/// 身份字段 (原版 DetectFeatures 内公式, EAX 分段提取)
/// stepping=EAX[3:0], model=EAX[7:4]+EAX[19:16]<<4, family=EAX[11:8], type=EAX[13:12], ext_model=EAX[19:16]
pub const ID_FIELDS: [&str; 7] = [
  "stepping",
  "model",
  "family",
  "type",
  "ext_model",
  "ext_family",
  "num_virtual_address_bits",
];

/// 按表测位 (对 CPUID 结果页应用单条特征)
#[inline]
#[must_use]
pub fn has_feature(regs: &[(u32; 4)], feature: &str) -> bool {
  let Some((_, src, idx, mask)) = CPUID_FEATURES.iter().find(|f| f.0 == feature) else {
    return false;
  };
  let page = match *src {
    "cpu_info" => 0,
    "cpu_info70" => 1,
    "cpu_info71" => 2,
    "cpu_info24" => 3,
    _ => return false,
  };
  regs.get(page).is_some_and(|r| r[*idx as usize] & mask != 0)
}

/// 非机械段登记 (rule: 不编造, 语义归 Developer)
/// has_avx10_1_ = (cpu_info24[1] & 0xff) >= 1 (比较式, 非位测) / has_intel_jcc_erratum_ (型号/stepping 表, 语义判定)
pub const NON_MECHANICAL: [&str; 2] = ["avx10_1_compare", "intel_jcc_erratum_table"];

// —— 提取不变式断言 ——
const _: () = {
  assert!(CPUID_FEATURES.len() == 24);
  assert!(CPUID_FEATURES[0].0 == "fpu");
  assert!(CPUID_FEATURES[0].3 == 0x1);
  assert!(ID_FIELDS.contains(&"stepping"));
};
