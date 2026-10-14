#!/usr/bin/env bun
// tool/src/codegen/arm64/reglist-arm64.h.js
// 机械提取 v8/src/codegen/arm64/reglist-arm64.h: CPURegList 有效位掩码 + AAPCS64 列表工厂
// → v8-codegen-arm64/src/reglist.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/codegen/arm64/reglist-arm64.h");

const m = (re, what) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

// 有效寄存器位掩码 (is_valid 私有常量, 机械展开)
const validRegs = m(/constexpr uint64_t kValidRegisters\{0x([0-9a-f]+)\};/, "kValidRegisters");
const validV = m(/constexpr uint64_t kValidVRegisters\{0x([0-9a-f]+)\};/, "kValidVRegisters");
const VALID_R = BigInt("0x" + validRegs[1]);
const VALID_V = BigInt("0x" + validV[1]);
// kValidRegisters = bit31(sp 内部码) | bit0..26; kValidVRegisters = bit0..27
if (VALID_R !== 0x8000000ffffffffn) throw new Error("kValidRegisters 值异常");
if (VALID_V !== 0x0000000ffffffffn) throw new Error("kValidVRegisters 值异常");

// 方法清点 (对数)
for (const f of ["Combine", "Remove", "Align", "PopLowestIndex", "PopHighestIndex",
  "GetCalleeSaved", "GetCalleeSavedD", "GetCalleeSavedV", "GetCallerSaved",
  "GetCallerSavedD", "GetCallerSavedV", "IsEmpty", "IncludesAliasOf", "Count",
  "RegisterSizeInBits", "RegisterSizeInBytes", "TotalSizeInBytes"]) {
  if (!src.includes(f)) throw new Error(`缺 CPURegList 成员 ${f}`);
}
// kRegListSizeInBits = sizeof(RegList) * 8 = 64
if (!src.includes("constexpr int kRegListSizeInBits = sizeof(RegList) * kBitsPerByte;"))
  throw new Error("缺 kRegListSizeInBits");

// 别名宏
for (const k of ["kCalleeSaved", "kCalleeSavedD", "kCalleeSavedV", "kCallerSaved", "kCallerSavedD", "kCallerSavedV"]) {
  if (!src.includes(`#define ${k} CPURegList::`)) throw new Error(`缺宏 ${k}`);
}

let rust = banner(
  "src/codegen/arm64/reglist-arm64.h.js",
  "src/codegen/arm64/reglist-arm64.h",
  "arm64 CPURegList 有效掩码 + AAPCS64 列表工厂签名骨架",
);

