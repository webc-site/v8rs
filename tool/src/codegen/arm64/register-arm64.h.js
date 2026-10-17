#!/usr/bin/env bun
// tool/src/codegen/arm64/register-arm64.h.js — 从 v8/src/codegen/arm64/register-arm64.h
// 提取 ARM64 通用寄存器 (x0..x30, sp, xzr, w0..w30, wsp, wzr)、
// VRegister (b, h, s, d, q, v 0..31)、调用约定别名、对数断言，
// 并生成 v8-codegen-arm64/src/registers.rs。
import { readV8, writeSrc, banner } from "../../../lib.js";

const CRATE = "v8-codegen-arm64";
const GEN = "src/codegen/arm64/register-arm64.h.js";
const SRC_REL = "src/codegen/arm64/register-arm64.h";
const H = readV8(SRC_REL);

function must(cond, msg) {
  if (!cond) throw new Error(`register-arm64.h.js 对数核对失败: ${msg}`);
}

// ── 结构锚核对 ──
must(H.includes("#ifndef V8_CODEGEN_ARM64_REGISTER_ARM64_H_"), "头文件卫哨缺失");
must(H.includes("#define GENERAL_REGISTER_CODE_LIST(R)"), "GENERAL_REGISTER_CODE_LIST 宏缺失");
must(H.includes("#define GENERAL_REGISTERS(R)"), "GENERAL_REGISTERS 宏缺失");
must(H.includes("#define FLOAT_REGISTERS(V)"), "FLOAT_REGISTERS 宏缺失");
must(H.includes("#define DOUBLE_REGISTERS(R)"), "DOUBLE_REGISTERS 宏缺失");
must(H.includes("#define SIMD128_REGISTERS(V)"), "SIMD128_REGISTERS 宏缺失");
must(H.includes("#define VECTOR_REGISTERS(V)"), "VECTOR_REGISTERS 宏缺失");
must(H.includes("#define ALWAYS_ALLOCATABLE_GENERAL_REGISTERS(R)"), "ALWAYS_ALLOCATABLE 宏缺失");
must(H.includes("#define MAYBE_ALLOCATABLE_GENERAL_REGISTERS(R)"), "MAYBE_ALLOCATABLE 宏缺失");
must(H.includes("#define ALLOCATABLE_DOUBLE_REGISTERS(R)"), "ALLOCATABLE_DOUBLE 宏缺失");
must(H.includes("#define C_CALL_CALLEE_SAVE_REGISTERS"), "C_CALL_CALLEE_SAVE 宏缺失");
must(H.includes("#define C_CALL_CALLEE_SAVE_FP_REGISTERS"), "C_CALL_CALLEE_SAVE_FP 宏缺失");

// ── 静态断言锚 ──
must(H.includes("ASSERT_TRIVIALLY_COPYABLE(CPURegister)"), "CPURegister trivially copyable 缺失");
must(H.includes("ASSERT_TRIVIALLY_COPYABLE(Register)"), "Register trivially copyable 缺失");
must(H.includes("ASSERT_TRIVIALLY_COPYABLE(VRegister)"), "VRegister trivially copyable 缺失");

// ── 宏列表解析 ──
function vLists(name) {
  const lines = H.split("\n");
  const head = new RegExp(`^#define ${name}\\(.*?\\)(.*)$`);
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(head);
    if (!m) continue;
    let body = m[1];
    let j = i;
    while (lines[j].trimEnd().endsWith("\\")) {
      j++;
      body += "\n" + lines[j];
    }
    return [...body.matchAll(/\b[RVZ]\s*\(\s*(\w+)\s*\)/g)].map((x) => x[1]);
  }
  throw new Error(`宏列表 ${name} 未找到`);
}

function expect(actual, golden, label) {
  must(
    actual.length === golden.length && actual.every((v, i) => v === golden[i]),
    `${label} 序列漂移: [${actual.join(",")}] != [${golden.join(",")}]`,
  );
}

const GENERAL_CODES = vLists("GENERAL_REGISTER_CODE_LIST");
expect(GENERAL_CODES, Array.from({ length: 32 }, (_, i) => `${i}`), "GENERAL_REGISTER_CODE_LIST");

const GENERAL = vLists("GENERAL_REGISTERS");
expect(GENERAL, Array.from({ length: 32 }, (_, i) => `x${i}`), "GENERAL_REGISTERS");

const FLOATS = vLists("FLOAT_REGISTERS");
expect(FLOATS, Array.from({ length: 32 }, (_, i) => `s${i}`), "FLOAT_REGISTERS");

const DOUBLES = vLists("DOUBLE_REGISTERS");
expect(DOUBLES, Array.from({ length: 32 }, (_, i) => `d${i}`), "DOUBLE_REGISTERS");

