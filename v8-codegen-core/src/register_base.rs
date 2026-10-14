// 由 tool/src/codegen/register-base.h.js 从 v8/src/codegen/register-base.h;src/codegen/reglist-base.h;src/codegen/register.h;src/codegen/reglist.h;src/codegen/register-arch.h;src/codegen/constants-arch.h;src/codegen/assembler-arch.h 生成; 修订就地改, 复跑覆盖
// 寄存器基类常量 + RegList 位集语义 + 平台派发表

// ===== RegisterBase (v8/src/codegen/register-base.h) =====

/// RegisterBase::kCode_no_reg
pub const K_CODE_NO_REG: i8 = -1;
/// kNumRegisters = kAfterLastRegister (实例化时定值; 各架构覆盖脚本落实际值)
/// static_assert(kAfterLastRegister <= kMaxInt8) ⇒ i8 上界
pub const K_MAX_INT8: i32 = 127;

/// RegListBase 存储宽度选择 (原版 num_registers_sized_storage_t):
/// kNumRegisters <= 16 → u16; <= 32 → u32; 其他 → u64; 总上限 64。
#[inline]
#[must_use]
pub const fn storage_bits_for(num_registers: usize) -> usize {
  if num_registers <= 16 {
    16
  } else if num_registers <= 32 {
    32
  } else {
    64
  }
}

/// 原版 RegListBase<RegisterT>: 位集寄存器列表。
/// ARM64 特例: 通用寄存器列表因 sp 内部码 63 用 u64 存储 (storage_t 特化)。
/// 泛型宿主类型由各架构寄存器模块实例化。
pub mod reglist_ops {
  /// 原版 set: regs |= 1 << code (无效寄存器为空操作)
  #[inline]
  #[must_use]
  pub const fn set(bits: u64, code: i8) -> u64 {
    if code < 0 { return bits; }
    bits | (1u64 << code)
  }

  /// 原版 clear: regs &= ~(1 << code)
  #[inline]
  #[must_use]
  pub const fn clear(bits: u64, code: i8) -> u64 {
    if code < 0 { return bits; }
    bits & !(1u64 << code)
  }

  /// 原版 has
  #[inline]
  #[must_use]
  pub const fn has(bits: u64, code: i8) -> bool {
    if code < 0 { return false; }
    (bits & (1u64 << code)) != 0
  }

  /// 原版 Count: CountPopulation
  #[inline]
  #[must_use]
  pub const fn count(bits: u64) -> u32 {
    bits.count_ones()
  }

  /// 原版 first: CountTrailingZerosNonZero (非空前提)
  #[inline]
  #[must_use]
  pub const fn first_code(bits: u64) -> i8 {
    bits.trailing_zeros() as i8
  }

  /// 原版 last: 8*sizeof - 1 - CountLeadingZeros
  #[inline]
  #[must_use]
  pub const fn last_code(bits: u64) -> i8 {
    (64 - 1 - bits.leading_zeros()) as i8
  }

  /// 原版 operator-: 差集
  #[inline]
  #[must_use]
  pub const fn difference(a: u64, b: u64) -> u64 {
    a & !b
  }
}

// ===== register.h 平台无关面 =====

/// 原版 AddArgumentPaddingSlots: argument_count + ArgumentPaddingSlots(argument_count)
/// (ArgumentPaddingSlots 由架构常量面提供, 签名骨架)
#[inline]
#[must_use]
pub const fn add_argument_padding_slots(argument_count: i32, arch_padding_slots: i32) -> i32 {
  argument_count + arch_padding_slots
}

/// 原版 ShouldPadArguments
#[inline]
#[must_use]
pub const fn should_pad_arguments(arch_padding_slots: i32) -> bool {
  arch_padding_slots != 0
}

/// 原版 AreAliased: 多寄存器别名判定 (签名骨架; 判定式: 去重数 < 有效数)
#[inline]
#[must_use]
pub fn are_aliased(codes: &[i8]) -> bool {
  let mut seen = 0u64;
  let mut valid = 0usize;
  for &c in codes {
    if c >= 0 {
      valid += 1;
      seen |= 1u64 << c;
    }
  }
  seen.count_ones() as usize < valid
}

// ===== reglist.h 平台无关面 =====

/// kEmptyRegList (空列表)
pub const K_EMPTY_REGLIST: u64 = 0;

/// REGISTER_ARCH_DISPATCH 平台派发表 (V8_TARGET_ARCH_* → 头文件)
pub const REGISTER_ARCH_DISPATCH: &[(&str, &str)] = &[
  ("IA32", "src/codegen/ia32/register-ia32.h"), // 非目标架构: 跳过登记
  ("X64", "src/codegen/x64/register-x64.h"), // 目标架构: 生效
  ("ARM64", "src/codegen/arm64/register-arm64.h"), // 目标架构: 生效
  ("ARM", "src/codegen/arm/register-arm.h"), // 非目标架构: 跳过登记
  ("PPC64", "src/codegen/ppc/register-ppc.h"), // 非目标架构: 跳过登记
  ("MIPS64", "src/codegen/mips64/register-mips64.h"), // 非目标架构: 跳过登记
  ("LOONG64", "src/codegen/loong64/register-loong64.h"), // 非目标架构: 跳过登记
  ("S390X", "src/codegen/s390/register-s390.h"), // 非目标架构: 跳过登记
  ("RISCV32", "src/codegen/riscv/register-riscv.h"), // 非目标架构: 跳过登记
];