rust += `
use crate::registers::{CPURegister, CPURegisterType};

/// 原版 kRegListSizeInBits = sizeof(RegList) * kBitsPerByte (u64 载体)
pub const K_REG_LIST_SIZE_IN_BITS: i32 = 64;

/// 原版 is_valid() 私有掩码: GP 合法位 (bit31 = sp 内部码, bit0..26)
pub const K_VALID_REGISTERS: u64 = 0x${VALID_R.toString(16)};
/// 原版 is_valid() 私有掩码: V 合法位 (bit0..27)
pub const K_VALID_V_REGISTERS: u64 = 0x${VALID_V.toString(16)};

// 原版 enum CPURegister::RegisterType 的本地引用 (定义见 registers 模块)

/// 原版 class CPURegList: 同尺寸同类型寄存器位集
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CPURegList {
  /// 原版 list_
  list_: u64,
  /// 原版 size_
  size_: i32,
  /// 原版 type_ (RegisterType 编码: 0=GP,1=V,2=Z,3=none)
  type_: i8,
}

/// RegisterType 编码常量 (对应 CPURegister::RegisterType)
pub const TYPE_K_REGISTER: i8 = 0;
pub const TYPE_K_VREGISTER: i8 = 1;
pub const TYPE_K_ZREGISTER: i8 = 2;
pub const TYPE_K_NO_REGISTER: i8 = 3;

impl CPURegList {
  /// 原版构造 3: (type, size, first_reg, last_reg) — 机械直译区间位图
  #[must_use]
  pub fn from_range(type_: i8, size: i32, first_reg: i32, last_reg: i32) -> Self {
    let mut list = (1u64 << (last_reg + 1)) - 1;
    list &= !((1u64 << first_reg) - 1);
    Self { list_: list, size_: size, type_ }
  }

  /// 原版构造 2: (size, GP list)
  #[must_use]
  pub fn from_gp(size: i32, list: u64) -> Self {
    Self { list_: list, size_: size, type_: TYPE_K_REGISTER }
  }

  /// 原版构造 2': (size, V list)
  #[must_use]
  pub fn from_v(size: i32, list: u64) -> Self {
    Self { list_: list, size_: size, type_: TYPE_K_VREGISTER }
  }

  /// 原版 is_valid(): 掩码校验
  #[inline]
  #[must_use]
  pub const fn is_valid(&self) -> bool {
    match self.type_ {
      0 => (self.list_ & K_VALID_REGISTERS) == self.list_,
      1 => (self.list_ & K_VALID_V_REGISTERS) == self.list_,
      3 => self.list_ == 0,
      _ => false, // UNREACHABLE
    }
  }

  /// 原版 type()
  #[inline]
  #[must_use]
  pub const fn reg_type(&self) -> i8 {
    self.type_
  }

  /// 原版 bits()
  #[inline]
  #[must_use]
  pub const fn bits(&self) -> u64 {
    self.list_
  }

  /// 原版 set_bits(new_bits)
  #[inline]
  pub fn set_bits(&mut self, new_bits: u64) {
    self.list_ = new_bits;
  }

  /// 原版 IsEmpty
  #[inline]
  #[must_use]
  pub const fn is_empty(&self) -> bool {
    self.list_ == 0
  }

  /// 原版 Count: CountSetBits(list_, kRegListSizeInBits)
  #[inline]
  #[must_use]
  pub const fn count(&self) -> i32 {
    self.list_.count_ones() as i32
  }

  /// 原版 RegisterSizeInBits
  #[inline]
  #[must_use]
  pub const fn register_size_in_bits(&self) -> i32 {
    self.size_
  }

  /// 原版 RegisterSizeInBytes
  #[inline]
  #[must_use]
  pub const fn register_size_in_bytes(&self) -> i32 {
    self.size_ / 8
  }

  /// 原版 TotalSizeInBytes: RegisterSizeInBytes * Count
  #[inline]
  #[must_use]
  pub const fn total_size_in_bytes(&self) -> i32 {
    self.register_size_in_bytes() * self.count()
  }

  // —— 完整实现 ——

  /// 原版 Combine(other)
  #[inline]
  pub fn combine(&mut self, other: &Self) {
    debug_assert_eq!(self.type_, other.type_);
    self.list_ |= other.list_;
  }

  /// 原版 Remove(other)
  #[inline]
  pub fn remove(&mut self, other: &Self) {
    debug_assert_eq!(self.type_, other.type_);
    self.list_ &= !other.list_;
  }

  /// 原版 Align(): 16 字节对齐
  #[inline]
  pub fn align(&mut self) {
    const PAD_BIT: u64 = 1u64 << 31;
    if self.count() % 2 != 0 {
      if (self.list_ & PAD_BIT) != 0 {
        self.list_ &= !PAD_BIT;
      } else {
        self.list_ |= PAD_BIT;
      }
    }
    debug_assert_eq!(self.count() % 2, 0);
  }

  /// 原版 PopLowestIndex
  #[inline]
  pub fn pop_lowest_index(&mut self) -> u32 {
    if self.is_empty() {
      return u32::MAX;
    }
    let idx = self.list_.trailing_zeros();
    self.list_ &= self.list_ - 1;
    idx
  }

  /// 原版 PopHighestIndex
  #[inline]
  pub fn pop_highest_index(&mut self) -> u32 {
    if self.is_empty() {
      return u32::MAX;
    }
    let idx = 63 - self.list_.leading_zeros();
    self.list_ &= !(1u64 << idx);
    idx
  }

  /// 原版 PopLowestCpuReg
  #[inline]
  pub fn pop_lowest_cpu_reg(&mut self) -> Option<CPURegister> {
    if self.is_empty() {
      return None;
    }
    let idx = self.pop_lowest_index() as i8;
    let reg_type = match self.type_ {
      TYPE_K_REGISTER => CPURegisterType::Register,
      TYPE_K_VREGISTER => CPURegisterType::VRegister,
      _ => CPURegisterType::NoRegister,
    };
    Some(CPURegister::new(idx, self.size_ as u8, reg_type))
  }

  /// 原版 PopHighestCpuReg
  #[inline]
  pub fn pop_highest_cpu_reg(&mut self) -> Option<CPURegister> {
    if self.is_empty() {
      return None;
    }
    let idx = self.pop_highest_index() as i8;
    let reg_type = match self.type_ {
      TYPE_K_REGISTER => CPURegisterType::Register,
      TYPE_K_VREGISTER => CPURegisterType::VRegister,
      _ => CPURegisterType::NoRegister,
    };
    Some(CPURegister::new(idx, self.size_ as u8, reg_type))
  }

  /// 原版 IncludesAliasOf (多变参形态此处收位集)
  #[inline]
  #[must_use]
  pub const fn includes_alias_of_bits(&self, list: u64) -> bool {
    (self.list_ & list) != 0
  }

  /// 原版 IncludesAliasOf (单参 CPURegister 形式)
  #[inline]
  #[must_use]
  pub fn includes_alias_of(&self, reg: impl Into<CPURegister>) -> bool {
    let reg: CPURegister = reg.into();
    if !reg.is_valid() {
      return false;
    }
    let code = reg.code();
    if !(0..64).contains(&code) {
      return false;
    }
    let expected_type = match self.type_ {
      TYPE_K_REGISTER => CPURegisterType::Register,
      TYPE_K_VREGISTER => CPURegisterType::VRegister,
      _ => CPURegisterType::NoRegister,
    };
    if reg.reg_type() != expected_type {
      return false;
    }
    (self.list_ & (1u64 << (code as u32))) != 0
  }

  /// 原版 GetCalleeSaved(): AAPCS64 被调方保存 GP (x19..x28)
  #[must_use]
  pub const fn get_callee_saved() -> Self {
    let mut bits = 0u64;
    let mut i = 19;
    while i <= 28 {
      bits |= 1u64 << i;
      i += 1;
    }
    Self { list_: bits, size_: 64, type_: TYPE_K_REGISTER }
  }

  /// 原版 GetCalleeSavedD(): AAPCS64 被调方保存浮点 (d8..d15)
  #[must_use]
  pub const fn get_callee_saved_d() -> Self {
    let mut bits = 0u64;
    let mut i = 8;
    while i <= 15 {
      bits |= 1u64 << i;
      i += 1;
    }
    Self { list_: bits, size_: 64, type_: TYPE_K_VREGISTER }
  }

  /// 原版 GetCalleeSavedV(): AAPCS64 仅要求保存 d8..d15, V 寄存器为 0
  #[must_use]
  pub const fn get_callee_saved_v() -> Self {
    Self { list_: 0, size_: 128, type_: TYPE_K_VREGISTER }
  }

  /// 原版 GetCallerSaved(): x0..x17 (assembler-arm64.cc:248-252, 不含 lr 与 x18)
  #[must_use]
  pub const fn get_caller_saved() -> Self {
    let bits = (1u64 << 18) - 1; // 0..17
    Self { list_: bits, size_: 64, type_: TYPE_K_REGISTER }
  }

  /// 原版 GetCallerSavedD(): d0..d7, d16..d31
  #[must_use]
  pub const fn get_caller_saved_d() -> Self {
    let low = (1u64 << 8) - 1; // 0..7
    let high = ((1u64 << 32) - 1) & !((1u64 << 16) - 1); // 16..31
    Self { list_: low | high, size_: 64, type_: TYPE_K_VREGISTER }
  }

  /// 原版 GetCallerSavedV(): v0..v31 全体
  #[must_use]
  pub const fn get_caller_saved_v() -> Self {
    let bits = (1u64 << 32) - 1;
    Self { list_: bits, size_: 128, type_: TYPE_K_VREGISTER }
  }
}

// 原版宏别名: kCalleeSaved/kCalleeSavedD/kCalleeSavedV/kCallerSaved/
// kCallerSavedD/kCallerSavedV = CPURegList::Get*Saved*() 静态工厂 (以函数调用表达)

/// 原版 using RegList/DoubleRegList/Simd128RegList (基类见 v8-codegen-core::register_base)
pub type RegList = u64;
pub type DoubleRegList = u64;
pub type Simd128RegList = u64;

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_VALID_REGISTERS == 0x8000000ffffffff);
  assert!(K_VALID_V_REGISTERS == 0x0000000ffffffff);
  assert!(K_REG_LIST_SIZE_IN_BITS == 64);
};
`;

const p = writeSrc("v8-codegen-arm64", "reglist.rs", rust);
console.log(`reglist-arm64: 掩码 2 / 方法 17 / 宏别名 6 → ${p}`);
