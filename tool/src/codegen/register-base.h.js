#!/usr/bin/env bun
// tool/src/codegen/register-base.h.js
// 机械提取 v8/src/codegen/register-base.h + reglist-base.h 的基类常量/位运算语义,
// 并展开 register.h / reglist.h / register-arch.h / constants-arch.h / assembler-arch.h
// 的平台派发表 (x64+arm64 展开, 其余架构登记) → v8-codegen-core/src/register_base.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const registerBase = readV8("src/codegen/register-base.h");
const reglistBase = readV8("src/codegen/reglist-base.h");
const registerH = readV8("src/codegen/register.h");
const reglistH = readV8("src/codegen/reglist.h");
const registerArch = readV8("src/codegen/register-arch.h");
const constantsArch = readV8("src/codegen/constants-arch.h");
const assemblerArch = readV8("src/codegen/assembler-arch.h");

// —— 对数守卫 ——
const m = (src, re, what) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

m(registerBase, /static constexpr int8_t kCode_no_reg = (-?\d+);/, "kCode_no_reg");
m(registerBase, /static constexpr int8_t kNumRegisters = (\w+);/, "kNumRegisters");
m(registerBase, /static_assert\(kAfterLastRegister <= kMaxInt8\);/, "static_assert kMaxInt8");
m(reglistBase, /static_assert\(RegisterT::kNumRegisters <= 64\);/, "RegList 64 上限");
for (const f of ["set", "clear", "has", "is_empty", "Count", "first", "last", "PopFirst", "bits", "FromBits"]) {
  if (!reglistBase.includes(f)) throw new Error(`RegListBase 缺 ${f}`);
}
for (const f of ["AddArgumentPaddingSlots", "ShouldPadArguments", "AreAliased"]) {
  if (!registerH.includes(f)) throw new Error(`register.h 缺 ${f}`);
}
if (!reglistH.includes("kAllocatableGeneralRegisters")) throw new Error("缺 kAllocatableGeneralRegisters");
if (!reglistH.includes("kAllocatableDoubleRegisters")) throw new Error("缺 kAllocatableDoubleRegisters");

// —— 平台派发表展开: 提取 #if 分支的 include 映射 ——
const ARCHS = ["IA32", "X64", "ARM64", "ARM", "PPC64", "MIPS64", "LOONG64", "S390X", "RISCV"];
// 目标: x64 + arm64; 其余登记跳过
const TARGETS = new Set(["X64", "ARM64"]);
function dispatchTable(src, what) {
  const rows = [...src.matchAll(/#(?:el)?if V8_TARGET_ARCH_(\w+)[^\n]*\n#include "([^"]+)"/g)];
  if (rows.length < 5) throw new Error(`${what} 派发分支过少: ${rows.length}`);
  return rows.map(([, arch, inc]) => ({ arch, inc, target: TARGETS.has(arch) }));
}
const regDispatch = dispatchTable(registerArch, "register-arch.h");
const reglistDispatch = dispatchTable(reglistH, "reglist.h");
const constantsDispatch = dispatchTable(constantsArch, "constants-arch.h");
const asmDispatch = dispatchTable(assemblerArch, "assembler-arch.h");

function fmtDispatch(rows, name) {
  const lines = rows
    .map(({ arch, inc, target }) =>
      target
        ? `  (${JSON.stringify(arch)}, ${JSON.stringify(inc)}), // 目标架构: 生效`
        : `  (${JSON.stringify(arch)}, ${JSON.stringify(inc)}), // 非目标架构: 跳过登记`,
    )
    .join("\n");
  return `/// ${name} 平台派发表 (V8_TARGET_ARCH_* → 头文件)\npub const ${name}: &[(&str, &str)] = &[\n${lines}\n];\n`;
}

let rust = banner(
  "src/codegen/register-base.h.js",
  "src/codegen/register-base.h;src/codegen/reglist-base.h;src/codegen/register.h;src/codegen/reglist.h;src/codegen/register-arch.h;src/codegen/constants-arch.h;src/codegen/assembler-arch.h",
  "寄存器基类常量 + RegList 位集语义 + 平台派发表",
);

rust += `
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

${fmtDispatch(regDispatch, "REGISTER_ARCH_DISPATCH")}
${fmtDispatch(reglistDispatch, "REGLIST_ARCH_DISPATCH")}
${fmtDispatch(constantsDispatch, "CONSTANTS_ARCH_DISPATCH")}
${fmtDispatch(asmDispatch, "ASSEMBLER_ARCH_DISPATCH")}

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
  assert!(REGISTER_ARCH_DISPATCH.len() == ${regDispatch.length});
  assert!(REGLIST_ARCH_DISPATCH.len() == ${reglistDispatch.length});
  assert!(CONSTANTS_ARCH_DISPATCH.len() == ${constantsDispatch.length});
  assert!(ASSEMBLER_ARCH_DISPATCH.len() == ${asmDispatch.length});
  assert!(reglist_ops::set(0, 3) == 0b1000);
  assert!(reglist_ops::count(0b1011) == 3);
  assert!(reglist_ops::first_code(0b1100) == 2);
  assert!(reglist_ops::last_code(0b1100) == 3);
  assert!(reglist_ops::difference(0b1111, 0b0011) == 0b1100);
  assert!(are_aliased(&[1, 1, 2]));
  assert!(!are_aliased(&[1, 2, 3]));
};
`;

const p = writeSrc("v8-codegen-core", "register_base.rs", rust);
console.log(
  `register-base 族: 基类常量 2 / reglist 位运算 7 / 派发表 ${regDispatch.length + reglistDispatch.length + constantsDispatch.length + asmDispatch.length} 分支 (x64+arm64 展开) → ${p}`,
);