/// REGLIST_ARCH_DISPATCH 平台派发表 (V8_TARGET_ARCH_* → 头文件)
pub const REGLIST_ARCH_DISPATCH: &[(&str, &str)] = &[
  ("IA32", "src/codegen/ia32/reglist-ia32.h"), // 非目标架构: 跳过登记
  ("X64", "src/codegen/x64/reglist-x64.h"), // 目标架构: 生效
  ("ARM64", "src/codegen/arm64/reglist-arm64.h"), // 目标架构: 生效
  ("ARM", "src/codegen/arm/reglist-arm.h"), // 非目标架构: 跳过登记
  ("PPC64", "src/codegen/ppc/reglist-ppc.h"), // 非目标架构: 跳过登记
  ("MIPS64", "src/codegen/mips64/reglist-mips64.h"), // 非目标架构: 跳过登记
  ("LOONG64", "src/codegen/loong64/reglist-loong64.h"), // 非目标架构: 跳过登记
  ("S390X", "src/codegen/s390/reglist-s390.h"), // 非目标架构: 跳过登记
  ("RISCV32", "src/codegen/riscv/reglist-riscv.h"), // 非目标架构: 跳过登记
];

/// CONSTANTS_ARCH_DISPATCH 平台派发表 (V8_TARGET_ARCH_* → 头文件)
pub const CONSTANTS_ARCH_DISPATCH: &[(&str, &str)] = &[
  ("ARM", "src/codegen/arm/constants-arm.h"), // 非目标架构: 跳过登记
  ("ARM64", "src/codegen/arm64/constants-arm64.h"), // 目标架构: 生效
  ("IA32", "src/codegen/ia32/constants-ia32.h"), // 非目标架构: 跳过登记
  ("MIPS64", "src/codegen/mips64/constants-mips64.h"), // 非目标架构: 跳过登记
  ("LOONG64", "src/codegen/loong64/constants-loong64.h"), // 非目标架构: 跳过登记
  ("PPC64", "src/codegen/ppc/constants-ppc.h"), // 非目标架构: 跳过登记
  ("S390X", "src/codegen/s390/constants-s390.h"), // 非目标架构: 跳过登记
  ("X64", "src/codegen/x64/constants-x64.h"), // 目标架构: 生效
  ("RISCV32", "src/codegen/riscv/constants-riscv.h"), // 非目标架构: 跳过登记
];

/// ASSEMBLER_ARCH_DISPATCH 平台派发表 (V8_TARGET_ARCH_* → 头文件)
pub const ASSEMBLER_ARCH_DISPATCH: &[(&str, &str)] = &[
  ("IA32", "src/codegen/ia32/assembler-ia32.h"), // 非目标架构: 跳过登记
  ("X64", "src/codegen/x64/assembler-x64.h"), // 目标架构: 生效
  ("ARM64", "src/codegen/arm64/assembler-arm64.h"), // 目标架构: 生效
  ("ARM", "src/codegen/arm/assembler-arm.h"), // 非目标架构: 跳过登记
  ("PPC64", "src/codegen/ppc/assembler-ppc.h"), // 非目标架构: 跳过登记
  ("MIPS64", "src/codegen/mips64/assembler-mips64.h"), // 非目标架构: 跳过登记
  ("LOONG64", "src/codegen/loong64/assembler-loong64.h"), // 非目标架构: 跳过登记
  ("S390X", "src/codegen/s390/assembler-s390.h"), // 非目标架构: 跳过登记
  ("RISCV32", "src/codegen/riscv/assembler-riscv.h"), // 非目标架构: 跳过登记
];


/// kAllocatableGeneralRegisters / kAllocatableDoubleRegisters:
/// 内容由各架构 ALLOCATABLE_*_REGISTERS 宏展开, 见对应架构覆盖脚本
/// (x64 → v8-codegen-x64, arm64 → v8-codegen-arm64)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_CODE_NO_REG == -1);
  assert!(K_MAX_INT8 == 127);
  assert!(storage_bits_for(16) == 16);
  assert!(storage_bits_for(17) == 32);
  assert!(storage_bits_for(33) == 64);
  assert!(REGISTER_ARCH_DISPATCH.len() == 9);
  assert!(REGLIST_ARCH_DISPATCH.len() == 9);
  assert!(CONSTANTS_ARCH_DISPATCH.len() == 9);
  assert!(ASSEMBLER_ARCH_DISPATCH.len() == 9);
  assert!(reglist_ops::set(0, 3) == 0b1000);
  assert!(reglist_ops::count(0b1011) == 3);
  assert!(reglist_ops::first_code(0b1100) == 2);
  assert!(reglist_ops::last_code(0b1100) == 3);
  assert!(reglist_ops::difference(0b1111, 0b0011) == 0b1100);
  assert!(are_aliased(&[1, 1, 2]));
  assert!(!are_aliased(&[1, 2, 3]));
};