const SIMD128 = vLists("SIMD128_REGISTERS");
expect(SIMD128, Array.from({ length: 32 }, (_, i) => `q${i}`), "SIMD128_REGISTERS");

const VECTORS = vLists("VECTOR_REGISTERS");
expect(VECTORS, Array.from({ length: 32 }, (_, i) => `v${i}`), "VECTOR_REGISTERS");

const ALWAYS_ALLOC = vLists("ALWAYS_ALLOCATABLE_GENERAL_REGISTERS");
must(ALWAYS_ALLOC.length === 24, `ALWAYS_ALLOCATABLE 数量 ${ALWAYS_ALLOC.length} != 24`);

const ALLOC_D = vLists("ALLOCATABLE_DOUBLE_REGISTERS");
must(ALLOC_D.length === 28, `ALLOCATABLE_DOUBLE 数量 ${ALLOC_D.length} != 28`);

const K_ZERO_REG_CODE = 31;

// ── C 调用约定参数寄存器 ──
const cArgsMatch = H.match(/constexpr Register kCArgRegs\[\] = \{([^}]+)\};/);
must(cArgsMatch, "kCArgRegs 声明缺失");
const cArgs = cArgsMatch[1].split(",").map((s) => s.trim());
expect(cArgs, ["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"], "kCArgRegs");

// ── 名字表锚 (register-base.h DEFINE_REGISTER_NAMES: 宏参数字符串化) ──
must(H.includes("DEFINE_REGISTER_NAMES(Register, GENERAL_REGISTERS)"), "DEFINE_REGISTER_NAMES(Register) 锚缺失");
must(H.includes("DEFINE_REGISTER_NAMES(VRegister, VECTOR_REGISTERS)"), "DEFINE_REGISTER_NAMES(VRegister) 锚缺失");
const REGISTER_NAMES = GENERAL; // ["x0".."x31"], 与 GENERAL_REGISTERS 串化 1:1
must(REGISTER_NAMES.length === 32 && REGISTER_NAMES[0] === "x0" && REGISTER_NAMES[31] === "x31", "REGISTER_NAMES 名表漂移");

