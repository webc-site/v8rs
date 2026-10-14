#!/usr/bin/env bun
// tool/src/codegen/register-configuration.h.js
// 机械提取 v8/src/codegen/register-configuration.{h,cc}: 架构无关上限 + 别名类别 + 访问器签名骨架
// → v8-codegen-core/src/register_config.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/register-configuration.h");
const ccSrc = readV8("src/codegen/register-configuration.cc");

const m = (re, what, src = hSrc) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

// 架构无关上限对数
const maxGeneral = m(/static constexpr int kMaxGeneralRegisters = (\d+);/, "kMaxGeneralRegisters");
const maxFP = m(/static constexpr int kMaxFPRegisters = (\d+);/, "kMaxFPRegisters");
const maxRegs = m(/static constexpr int kMaxRegisters =\s*std::max\(kMaxFPRegisters, kMaxGeneralRegisters\);/, "kMaxRegisters");
if (maxGeneral[1] !== "32" || maxFP[1] !== "32") throw new Error("寄存器上限异常");

// AliasingKind 定义于 common/globals.h: kCombine / kOverlap(/kIndependent)
const globalsSrc = readV8("src/common/globals.h");
const aliasing = m(/enum class AliasingKind \{[\s\S]*?\}/, "AliasingKind", globalsSrc);
const aliasVariants = [...aliasing[0].matchAll(/k(\w+)/g)].map((x) => x[0]);
if (!aliasVariants.includes("kCombine") || !aliasVariants.includes("kOverlap"))
  throw new Error("AliasingKind 变体异常");

// 访问器清点 (对数)
const accessors = [
  "num_general_registers", "num_float_registers", "num_double_registers",
  "num_simd128_registers", "num_simd256_registers",
  "num_allocatable_general_registers", "num_allocatable_float_registers",
  "num_allocatable_double_registers", "num_allocatable_simd128_registers",
  "num_allocatable_simd256_registers", "fp_aliasing_kind",
  "allocatable_general_codes_mask", "allocatable_double_codes_mask",
  "allocatable_float_codes_mask", "allocatable_simd128_codes_mask",
  "GetAllocatableGeneralCode", "IsAllocatableGeneralCode",
  "GetAllocatableDoubleCode", "IsAllocatableDoubleCode",
  "GetAllocatableSimd128Code", "IsAllocatableSimd128Code",
  "GetAliases", "AreAliases",
];
for (const a of accessors) {
  if (!hSrc.includes(a)) throw new Error(`缺访问器 ${a}`);
}

// .cc 关键方法
for (const want of ["Default", "RestrictGeneralRegisters", "GetAliases", "AreAliases"]) {
  if (!ccSrc.includes(`RegisterConfiguration::${want}(`)) throw new Error(`.cc 缺 ${want}`);
}

let rust = banner(
  "src/codegen/register-configuration.h.js",
  "src/codegen/register-configuration.{h,cc}",
  "RegisterConfiguration 上限/别名类别/访问器签名骨架",
);

rust += `
/// 架构无关上限: 通用寄存器 (原版 kMaxGeneralRegisters)
pub const K_MAX_GENERAL_REGISTERS: i32 = ${maxGeneral[1]};
/// 架构无关上限: FP 寄存器 (原版 kMaxFPRegisters)
pub const K_MAX_FP_REGISTERS: i32 = ${maxFP[1]};
/// 架构无关上限: 二者取大 (原版 kMaxRegisters)
pub const K_MAX_REGISTERS: i32 = if K_MAX_FP_REGISTERS > K_MAX_GENERAL_REGISTERS {
  K_MAX_FP_REGISTERS
} else {
  K_MAX_GENERAL_REGISTERS
};

/// 原版 enum class AliasingKind (common/globals.h): FP 寄存器别名策略
/// (判别值按原版声明顺序: ${aliasVariants.join(" → ")})
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AliasingKind {
${aliasVariants
  .map(
    (v, i) =>
      `  /// ${v === "kOverlap" ? "同尺寸寄存器互别名 (x64 Intel 布局)" : v === "kCombine" ? "别名到更小尺寸的一对寄存器 (ARM 布局)" : "SIMD128 与其他尺寸独立 (Riscv 布局)"}\n  ${v} = ${i},`,
  )
  .join("\n")}
}

