// 由 tool/src/wasm/baseline/liftoff-register.h.js 从 v8/src/wasm/baseline/liftoff-register.h 生成; 修订就地改, 复跑覆盖
// Liftoff 寄存器编码常量 (x64+arm64 展开; 其余架构跳过: arm/ia32/loong64/mips64/ppc/riscv/s390)

use v8_wasm_core::value_type::ValueKind;

/// 平台谓词 (机械展开; x64/arm64: 指针 8 字节, FP 别名 kOverlap)
pub const NEED_I64_REG_PAIR: bool = false;
pub const NEED_S128_REG_PAIR: bool = false;
pub const HAS_INDEPENDENT_SIMD128_REGS: bool = false;

/// 寄存器类别 (对应原版 enum RegClass : uint8_t; 本目标 kNoReg=2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RegClass {
  /// kGpReg
  GpReg = 0,
  /// kFpReg
  FpReg = 1,
  /// kNoReg — 聚合哨兵 (原版为 constexpr 别名, 值 2)
  NoReg = 2,
}
/// kSimd128Reg (原版本目标下 == kNoReg)
pub const SIMD128_REG: RegClass = RegClass::NoReg;
/// kGpRegPair (原版本目标下 == kNoReg)
pub const GP_REG_PAIR: RegClass = RegClass::NoReg;
/// kFpRegPair (原版本目标下 == kNoReg)
pub const FP_REG_PAIR: RegClass = RegClass::NoReg;

/// 配对半字 (对应原版 enum RegPairHalf : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RegPairHalf {
  /// kLowWord
  LowWord = 0,
  /// kHighWord
  HighWord = 1,
}

/// ValueKind → RegClass 查询表 (原版 reg_class_for 的 make_array 展开;
/// ValueKind 顺序对齐 value-type.h)
#[rustfmt::skip]
pub static REG_CLASS_FOR: [(ValueKind, RegClass); 11] = [
  (ValueKind::Void, RegClass::NoReg),
  (ValueKind::I32, RegClass::GpReg),
  (ValueKind::I64, RegClass::GpReg),
  (ValueKind::F32, RegClass::FpReg),
  (ValueKind::F64, RegClass::FpReg),
  (ValueKind::S128, RegClass::FpReg),
  (ValueKind::I8, RegClass::GpReg),
  (ValueKind::I16, RegClass::GpReg),
  (ValueKind::F16, RegClass::FpReg),
  (ValueKind::Ref, RegClass::GpReg),
  (ValueKind::RefNull, RegClass::GpReg),
];

/// x64 目标 (V8_TARGET_ARCH_X64)
pub mod x64 {
  /// 最大 gp 缓存寄存器编码 (原版 kMaxGpRegCode)
  pub const MAX_GP_REG_CODE: i32 = 15;
  /// 最大 fp 缓存寄存器编码 (原版 kMaxFpRegCode)
  pub const MAX_FP_REG_CODE: i32 = 7;
  /// simd128 缓存表为空 → -1 (原版 kMaxSimd128RegCode)
  pub const MAX_SIMD128_REG_CODE: i32 = -1;
  /// 原版 kAfterMaxLiftoffGpRegCode
  pub const AFTER_MAX_LIFTOFF_GP_REG_CODE: i32 = 16;
  /// 原版 kAfterMaxLiftoffFpRegCode
  pub const AFTER_MAX_LIFTOFF_FP_REG_CODE: i32 = 24;
  /// 原版 kAfterMaxLiftoffSimd128RegCode
  pub const AFTER_MAX_LIFTOFF_SIMD128_REG_CODE: i32 = 24;
  /// 原版 kAfterMaxLiftoffRegCode
  pub const AFTER_MAX_LIFTOFF_REG_CODE: i32 = 24;
  /// 原版 kBitsPerLiftoffRegCode
  pub const BITS_PER_LIFTOFF_REG_CODE: i32 = 5;
  /// 原版 kBitsPerGpRegCode
  pub const BITS_PER_GP_REG_CODE: i32 = 4;
  /// 原版 kBitsPerFpRegCode
  pub const BITS_PER_FP_REG_CODE: i32 = 3;
  /// 原版 kBitsPerSimd128RegCode
  pub const BITS_PER_SIMD128_REG_CODE: i32 = 0;
  /// 原版 kBitsPerRegPair
  pub const BITS_PER_REG_PAIR: i32 = 9;
}

/// arm64 目标 (V8_TARGET_ARCH_ARM64)
pub mod arm64 {
  /// 最大 gp 缓存寄存器编码 (原版 kMaxGpRegCode)
  pub const MAX_GP_REG_CODE: i32 = 27;
  /// 最大 fp 缓存寄存器编码 (原版 kMaxFpRegCode)
  pub const MAX_FP_REG_CODE: i32 = 27;
  /// simd128 缓存表为空 → -1 (原版 kMaxSimd128RegCode)
  pub const MAX_SIMD128_REG_CODE: i32 = -1;
  /// 原版 kAfterMaxLiftoffGpRegCode
  pub const AFTER_MAX_LIFTOFF_GP_REG_CODE: i32 = 28;
  /// 原版 kAfterMaxLiftoffFpRegCode
  pub const AFTER_MAX_LIFTOFF_FP_REG_CODE: i32 = 56;
  /// 原版 kAfterMaxLiftoffSimd128RegCode
  pub const AFTER_MAX_LIFTOFF_SIMD128_REG_CODE: i32 = 56;
  /// 原版 kAfterMaxLiftoffRegCode
  pub const AFTER_MAX_LIFTOFF_REG_CODE: i32 = 56;
  /// 原版 kBitsPerLiftoffRegCode
  pub const BITS_PER_LIFTOFF_REG_CODE: i32 = 6;
  /// 原版 kBitsPerGpRegCode
  pub const BITS_PER_GP_REG_CODE: i32 = 5;
  /// 原版 kBitsPerFpRegCode
  pub const BITS_PER_FP_REG_CODE: i32 = 5;
  /// 原版 kBitsPerSimd128RegCode
  pub const BITS_PER_SIMD128_REG_CODE: i32 = 0;
  /// 原版 kBitsPerRegPair
  pub const BITS_PER_REG_PAIR: i32 = 11;
}

// 跳过架构登记 (硬规则 9): arm / ia32 / loong64 / mips64 / ppc / riscv(32/64) / s390
// 的 liftoff-register 派生常量按需再展开 — 缓存寄存器表见 liftoff-assembler-defs.h
// 对应架构分支。

// —— 提取不变式断言 ——
const _: () = {
  assert!(!NEED_I64_REG_PAIR);
  assert!(!NEED_S128_REG_PAIR);
  assert!(!HAS_INDEPENDENT_SIMD128_REGS);
  assert!(RegClass::NoReg as u8 == 2);
  assert!(x64::AFTER_MAX_LIFTOFF_REG_CODE == 24);
  assert!(x64::BITS_PER_LIFTOFF_REG_CODE == 5);
  assert!(x64::BITS_PER_GP_REG_CODE == 4);
  assert!(x64::BITS_PER_REG_PAIR == 9);
  assert!(arm64::AFTER_MAX_LIFTOFF_REG_CODE == 56);
  assert!(arm64::BITS_PER_LIFTOFF_REG_CODE == 6);
};