// ── GetSpecialRegisterName 函数体 1:1 锚 (register-arm64.h:279-281) ──
must(
  /static const char\* GetSpecialRegisterName\(int code\) \{\s*return \(code == kSPRegInternalCode\) \? "sp" : "UNKNOWN";/.test(H),
  "GetSpecialRegisterName 函数体漂移",
);

// ── ALIAS_REGISTER 别名链 golden (register-arm64.h:563-598) ──
const aliasLines = [...H.matchAll(/^ALIAS_REGISTER\((Register|VRegister), (\w+), (\w+)\);/gm)].map((m) => ({
  cls: m[1],
  alias: m[2],
  target: m[3],
}));
const GOLD_ALIAS = [
  ["Register", "kStackPointerRegister", "sp"],
  ["VRegister", "v8_", "v8"],
  ["Register", "ip0", "x16"],
  ["Register", "ip1", "x17"],
  ["Register", "wip0", "w16"],
  ["Register", "wip1", "w17"],
  ["Register", "kRootRegister", "x26"],
  ["Register", "rr", "x26"],
  ["Register", "kPtrComprCageBaseRegister", "x28"],
  ["Register", "kPtrComprCageBaseRegister", "no_reg"],
  ["Register", "cp", "x27"],
  ["Register", "fp", "x29"],
  ["Register", "lr", "x30"],
  ["Register", "xzr", "x31"],
  ["Register", "wzr", "w31"],
  ["Register", "padreg", "x31"],
  ["VRegister", "fp_zero", "d15"],
  ["VRegister", "fp_fixed1", "d27"],
  ["VRegister", "fp_fixed2", "d28"],
  ["VRegister", "fp_scratch", "d30"],
  ["VRegister", "fp_scratch1", "d30"],
  ["VRegister", "fp_scratch2", "d31"],
];
must(aliasLines.length === GOLD_ALIAS.length, `ALIAS_REGISTER 对数 ${aliasLines.length} != ${GOLD_ALIAS.length}`);
aliasLines.forEach((a, i) => {
  const g = GOLD_ALIAS[i];
  must(
    a.cls === g[0] && a.alias === g[1] && a.target === g[2],
    `ALIAS 漂移 #${i}: ${a.cls} ${a.alias}=${a.target} != ${g[0]} ${g[1]}=${g[2]}`,
  );
});

// ── 调用约定 constexpr Register 别名 golden (register-arm64.h:663-688, 23 条) ──
const ccBlockMatch = H.match(/constexpr Register kReturnRegister0[\s\S]*?constexpr Register kSimulatorHltArgument = \w+;/);
must(ccBlockMatch, "调用约定别名区缺失");
const CC = [...ccBlockMatch[0].matchAll(/constexpr Register (\w+) = (\w+);/g)].map((m) => ({ name: m[1], target: m[2] }));
const GOLD_CC = [
  ["kReturnRegister0", "x0"],
  ["kReturnRegister1", "x1"],
  ["kReturnRegister2", "x2"],
  ["kJSFunctionRegister", "x1"],
  ["kContextRegister", "cp"],
  ["kAllocateSizeRegister", "x1"],
  ["kInterpreterAccumulatorRegister", "x0"],
  ["kInterpreterBytecodeOffsetRegister", "x19"],
  ["kInterpreterBytecodeArrayRegister", "x20"],
  ["kInterpreterDispatchTableRegister", "x21"],
  ["kJavaScriptCallArgCountRegister", "x0"],
  ["kJavaScriptCallCodeStartRegister", "x2"],
  ["kJavaScriptCallTargetRegister", "kJSFunctionRegister"],
  ["kJavaScriptCallNewTargetRegister", "x3"],
  ["kJavaScriptCallExtraArg1Register", "x2"],
  ["kJavaScriptCallDispatchHandleRegister", "x4"],
  ["kRuntimeCallFunctionRegister", "x1"],
  ["kRuntimeCallArgCountRegister", "x0"],
  ["kRuntimeCallArgvRegister", "x11"],
  ["kWasmImplicitArgRegister", "x7"],
  ["kWasmCompileLazyFuncIndexRegister", "x8"],
  ["kWasmTrapHandlerFaultAddressRegister", "x16"],
  ["kSimulatorHltArgument", "x16"],
];
must(CC.length === GOLD_CC.length, `调用约定别名对数 ${CC.length} != ${GOLD_CC.length}`);
CC.forEach((c, i) => {
  must(c.name === GOLD_CC[i][0] && c.target === GOLD_CC[i][1], `调用约定别名漂移 #${i}: ${c.name}=${c.target}`);
});
must(/constexpr DoubleRegister kFPReturnRegister0 = d0;/.test(H), "kFPReturnRegister0 锚缺失");

// ── kFoo → K_FOO (与 v8-common globals.rs 规约一致; 缩略词 JSFunction→JS_FUNCTION) ──
function rustUpper(name) {
  if (name.length > 1 && name[0] === "k" && name[1] === name[1].toUpperCase() && /[A-Za-z]/.test(name[1])) {
    name = "K_" + name.slice(1);
  }
  let s = name.replace(/([a-z0-9])([A-Z])/g, "$1_$2");
  s = s.replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2");
  return s.toUpperCase();
}

// C++ 寄存器名 → 本 crate Rust 常量表达式
const ALIAS_RUST = {
  sp: "SP",
  wsp: "WSP",
  no_reg: "NO_REG",
  cp: "CP",
  fp: "FP",
  lr: "LR",
  rr: "RR",
  padreg: "PADREG",
};
function rustRegExpr(name) {
  const m = name.match(/^[xwdqsbhv](\d+)$/);
  if (m) return name.toUpperCase(); // x16→X16, d15→D15, v8→V8
  if (ALIAS_RUST[name]) return ALIAS_RUST[name];
  if (name.startsWith("k")) return rustUpper(name); // kJSFunctionRegister→K_JS_FUNCTION_REGISTER
  throw new Error(`register-arm64.h.js: 未映射的寄存器名 ${name}`);
}

// ── 生成 registers.rs ──
let out = banner(GEN, SRC_REL, "ARM64 架构寄存器与条件码定义");

out += `// 注: 原版 C++ 侧寄存器/条件码常量为小写 (x0/d0/eq/...), Rust 侧统一以大写常量与
// Condition 枚举变体承接; 小写别名已按 non_upper_case_globals 清障移除。

use core::mem::size_of;

use crate::constants::{
  K_B_REG_SIZE_IN_BITS, K_D_REG_SIZE_IN_BITS, K_H_REG_SIZE_IN_BITS,
  K_NUMBER_OF_REGISTERS, K_NUMBER_OF_V_REGISTERS, K_Q_REG_SIZE_IN_BITS, K_SP_REG_INTERNAL_CODE,
  K_S_REG_SIZE_IN_BITS, K_W_REG_SIZE_IN_BITS, K_X_REG_SIZE_IN_BITS, K_ZERO_REG_CODE,
};

/// 对应原版 RegisterBase::kCode_no_reg = -1
pub const K_CODE_NO_REG: i8 = -1;

/// 寄存器类别 (对应原版 CPURegister::RegisterType, register-arm64.h:112 声明序 kRegister,kVRegister,kZRegister,kNoRegister = 0,1,2,3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum CPURegisterType {
  Register = 0,
  VRegister = 1,
  ZRegister = 2,
  #[default]
  NoRegister = 3,
}

/// 通用/SIMD/FP 统一寄存器基础类型 (对应原版 v8::internal::CPURegister)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CPURegister {
  code: i8,
  size_in_bits: u8,
  reg_type: CPURegisterType,
}

impl CPURegister {
  pub const fn no_reg() -> Self {
    Self {
      code: K_CODE_NO_REG,
      size_in_bits: 0,
      reg_type: CPURegisterType::NoRegister,
    }
  }

  pub const fn new(code: i8, size_in_bits: u8, reg_type: CPURegisterType) -> Self {
    Self {
      code,
      size_in_bits,
      reg_type,
    }
  }

  #[inline]
  pub const fn is_valid(self) -> bool {
    self.code != K_CODE_NO_REG && self.reg_type as i8 != CPURegisterType::NoRegister as i8
  }

  #[inline]
  pub const fn is_none(self) -> bool {
    !self.is_valid()
  }

  #[inline]
  pub const fn code(self) -> i8 {
    self.code
  }

  #[inline]
  pub const fn size_in_bits(self) -> usize {
    self.size_in_bits as usize
  }

  #[inline]
  pub const fn size_in_bytes(self) -> usize {
    (self.size_in_bits as usize) / 8
  }

  #[inline]
  pub const fn reg_type(self) -> CPURegisterType {
    self.reg_type
  }

  #[inline]
  pub const fn is_register(self) -> bool {
    matches!(self.reg_type, CPURegisterType::Register)
  }

  #[inline]
  pub const fn is_v_register(self) -> bool {
    matches!(self.reg_type, CPURegisterType::VRegister)
  }

  #[inline]
  pub const fn is_64_bits(self) -> bool {
    self.size_in_bits == 64
  }

  #[inline]
  pub const fn is_32_bits(self) -> bool {
    self.size_in_bits == 32
  }

  #[inline]
  pub const fn is_sp(self) -> bool {
    self.is_register() && self.code == K_SP_REG_INTERNAL_CODE as i8
  }

  #[inline]
  pub const fn is_zero(self) -> bool {
    self.is_register() && self.code == K_ZERO_REG_CODE as i8
  }
}

impl Default for CPURegister {
  fn default() -> Self {
    Self::no_reg()
  }
}

/// 通用寄存器 (区分 64 位 X 与 32 位 W，对应原版 v8::internal::Register)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register {
  code: i8,
  size_in_bits: u8,
}

impl Register {
  pub const fn no_reg() -> Self {
    Self {
      code: K_CODE_NO_REG,
      size_in_bits: 0,
    }
  }

  pub const fn from_code(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_X_REG_SIZE_IN_BITS as u8,
    }
  }

  pub const fn from_code_and_size(code: i8, size_in_bits: u8) -> Self {
    Self { code, size_in_bits }
  }

  pub const fn x(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_X_REG_SIZE_IN_BITS as u8,
    }
  }

  pub const fn w(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_W_REG_SIZE_IN_BITS as u8,
    }
  }

  pub const fn sp() -> Self {
    Self {
      code: K_SP_REG_INTERNAL_CODE as i8,
      size_in_bits: K_X_REG_SIZE_IN_BITS as u8,
    }
  }

  pub const fn wsp() -> Self {
    Self {
      code: K_SP_REG_INTERNAL_CODE as i8,
      size_in_bits: K_W_REG_SIZE_IN_BITS as u8,
    }
  }

  #[inline]
  pub const fn is_valid(self) -> bool {
    self.code != K_CODE_NO_REG
  }

  #[inline]
  pub const fn code(self) -> i8 {
    self.code
  }

  #[inline]
  pub const fn size_in_bits(self) -> usize {
    self.size_in_bits as usize
  }

  #[inline]
  pub const fn size_in_bytes(self) -> usize {
    (self.size_in_bits as usize) / 8
  }

  #[inline]
  pub const fn is_64_bits(self) -> bool {
    self.size_in_bits == K_X_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_32_bits(self) -> bool {
    self.size_in_bits == K_W_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_sp(self) -> bool {
    self.code == K_SP_REG_INTERNAL_CODE as i8
  }

  #[inline]
  pub const fn is_zero(self) -> bool {
    self.code == K_ZERO_REG_CODE as i8
  }

  #[inline]
  pub const fn to_x(self) -> Self {
    Self {
      code: self.code,
      size_in_bits: K_X_REG_SIZE_IN_BITS as u8,
    }
  }

  #[inline]
  pub const fn to_w(self) -> Self {
    Self {
      code: self.code,
      size_in_bits: K_W_REG_SIZE_IN_BITS as u8,
    }
  }

  #[inline]
  pub const fn as_cpu_register(self) -> CPURegister {
    CPURegister::new(self.code, self.size_in_bits, CPURegisterType::Register)
  }
}

impl Default for Register {
  fn default() -> Self {
    Self::no_reg()
  }
}

/// NEON/FP 向量寄存器 (对应原版 v8::internal::VRegister)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VRegister {
  code: i8,
  size_in_bits: u8,
  lane_count: u8,
}

impl VRegister {
  pub const fn no_reg() -> Self {
    Self {
      code: K_CODE_NO_REG,
      size_in_bits: 0,
      lane_count: 0,
    }
  }

  pub const fn from_code(code: i8) -> Self {
    // 默认 D 寄存器 (64 位，1 车道)
    Self {
      code,
      size_in_bits: K_D_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn b(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_B_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn h(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_H_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn s(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_S_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn d(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_D_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn q(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_Q_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn v(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_Q_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  #[inline]
  pub const fn is_valid(self) -> bool {
    self.code != K_CODE_NO_REG
  }

  #[inline]
  pub const fn code(self) -> i8 {
    self.code
  }

  #[inline]
  pub const fn size_in_bits(self) -> usize {
    self.size_in_bits as usize
  }

  #[inline]
  pub const fn size_in_bytes(self) -> usize {
    (self.size_in_bits as usize) / 8
  }

  #[inline]
  pub const fn lane_count(self) -> usize {
    self.lane_count as usize
  }

  #[inline]
  pub const fn is_b(self) -> bool {
    self.size_in_bits == K_B_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_h(self) -> bool {
    self.size_in_bits == K_H_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_s(self) -> bool {
    self.size_in_bits == K_S_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_d(self) -> bool {
    self.size_in_bits == K_D_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_q(self) -> bool {
    self.size_in_bits == K_Q_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn as_cpu_register(self) -> CPURegister {
    CPURegister::new(self.code, self.size_in_bits, CPURegisterType::VRegister)
  }
}

impl Default for VRegister {
  fn default() -> Self {
    Self::no_reg()
  }
}

pub type FloatRegister = VRegister;
pub type DoubleRegister = VRegister;
pub type Simd128Register = VRegister;

/// 条件码枚举 (对应原版 v8::internal::Condition)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Condition {
  Eq = 0,
  Ne = 1,
  Cs = 2,
  Cc = 3,
  Mi = 4,
  Pl = 5,
  Vs = 6,
  Vc = 7,
  Hi = 8,
  Ls = 9,
  Ge = 10,
  Lt = 11,
  Gt = 12,
  Le = 13,
  Al = 14,
  Nv = 15,
}

impl Condition {
  #[inline]
  pub const fn code(self) -> u32 {
    self as u32
  }

  #[inline]
  pub const fn negate(self) -> Self {
    match (self as u8) ^ 1 {
      0 => Self::Eq,
      1 => Self::Ne,
      2 => Self::Cs,
      3 => Self::Cc,
      4 => Self::Mi,
      5 => Self::Pl,
      6 => Self::Vs,
      7 => Self::Vc,
      8 => Self::Hi,
      9 => Self::Ls,
      10 => Self::Ge,
      11 => Self::Lt,
      12 => Self::Gt,
      13 => Self::Le,
      14 => Self::Al,
      _ => Self::Nv,
    }
  }
}

impl From<Register> for CPURegister {
  #[inline]
  fn from(reg: Register) -> Self {
    reg.as_cpu_register()
  }
}

impl From<VRegister> for CPURegister {
  #[inline]
  fn from(reg: VRegister) -> Self {
    reg.as_cpu_register()
  }
}

/// 寄存器列表集合 (对应原版 v8::internal::CPURegList)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CPURegList {
  bits: u64,
  size_in_bits: u8,
  reg_type: CPURegisterType,
}

impl CPURegList {
  pub const fn empty(size_in_bits: u8, reg_type: CPURegisterType) -> Self {
    Self {
      bits: 0,
      size_in_bits,
      reg_type,
    }
  }

  pub const fn from_bits(bits: u64, size_in_bits: u8, reg_type: CPURegisterType) -> Self {
    Self {
      bits,
      size_in_bits,
      reg_type,
    }
  }

  pub fn from_slice(regs: &[Register]) -> Self {
    let mut list = Self::empty(64, CPURegisterType::Register);
    if let Some(first) = regs.first() {
      list.size_in_bits = first.size_in_bits;
    }
    for &r in regs {
      list.set(r);
    }
    list
  }

  pub fn from_vslice(regs: &[VRegister]) -> Self {
    let mut list = Self::empty(64, CPURegisterType::VRegister);
    if let Some(first) = regs.first() {
      list.size_in_bits = first.size_in_bits;
    }
    for &r in regs {
      list.set_v(r);
    }
    list
  }

  #[inline]
  pub const fn bits(self) -> u64 {
    self.bits
  }

  #[inline]
  pub const fn size_in_bits(self) -> usize {
    self.size_in_bits as usize
  }

  #[inline]
  pub const fn reg_type(self) -> CPURegisterType {
    self.reg_type
  }

  #[inline]
  pub const fn is_empty(self) -> bool {
    self.bits == 0
  }

  #[inline]
  pub const fn count(self) -> usize {
    self.bits.count_ones() as usize
  }

  #[inline]
  pub const fn total_size_in_bytes(self) -> usize {
    (self.size_in_bits as usize / 8) * self.count()
  }

  #[inline]
  pub fn set(&mut self, reg: Register) {
    if reg.is_valid() && reg.code() >= 0 && reg.code() < 32 {
      self.bits |= 1u64 << (reg.code() as u32);
    }
  }

  #[inline]
  pub fn clear(&mut self, reg: Register) {
    if reg.is_valid() && reg.code() >= 0 && reg.code() < 32 {
      self.bits &= !(1u64 << (reg.code() as u32));
    }
  }

  #[inline]
  pub const fn has(self, reg: Register) -> bool {
    if !reg.is_valid() || reg.code() < 0 || reg.code() >= 32 {
      false
    } else {
      (self.bits & (1u64 << (reg.code() as u32))) != 0
    }
  }

  #[inline]
  pub fn set_v(&mut self, reg: VRegister) {
    if reg.is_valid() && reg.code() >= 0 && reg.code() < 32 {
      self.bits |= 1u64 << (reg.code() as u32);
    }
  }

  #[inline]
  pub fn clear_v(&mut self, reg: VRegister) {
    if reg.is_valid() && reg.code() >= 0 && reg.code() < 32 {
      self.bits &= !(1u64 << (reg.code() as u32));
    }
  }

  #[inline]
  pub const fn has_v(self, reg: VRegister) -> bool {
    if !reg.is_valid() || reg.code() < 0 || reg.code() >= 32 {
      false
    } else {
      (self.bits & (1u64 << (reg.code() as u32))) != 0
    }
  }

  #[inline]
  pub fn combine(&mut self, other: Self) {
    self.bits |= other.bits;
  }

  #[inline]
  pub fn remove(&mut self, other: Self) {
    self.bits &= !other.bits;
  }

  pub fn pop_lowest_index(&mut self) -> Option<Register> {
    if self.bits == 0 {
      return None;
    }
    let idx = self.bits.trailing_zeros() as i8;
    self.bits &= self.bits - 1;
    Some(Register::from_code_and_size(idx, self.size_in_bits))
  }

  pub fn pop_highest_index(&mut self) -> Option<Register> {
    if self.bits == 0 {
      return None;
    }
    let idx = 63 - self.bits.leading_zeros() as i8;
    self.bits &= !(1u64 << idx);
    Some(Register::from_code_and_size(idx, self.size_in_bits))
  }

  pub fn pop_lowest_cpu_reg(&mut self) -> Option<CPURegister> {
    if self.bits == 0 {
      return None;
    }
    let idx = self.bits.trailing_zeros() as i8;
    self.bits &= self.bits - 1;
    Some(CPURegister::new(idx, self.size_in_bits, self.reg_type))
  }

  pub fn pop_highest_cpu_reg(&mut self) -> Option<CPURegister> {
    if self.bits == 0 {
      return None;
    }
    let idx = 63 - self.bits.leading_zeros() as i8;
    self.bits &= !(1u64 << idx);
    Some(CPURegister::new(idx, self.size_in_bits, self.reg_type))
  }

  /// AAPCS64 被调用者保存寄存器 (x19..x28)
  pub const fn get_callee_saved() -> Self {
    let mut bits = 0u64;
    let mut i = 19;
    while i <= 28 {
      bits |= 1u64 << i;
      i += 1;
    }
    Self::from_bits(bits, 64, CPURegisterType::Register)
  }

  /// AAPCS64 被调用者保存浮点寄存器 (d8..d15)
  pub const fn get_callee_saved_d() -> Self {
    let mut bits = 0u64;
    let mut i = 8;
    while i <= 15 {
      bits |= 1u64 << i;
      i += 1;
    }
    Self::from_bits(bits, 64, CPURegisterType::VRegister)
  }

  /// 原版 GetCallerSaved: 通用寄存器 x0..x17 (assembler-arm64.cc:248-252)。
  /// x18 为平台寄存器保留 (cc:249-250 注), lr/x30 不在实现列表——头注释
  /// reglist-arm64.h:108 "includes lr" 系过时注释, 以 .cc 实现为准
  pub const fn get_caller_saved() -> Self {
    let bits = (1u64 << 18) - 1; // 0..17
    Self::from_bits(bits, 64, CPURegisterType::Register)
  }

  /// AAPCS64 调用者保存浮点寄存器 (d0..d7, d16..d31)
  pub const fn get_caller_saved_d() -> Self {
    let low = (1u64 << 8) - 1; // 0..7
    let high = ((1u64 << 32) - 1) & !((1u64 << 16) - 1); // 16..31
    Self::from_bits(low | high, 64, CPURegisterType::VRegister)
  }

  /// 原版 GetCalleeSavedV: **空集** (assembler-arm64.cc:244-246——AAPCS64 仅要求
  /// 保存 v8-v15 的低 64 位即 d8-d15, 整个 Q 寄存器无须被调保存), size=kQRegSizeInBits=128
  pub const fn get_callee_saved_v() -> Self {
    Self::from_bits(0, 128, CPURegisterType::VRegister)
  }

  /// 原版 GetCallerSavedV: v0..v31 全体 (assembler-arm64.cc:262-268,
  /// kQRegSizeInBits=128, CPURegister::kVRegister, 0, 31)
  pub const fn get_caller_saved_v() -> Self {
    let bits = (1u64 << 32) - 1; // 0..31
    Self::from_bits(bits, 128, CPURegisterType::VRegister)
  }

  /// V8 原版无对应函数 (x0..x7 系 AAPCS64 参数寄存器惯例), 归 macro-assembler 票收编
  pub const fn get_argument_registers() -> Self {
    let bits = (1u64 << 8) - 1; // 0..7
    Self::from_bits(bits, 64, CPURegisterType::Register)
  }

  /// V8 原版无对应函数 (d0..d7 系 AAPCS64 浮点参数寄存器惯例), 归 macro-assembler 票收编
  pub const fn get_argument_registers_d() -> Self {
    let bits = (1u64 << 8) - 1; // 0..7
    Self::from_bits(bits, 64, CPURegisterType::VRegister)
  }

  /// 原版 IncludesAliasOf 变参模板收窄为单参 (reglist-arm64.h:119-133):
  /// 各参先隐式转 CPURegister (Rust 侧以 impl Into 承接 std::is_convertible_v 约束),
  /// 有效且 type()==type_ 才置位, 返回 (list_ & list) != 0。跨类型恒 false
  pub fn includes_alias_of(&self, reg: impl Into<CPURegister>) -> bool {
    let reg: CPURegister = reg.into();
    if !reg.is_valid() || reg.reg_type() != self.reg_type {
      return false;
    }
    let code = reg.code();
    code >= 0 && (code as u64) < 64 && ((self.bits >> (code as u64)) & 1) == 1
  }

  /// 原版 CPURegList::Align (assembler-arm64.cc:221-232): 栈对齐维护——奇数个时
  /// 翻转 padreg: 已含则 Remove, 否则 Combine; padreg=x31 (register-arm64.h:586,
  /// Rust 侧 xzr 与 x31 同 code)。偶数不动, 收尾承接 DCHECK_EQ(Count()%2, 0) (:230)
  pub fn align(&mut self) {
    const PAD_BIT: u64 = 1u64 << 31;
    if !self.count().is_multiple_of(2) {
      if (self.bits & PAD_BIT) != 0 {
        self.bits &= !PAD_BIT; // Remove(padreg)
      } else {
        self.bits |= PAD_BIT; // Combine(padreg)
      }
    }
    debug_assert!(self.count().is_multiple_of(2));
  }
}

// ── 32 个通用 64 位 X 寄存器常量 ──
`;

for (let i = 0; i < 32; i++) {
  out += `pub const X${i}: Register = Register::x(${i});\n`;
}
out += `pub const SP: Register = Register::sp();\n`;
out += `pub const XZR: Register = Register::x(${K_ZERO_REG_CODE});\n`;
out += `pub const NO_REG: Register = Register::no_reg();\n\n`;

out += "// ── 32 个通用 32 位 W 寄存器常量 ──\n";
for (let i = 0; i < 32; i++) {
  out += `pub const W${i}: Register = Register::w(${i});\n`;
}
out += `pub const WSP: Register = Register::wsp();\n`;
out += `pub const WZR: Register = Register::w(${K_ZERO_REG_CODE});\n\n`;

out += "// ── 32 个 VRegister (D 寄存器) 常量 ──\n";
for (let i = 0; i < 32; i++) {
  out += `pub const D${i}: VRegister = VRegister::d(${i});\n`;
  out += `pub const V${i}: VRegister = VRegister::v(${i});\n`;
  out += `pub const S${i}: VRegister = VRegister::s(${i});\n`;
  out += `pub const Q${i}: VRegister = VRegister::q(${i});\n`;
  out += `pub const B${i}: VRegister = VRegister::b(${i});\n`;
  out += `pub const H${i}: VRegister = VRegister::h(${i});\n`;
}
out += `pub const NO_DREG: VRegister = VRegister::no_reg();\n\n`;

out += `// ── 常见架构别名 (原版小写 ip0/fp/lr/... 由下列大写形态承接) ──
pub const IP0: Register = X16;
pub const IP1: Register = X17;
pub const WIP0: Register = W16;
pub const WIP1: Register = W17;
pub const FP: Register = X29;
pub const LR: Register = X30;
pub const CP: Register = X27;
pub const RR: Register = X26;

// C 调用约定参数寄存器 (kCArgRegs = {x0..x7})
pub const K_C_ARG_REGS: [Register; 8] = [X0, X1, X2, X3, X4, X5, X6, X7];
pub const K_REGISTER_PASSED_ARGUMENTS: usize = 8;
pub const K_FP_REGISTER_PASSED_ARGUMENTS: usize = 8;
`;

// ── wave1 纯数据面追加: 名字表 / 取名函数 / 完整别名链 / 调用约定别名 ──
out += `// ===== 名字表 (DEFINE_REGISTER_NAMES(Register, GENERAL_REGISTERS) 串化展开) =====
pub const GENERAL_REG_NAMES: [&str; ${REGISTER_NAMES.length}] = [
  ${REGISTER_NAMES.map((n) => `"${n}"`).join(", ")},
];

/// RegisterName(Register) 1:1 (register-base.h:83-87): 无效或越界编码返回 "invalid"
/// 注: sp 内部编码 63 越界, 原版 Names[63] 为未定义行为, 此处按 v8-codegen-x64 同款边界守护
#[inline]
pub const fn register_name(reg: Register) -> &'static str {
  let code = reg.code();
  if code >= 0 && (code as usize) < GENERAL_REG_NAMES.len() {
    GENERAL_REG_NAMES[code as usize]
  } else {
    "invalid"
  }
}

/// 名称反查编号, 与 register_name 互逆 (GENERAL_REG_NAMES 内名字唯一)
pub fn reg_code_of(name: &str) -> Option<i8> {
  GENERAL_REG_NAMES.iter().position(|&n| n == name).map(|i| i as i8)
}

/// GetSpecialRegisterName (register-arm64.h:279-281) 1:1: 仅认 sp 内部编码, 其余 "UNKNOWN"
#[inline]
pub const fn get_special_register_name(code: i32) -> &'static str {
  if code == K_SP_REG_INTERNAL_CODE {
    "sp"
  } else {
    "UNKNOWN"
  }
}

`;

// ALIAS_REGISTER 链中静态区未覆盖的条目 1:1 补全
const EMITTED_ALIAS = new Set(["ip0", "ip1", "wip0", "wip1", "cp", "fp", "lr", "rr", "xzr", "wzr"]);
const CAGE = {x28: "WITH_COMPRESSION", no_reg: "WITHOUT_COMPRESSION"};
for (const a of aliasLines) {
  if (EMITTED_ALIAS.has(a.alias)) continue;
  const rhs = rustRegExpr(a.target);
  if (a.alias === "kPtrComprCageBaseRegister") {
    must(CAGE[a.target], `kPtrComprCageBaseRegister 未知分支目标 ${a.target}`);
    out += `/// kPtrComprCageBaseRegister (V8_COMPRESS_POINTERS ${CAGE[a.target]})\npub const K_PTR_COMPR_CAGE_BASE_REGISTER_${CAGE[a.target]}: ${a.cls} = ${rhs};\n`;
  } else {
    out += `pub const ${rustUpper(a.alias)}: ${a.cls} = ${rhs};\n`;
  }
}
out += `\n// ===== 调用约定别名 (constexpr Register ${CC.length} 条 1:1) =====\n`;
for (const c of CC) {
  out += `pub const ${rustUpper(c.name)}: Register = ${rustRegExpr(c.target)};\n`;
}
out += `pub const K_FP_RETURN_REGISTER0: VRegister = D0; // constexpr DoubleRegister kFPReturnRegister0 = d0;\n`;

out += `
// ── 静态断言 ──
const _: () = assert!(size_of::<CPURegister>() <= 4);
const _: () = assert!(size_of::<Register>() <= 4);
const _: () = assert!(size_of::<VRegister>() <= 4);
const _: () = assert!(K_NUMBER_OF_REGISTERS == 32);
const _: () = assert!(K_NUMBER_OF_V_REGISTERS == 32);
const _: () = assert!(K_C_ARG_REGS.len() == 8);
const _: () = assert!(GENERAL_REG_NAMES.len() == K_NUMBER_OF_REGISTERS as usize);
`;

const path = writeSrc(CRATE, "registers.rs", out);
console.log(
  `register-arm64.h.js: 校验通过! 提取 GPR=32 VREG=32 名表=${REGISTER_NAMES.length} 别名=${aliasLines.length} 调用约定=${CC.length} 并生成 ${path}`,
);