/// 原版 RegisterConfiguration: 寄存器配置抽象基类。
/// 数值字段以原版私有字段名一一对应; 分配码数组以 owned Vec 承载。
pub struct RegisterConfiguration {
  pub num_general_registers_: i32,
  pub num_float_registers_: i32,
  pub num_double_registers_: i32,
  pub num_simd128_registers_: i32,
  pub num_simd256_registers_: i32,
  pub num_allocatable_general_registers_: i32,
  pub num_allocatable_float_registers_: i32,
  pub num_allocatable_double_registers_: i32,
  pub num_allocatable_simd128_registers_: i32,
  pub num_allocatable_simd256_registers_: i32,
  pub allocatable_general_codes_mask_: i32,
  pub allocatable_float_codes_mask_: i32,
  pub allocatable_double_codes_mask_: i32,
  pub allocatable_simd128_codes_mask_: i32,
  pub allocatable_simd256_codes_mask_: i32,
  pub allocatable_general_codes_: Vec<i32>,
  pub allocatable_float_codes_: Vec<i32>,
  pub allocatable_double_codes_: Vec<i32>,
  pub allocatable_simd128_codes_: Vec<i32>,
  pub allocatable_simd256_codes_: Vec<i32>,
  pub fp_aliasing_kind_: AliasingKind,
}

/// GetAllocatable*/IsAllocatable* 通用生成宏的机械展开 (5 组同构访问器)
macro_rules! allocatable_accessors {
  ($get:ident, $is:ident, $num:ident, $mask:ident, $codes:ident) => {
    pub fn $get(&self, index: usize) -> i32 {
      debug_assert!(index < self.$num() as usize);
      self.$codes[index]
    }
    #[inline]
    #[must_use]
    pub fn $is(&self, index: usize) -> bool {
      ((1i32 << index) & self.$mask) != 0
    }
  };
}

impl RegisterConfiguration {
${accessors
  .filter((a) => a.startsWith("num_") || a === "fp_aliasing_kind")
  .map(
    (a) => `  /// 原版 ${a}()
  #[inline]
  #[must_use]
  pub const fn ${a}(&self) -> ${a === "fp_aliasing_kind" ? "AliasingKind" : "i32"} {
    self.${a}_
  }`,
  )
  .join("\n\n")}

  allocatable_accessors!(
    get_allocatable_general_code,
    is_allocatable_general_code,
    num_allocatable_general_registers,
    allocatable_general_codes_mask_,
    allocatable_general_codes_
  );
  allocatable_accessors!(
    get_allocatable_double_code,
    is_allocatable_double_code,
    num_allocatable_double_registers,
    allocatable_double_codes_mask_,
    allocatable_double_codes_
  );
  allocatable_accessors!(
    get_allocatable_simd128_code,
    is_allocatable_simd128_code,
    num_allocatable_simd128_registers,
    allocatable_simd128_codes_mask_,
    allocatable_simd128_codes_
  );

  /// 原版 allocatable_general_codes()
  #[inline]
  #[must_use]
  pub fn allocatable_general_codes(&self) -> &[i32] {
    &self.allocatable_general_codes_
  }

  /// 原版 Default(): 目标架构默认配置 — 骨架
  pub fn default_configuration() -> &'static RegisterConfiguration {
    todo!("待 Developer 修订: 对齐 register-configuration.cc Default/各架构默认值")
  }

  /// 原版 Poisoning(): 带保留掩码寄存器的配置 — 骨架
  pub fn poisoning() -> &'static RegisterConfiguration {
    todo!("待 Developer 修订")
  }

  /// 原版 RestrictGeneralRegisters(reglist) — 骨架
  pub fn restrict_general_registers(_registers: u64) -> &'static RegisterConfiguration {
    todo!("待 Developer 修订")
  }

  /// 原版 GetAliases(rep, index, other_rep, &alias_base_index) — 骨架
  pub fn get_aliases(
    &self,
    _rep: u8,
    _index: i32,
    _other_rep: u8,
    _alias_base_index: &mut i32,
  ) -> i32 {
    todo!("待 Developer 修订: 对齐 register-configuration.cc GetAliases")
  }

  /// 原版 AreAliases(rep, index, other_rep, other_index) — 骨架
  #[must_use]
  pub fn are_aliases(&self, _rep: u8, _index: i32, _other_rep: u8, _other_index: i32) -> bool {
    todo!("待 Developer 修订")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_GENERAL_REGISTERS == 32 && K_MAX_FP_REGISTERS == 32);
  assert!(K_MAX_REGISTERS == 32);
};
`;

const p = writeSrc("v8-codegen-core", "register_config.rs", rust);
console.log(
  `register-configuration: 上限 3 / AliasingKind ${aliasVariants.length} / 访问器 ${accessors.length} → ${p}`,
);
