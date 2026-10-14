#!/usr/bin/env bun
// tool/src/codegen/x64/register-x64.h.js — v8/src/codegen/x64/register-x64.h → v8-codegen-x64/src/registers.rs
//
// wave1 纯数据面: 寄存器名表 / 编码枚举 / ABI 常量表 / constexpr 别名常量。
//   V-宏列表 (GENERAL_REGISTERS 等) 逐项解析并展开; #ifdef 双分支 (WIN ABI /
//   V8_COMPRESS_POINTERS / AVX10) 两支物化为 *_WIN / *_SYSV / *_WITH(OUT)_* 常量。
//   MacroAssembler 耦合方法 (ReassignRegister / AliasingKind / APX 分支) 推迟 wave2。
// 结构锚 + 对数断言真跑, missing 必 throw; 复跑幂等, 修订就地改, 复跑覆盖。
import { spawnSync } from "node:child_process";
import { readV8, writeSrc, banner, root } from "../../../lib.js";

const CRATE = "v8-codegen-x64";
const GEN = "src/codegen/x64/register-x64.h.js";
const SRC_REL = "src/codegen/x64/register-x64.h";
const BASE_REL = "src/codegen/register-base.h";
const H = readV8(SRC_REL);
const B = readV8(BASE_REL);
const GLOB = readV8("src/common/globals.h");

// 产物入仓前过 nightly rustfmt (固定点, 复跑幂等)
function fmtCrate() {
  const r = spawnSync("cargo", ["+nightly", "fmt", "-p", CRATE], { cwd: root, encoding: "utf8" });
  if (r.status !== 0) throw new Error(`${CRATE} rustfmt 失败: ${r.stderr}`);
}

function must(cond, msg) {
  if (!cond) throw new Error(`register-x64.h.js 对数核对失败: ${msg}`);
}

// ---------- 结构锚 ----------
must(H.includes("#ifndef V8_CODEGEN_X64_REGISTER_X64_H_"), "头文件卫哨缺失");
must(H.includes('#include "src/codegen/register-base.h"'), "register-base.h include 缺失");
must(B.includes("static constexpr int8_t kCode_no_reg = -1;"), "kCode_no_reg = -1 漂移");
must(B.includes("static constexpr int8_t kNumRegisters = kAfterLastRegister;"), "kNumRegisters 别名漂移");
must(B.includes("static_assert(kAfterLastRegister <= kMaxInt8);"), "register-base static_assert 缺失");
must(H.includes("constexpr AliasingKind kFPAliasing = AliasingKind::kOverlap;"), "kFPAliasing 锚点漂移 (值面推迟 wave2)");
must(H.includes("constexpr int kRegisterPassedArguments = arraysize(kCArgRegs);"), "kRegisterPassedArguments 锚点漂移");
must(H.includes("constexpr bool kSimdMaskRegisters = false;"), "kSimdMaskRegisters 锚点漂移");
must(H.includes("return V8_X64_16BYTE_STACK_ALIGNMENT_BOOL ? (argument_count & 1) : 0;"), "ArgumentPaddingSlots 规则漂移");
must(GLOB.includes("#define V8_X64_16BYTE_STACK_ALIGNMENT_BOOL true") && GLOB.includes("#define V8_X64_16BYTE_STACK_ALIGNMENT_BOOL false"), "16BYTE_STACK_ALIGNMENT 双分支宏漂移");
must(H.includes("#define FLOAT_REGISTERS ALL_XMM_REGISTERS") && H.includes("#define SIMD128_REGISTERS ALL_XMM_REGISTERS"), "Float/Simd128 别名宏漂移");

// ---------- static_assert / trivially-copyable 锚 (逐条对齐) ----------
const STATIC_ANCHORS = [
  "static_assert(sizeof(Register) <= sizeof(int)",
  "static_assert(sizeof(XMMRegister) <= sizeof(int)",
  "static_assert(sizeof(YMMRegister) <= sizeof(int)",
  "ASSERT_TRIVIALLY_COPYABLE(Register)",
  "ASSERT_TRIVIALLY_COPYABLE(XMMRegister)",
  "ASSERT_TRIVIALLY_COPYABLE(YMMRegister)",
  "static_assert(static_cast<int>(kDoubleAfterLast) ==",
  "static_cast<int>(kYMMAfterLast)",
];
for (const a of STATIC_ANCHORS) must(H.includes(a), `锚点缺失: ${a}`);

// ---------- V-宏列表解析 ----------
function vLists(name) {
  const lines = H.split("\n");
  const out = [];
  const head = new RegExp(`^#define ${name}\\(V\\)(.*)$`);
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(head);
    if (!m) continue;
    let body = m[1];
    let j = i;
    while (lines[j].trimEnd().endsWith("\\")) {
      j++;
      body += "\n" + lines[j];
    }
    out.push([...body.matchAll(/\bV\s*\(\s*(\w+)\s*\)/g)].map((x) => x[1]));
  }
  must(out.length > 0, `宏列表 ${name} 未找到`);
  return out;
}

function expect(actual, golden, label) {
  must(
    actual.length === golden.length && actual.every((v, i) => v === golden[i]),
    `${label} 序列漂移: [${actual.join(",")}] != [${golden.join(",")}]`,
  );
}

const GENERAL = vLists("GENERAL_REGISTERS");
must(GENERAL.length === 1, "GENERAL_REGISTERS 应单定义");
expect(GENERAL[0], ["rax","rcx","rdx","rbx","rsp","rbp","rsi","rdi","r8","r9","r10","r11","r12","r13","r14","r15"], "GENERAL_REGISTERS");

const ALWAYS = vLists("ALWAYS_ALLOCATABLE_GENERAL_REGISTERS");
must(ALWAYS.length === 1, "ALWAYS_ALLOCATABLE 应单定义");
expect(ALWAYS[0], ["rax","rbx","rdx","rcx","rsi","rdi","r8","r9","r11","r12","r15"], "ALWAYS_ALLOCATABLE");

const MAYBE = vLists("MAYBE_ALLOCATABLE_GENERAL_REGISTERS");
must(MAYBE.length === 2 && MAYBE[0].length === 0, "MAYBE_ALLOCATABLE 应两支且 COMPRESS 支为空");
expect(MAYBE[1], ["r14"], "MAYBE_ALLOCATABLE(无压缩支)");

const DOUBLE = vLists("DOUBLE_REGISTERS");
must(DOUBLE.length === 1, "DOUBLE_REGISTERS 应单定义");
expect(DOUBLE[0], Array.from({ length: 16 }, (_, i) => `xmm${i}`), "DOUBLE_REGISTERS");

const XMM_EXT = vLists("EXTENDED_XMM_REGISTERS");
expect(XMM_EXT[0], Array.from({ length: 16 }, (_, i) => `xmm${i + 16}`), "EXTENDED_XMM_REGISTERS");

const ALLOC_D = vLists("ALLOCATABLE_DOUBLE_REGISTERS");
expect(ALLOC_D[0], Array.from({ length: 15 }, (_, i) => `xmm${i}`), "ALLOCATABLE_DOUBLE_REGISTERS");

const YMM = vLists("YMM_REGISTERS");
expect(YMM[0], Array.from({ length: 16 }, (_, i) => `ymm${i}`), "YMM_REGISTERS");

const YMM_EXT = vLists("EXTENDED_YMM_REGISTERS");
expect(YMM_EXT[0], Array.from({ length: 16 }, (_, i) => `ymm${i + 16}`), "EXTENDED_YMM_REGISTERS");

must(H.includes("#define ALL_XMM_REGISTERS(V) DOUBLE_REGISTERS(V)"), "ALL_XMM 默认支漂移");
must(H.includes("#define ALL_YMM_REGISTERS(V) YMM_REGISTERS(V)"), "ALL_YMM 默认支漂移");

// ---------- 标量常量 ----------
function cnum(name, golden) {
  const m = H.match(new RegExp(`constexpr int ${name} = (\\d+);`));
  must(m, `constexpr int ${name} 缺失`);
  must(+m[1] === golden, `${name} 值漂移: ${m[1]} != ${golden}`);
  return +m[1];
}
const K_NUM_REGS = cnum("kNumRegs", 16);
const HOME_SLOTS = cnum("kWindowsHomeStackSlots", 4);

// ---------- kCArgRegs 双 ABI ----------
const cargs = [...H.matchAll(/constexpr Register kCArgRegs\[\] = \{([^}]+)\};/g)];
must(cargs.length === 2, "kCArgRegs 应 WIN/SYSV 两支");
must(cargs[0].index > H.indexOf("#ifdef V8_TARGET_OS_WIN"), "kCArgRegs 首支应在 WIN 分支");
const CARG_WIN = cargs[0][1].split(",").map((s) => s.trim());
const CARG_SYSV = cargs[1][1].split(",").map((s) => s.trim());
expect(CARG_WIN, ["rcx","rdx","r8","r9"], "kCArgRegs(WIN)");
expect(CARG_SYSV, ["rdi","rsi","rdx","rcx","r8","r9"], "kCArgRegs(SysV)");

// ---------- C-call callee-save 双 ABI ----------
function csvMacro(name) {
  const lines = H.split("\n");
  const out = [];
  const head = new RegExp(`^#define ${name}(.*)$`);
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(head);
    if (!m) continue;
    let body = m[1];
    let j = i;
    while (lines[j].trimEnd().endsWith("\\")) {
      j++;
      body += "\n" + lines[j];
    }
    out.push(body.split(/[,\s]+/).filter((t) => /^\w+$/.test(t)));
  }
  must(out.length === 2, `${name} 应 WIN/SYSV 两支`);
  return out;
}
const [CS_GPR_WIN, CS_GPR_SYSV] = csvMacro("C_CALL_CALLEE_SAVE_REGISTERS");
const [CS_FP_WIN, CS_FP_SYSV] = csvMacro("C_CALL_CALLEE_SAVE_FP_REGISTERS");
expect(CS_GPR_WIN, ["rbx","rdi","rsi","r12","r13","r14","r15"], "callee-save GPR(WIN)");
expect(CS_GPR_SYSV, ["rbx","r12","r13","r14","r15"], "callee-save GPR(SysV)");
expect(CS_FP_WIN, Array.from({ length: 10 }, (_, i) => `xmm${i + 6}`), "callee-save FP(WIN)");
must(CS_FP_SYSV.length === 0, "callee-save FP(SysV) 应为空");

// ---------- Register 别名常量 (kPtrComprCageBaseRegister 双支除外) ----------
const ALIAS_GOLDEN = {
  kStackPointerRegister: "rsp",
  kReturnRegister0: "rax",
  kReturnRegister1: "rdx",
  kReturnRegister2: "r8",
  kJSFunctionRegister: "rdi",
  kContextRegister: "rsi",
  kAllocateSizeRegister: "rdx",
  kInterpreterAccumulatorRegister: "rax",
  kInterpreterBytecodeOffsetRegister: "r9",
  kInterpreterBytecodeArrayRegister: "r12",
  kInterpreterDispatchTableRegister: "r15",
  kJavaScriptCallArgCountRegister: "rax",
  kJavaScriptCallCodeStartRegister: "rcx",
  kJavaScriptCallTargetRegister: "kJSFunctionRegister",
  kJavaScriptCallNewTargetRegister: "rdx",
  kJavaScriptCallExtraArg1Register: "rbx",
  kJavaScriptCallDispatchHandleRegister: "r15",
  kRuntimeCallFunctionRegister: "rbx",
  kRuntimeCallArgCountRegister: "rax",
  kRuntimeCallArgvRegister: "r15",
  kWasmImplicitArgRegister: "rsi",
  kWasmTrapHandlerFaultAddressRegister: "r10",
  kScratchRegister: "r10",
  kRootRegister: "r13",
};
const aliasMatches = [...H.matchAll(/^constexpr Register (k\w+) = ([\w]+);/gm)]
  .map((m) => ({ name: m[1], val: m[2] }))
  .filter((a) => a.name !== "kPtrComprCageBaseRegister");
must(aliasMatches.length === Object.keys(ALIAS_GOLDEN).length, `Register 别名数漂移: ${aliasMatches.length}`);
for (const a of aliasMatches) {
  must(ALIAS_GOLDEN[a.name] === a.val, `别名 ${a.name} 值漂移: ${a.val} != ${ALIAS_GOLDEN[a.name]}`);
}

// kPtrComprCageBaseRegister #ifdef V8_COMPRESS_POINTERS 双支
const ptrCompr = [...H.matchAll(/constexpr Register kPtrComprCageBaseRegister = ([\w]+);/g)];
must(ptrCompr.length === 2 && ptrCompr[0][1] === "r14" && ptrCompr[1][1] === "no_reg", "kPtrComprCageBaseRegister 双支漂移");

// ---------- Double/YMM 别名常量 ----------
const dAliases = [...H.matchAll(/^constexpr (DoubleRegister|XMMRegister|YMMRegister) (k\w+) = ([\w]+);/gm)].map((m) => ({ t: m[1], name: m[2], val: m[3] }));
const D_ALIAS_GOLDEN = {
  kScratchDoubleReg: ["XMMRegister", "xmm15"],
  kScratchSimd256Reg: ["YMMRegister", "ymm15"],
  kFPReturnRegister0: ["DoubleRegister", "xmm0"],
};
must(dAliases.length === 3, `Double/YMM 别名数漂移: ${dAliases.length}`);
for (const a of dAliases) {
  const g = D_ALIAS_GOLDEN[a.name];
  must(g && g[0] === a.t && g[1] === a.val, `别名 ${a.name} 漂移: ${a.t} ${a.val}`);
}

// ---------- 组装生成 ----------
const rc = (n) => n.toUpperCase(); // rax→RAX, xmm5→XMM5, ymm2→YMM2
const list = (xs) => xs.map((n) => rc(n)).join(", ");
const strs = (xs) => xs.map((n) => `"${n}"`).join(", ");
const aliasRef = (v) => (v === "no_reg" ? "NO_REG" : v.startsWith("k") ? rustConst(v) : rc(v));
function rustConst(name) {
  const s = name.replace(/^k/, "");
  return (
    "K_" +
    s
      .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
      .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
      .toUpperCase()
  );
}

const G = GENERAL[0];
const D = DOUBLE[0];
const DE = XMM_EXT[0];
const Y = YMM[0];
const YE = YMM_EXT[0];
const K_REG_AFTER_LAST = G.length;
const K_DOUBLE_AFTER_LAST = D.length;
const K_YMM_AFTER_LAST = Y.length;
const K_DOUBLE_AFTER_LAST_AVX10 = D.length + DE.length;
const K_YMM_AFTER_LAST_AVX10 = Y.length + YE.length;

let s = banner(GEN, SRC_REL, "wave1 纯数据面: 零依赖寄存器表; MacroAssembler 耦合面见 readme 推迟清单。");

s += `
use std::mem::size_of;

// ===== RegisterBase (v8/${BASE_REL}) 数据面基准 =====
/// RegisterBase::kCode_no_reg
pub const K_CODE_NO_REG: i8 = -1;
/// kNumRegs: 通用寄存器数 (register-x64.h:112; C++ 型别 int, Rust 侧用作数组维故 usize, 型别偏差在此登记)
pub const K_NUM_REGS: usize = ${K_NUM_REGS};
// 承接 register-base.h:70 static_assert(kAfterLastRegister <= kMaxInt8)——主语系 kAfterLastRegister, 与 kNumRegs 数值相等由跨表断言钉死
const _: () = assert!(K_NUM_REGS <= i8::MAX as usize);

// ===== enum RegisterCode (kRegCode_* 逐项) =====
pub mod reg_code {
  //! RegisterCode: 按 GENERAL_REGISTERS(V) 展开序编号
${G.map((n, i) => `  pub const ${rc(n)}: i8 = ${i};`).join("\n")}
  pub const K_REG_AFTER_LAST: i8 = ${K_REG_AFTER_LAST};
}

pub mod double_code {
  //! DoubleRegisterCode: 按 ALL_XMM_REGISTERS 展开序编号 (xmm16-31 仅 AVX10 配置启用)
${[...D, ...DE].map((n, i) => `  pub const ${rc(n)}: i8 = ${i};`).join("\n")}
  /// 默认构建 (非 AVX10) 的 kDoubleAfterLast
  pub const K_DOUBLE_AFTER_LAST: i8 = ${K_DOUBLE_AFTER_LAST};
  /// AVX10 构建的 kDoubleAfterLast
  pub const K_DOUBLE_AFTER_LAST_AVX10: i8 = ${K_DOUBLE_AFTER_LAST_AVX10};
}

pub mod ymm_code {
  //! YMMRegisterCode: 按 ALL_YMM_REGISTERS 展开序编号 (ymm16-31 仅 AVX10 配置启用)
${[...Y, ...YE].map((n, i) => `  pub const ${rc(n)}: i8 = ${i};`).join("\n")}
  /// 默认构建 (非 AVX10) 的 kYMMAfterLast
  pub const K_YMM_AFTER_LAST: i8 = ${K_YMM_AFTER_LAST};
  /// AVX10 构建的 kYMMAfterLast
  pub const K_YMM_AFTER_LAST_AVX10: i8 = ${K_YMM_AFTER_LAST_AVX10};
}

// ===== Register =====

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register {
  code: i8,
}

impl Register {
  /// 裸编码, 不校验范围 (对应 C++ 私有 ctor / AVX10 扩展位)
  #[inline]
  pub const fn with_code(code: i8) -> Self {
    Self { code }
  }
  #[inline]
  pub const fn no_reg() -> Self {
    Self { code: K_CODE_NO_REG }
  }
  #[inline]
  pub const fn from_code(code: i8) -> Self {
    debug_assert!(code >= 0 && (code as usize) < K_NUM_REGS);
    Self { code }
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
  pub const fn is_byte_register(self) -> bool {
    self.code <= 3
  }
  /// REX2 前缀用: code 第 5 位
  #[inline]
  pub const fn bit4(self) -> u8 {
    ((self.code >> 4) & 0x1) as u8
  }
  /// 已采掩码支 (V8_ENABLE_APX_F, cc:70; 与 cc:72 非掩码支在合法域 0-15 数值等价)
  #[inline]
  pub const fn high_bit(self) -> u8 {
    ((self.code >> 3) & 0x1) as u8
  }
  /// modR/M、SIB、opcode 编码用低 3 位
  #[inline]
  pub const fn low_bits(self) -> u8 {
    (self.code & 0x7) as u8
  }
}

// static_assert(sizeof(Register) <= sizeof(int))
const _: () = assert!(size_of::<Register>() <= size_of::<i32>());

/// 存放 tagged 值的寄存器包装 (指针压缩下为压缩形态)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaggedRegister {
  pub reg: Register,
}

impl TaggedRegister {
  #[inline]
  pub const fn new(reg: Register) -> Self {
    Self { reg }
  }
}

// ===== RegList 通用寄存器集合 =====
// 原版 storage_t: kNumRegisters=16 ≤ 16 → uint16_t (reglist-base.h:22-24,37)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RegList {
  bits: u16,
}

impl RegList {
  #[inline]
  pub const fn empty() -> Self {
    Self { bits: 0 }
  }

  #[inline]
  pub const fn from_bits(bits: u16) -> Self {
    Self { bits }
  }

  #[inline]
  pub fn from_slice(regs: &[Register]) -> Self {
    let mut list = Self::empty();
    for &reg in regs {
      list.set(reg);
    }
    list
  }

  #[inline]
  pub const fn bits(self) -> u16 {
    self.bits
  }

  #[inline]
  pub const fn is_empty(self) -> bool {
    self.bits == 0
  }

  #[inline]
  pub const fn len(self) -> usize {
    self.bits.count_ones() as usize
  }

  /// 原版 storage_t{1}<<code 经 int 提升后窄化回 uint16_t 的承接 (reglist-base.h:47-59):
  /// code≥16 窄化为 0 → 静默 no-op (原版合法域 0-15, 该支经 register-base.h:48 V8_ASSUME 不可达)
  #[inline]
  const fn bit_for(code: i8) -> u16 {
    if code < 0 || code >= 32 {
      0
    } else {
      (1u32 << code) as u16
    }
  }

  #[inline]
  pub fn set(&mut self, reg: Register) {
    if reg.is_valid() {
      self.bits |= Self::bit_for(reg.code());
    }
  }

  #[inline]
  pub fn clear(&mut self, reg: Register) {
    if reg.is_valid() {
      self.bits &= !Self::bit_for(reg.code());
    }
  }

  #[inline]
  pub const fn has(self, reg: Register) -> bool {
    if !reg.is_valid() {
      return false;
    }
    (self.bits & Self::bit_for(reg.code())) != 0
  }

  #[inline]
  pub const fn union(self, other: Self) -> Self {
    Self {
      bits: self.bits | other.bits,
    }
  }

  #[inline]
  pub const fn intersect(self, other: Self) -> Self {
    Self {
      bits: self.bits & other.bits,
    }
  }
}

impl FromIterator<Register> for RegList {
  #[inline]
  fn from_iter<T: IntoIterator<Item = Register>>(iter: T) -> Self {
    let mut list = Self::empty();
    for reg in iter {
      list.set(reg);
    }
    list
  }
}

impl Extend<Register> for RegList {
  #[inline]
  fn extend<T: IntoIterator<Item = Register>>(&mut self, iter: T) {
    for reg in iter {
      self.set(reg);
    }
  }
}

// 与原版 storage_t 严格 1:1: sizeof(RegList) == sizeof(uint16_t) (reglist-base.h:22-24)
const _: () = assert!(size_of::<RegList>() == size_of::<u16>());

// ===== 通用寄存器常量 (DECLARE_REGISTER 展开) =====
${G.map((n, i) => `pub const ${rc(n)}: Register = Register::with_code(reg_code::${rc(n)}); // code=${i}`).join("\n")}
/// no_reg
pub const NO_REG: Register = Register::no_reg();
// 注: 原版 C++ 侧寄存器常量为小写 (rax/rcx/...), Rust 侧统一大写形态承接 (本表上方),
// 小写别名已按 non_upper_case_globals 清障移除。

// ===== 名字表 (DEFINE_REGISTER_NAMES 展开) =====
pub const GENERAL_REG_NAMES: [&str; K_NUM_REGS] = [${strs(G)}];

/// RegisterName(Register): 无效编码返回 "invalid"
#[inline]
pub const fn register_name(reg: Register) -> &'static str {
  let code = reg.code();
  if code >= 0 && (code as usize) < GENERAL_REG_NAMES.len() {
    GENERAL_REG_NAMES[code as usize]
  } else {
    "invalid"
  }
}

// ===== 可分配寄存器集合 =====
pub const ALWAYS_ALLOCATABLE_GENERAL_REGISTERS: [Register; ${ALWAYS[0].length}] = [${list(ALWAYS[0])}];
/// V8_COMPRESS_POINTERS 启用时为空
pub const MAYBE_ALLOCATABLE_GENERAL_REGISTERS_WITH_COMPRESSION: [Register; ${MAYBE[0].length}] = [${list(MAYBE[0])}];
/// V8_COMPRESS_POINTERS 未启用时为 [r14]
pub const MAYBE_ALLOCATABLE_GENERAL_REGISTERS_WITHOUT_COMPRESSION: [Register; ${MAYBE[1].length}] = [${list(MAYBE[1])}];
/// ALLOCATABLE = ALWAYS + MAYBE (V8_COMPRESS_POINTERS 支)
pub const ALLOCATABLE_GENERAL_REGISTERS_WITH_COMPRESSION: [Register; ${ALWAYS[0].length + MAYBE[0].length}] = [${list([...ALWAYS[0], ...MAYBE[0]])}];
/// ALLOCATABLE = ALWAYS + MAYBE (无压缩支)
pub const ALLOCATABLE_GENERAL_REGISTERS_WITHOUT_COMPRESSION: [Register; ${ALWAYS[0].length + MAYBE[1].length}] = [${list([...ALWAYS[0], ...MAYBE[1]])}];

// ===== C 调用约定 (双 ABI 物化) =====
pub const K_C_ARG_REGS_WIN: [Register; ${CARG_WIN.length}] = [${list(CARG_WIN)}];
pub const K_C_ARG_REGS_SYSV: [Register; ${CARG_SYSV.length}] = [${list(CARG_SYSV)}];
/// kRegisterPassedArguments = arraysize(kCArgRegs) (WIN 支)
pub const K_REGISTER_PASSED_ARGUMENTS_WIN: usize = K_C_ARG_REGS_WIN.len();
/// kRegisterPassedArguments = arraysize(kCArgRegs) (SysV 支)
pub const K_REGISTER_PASSED_ARGUMENTS_SYSV: usize = K_C_ARG_REGS_SYSV.len();
/// WIN ABI: 恒定保留 4 个 spill 槽 (home space)
pub const K_WINDOWS_HOME_STACK_SLOTS: i32 = ${HOME_SLOTS};

pub const C_CALL_CALLEE_SAVE_REGISTERS_WIN: [Register; ${CS_GPR_WIN.length}] = [${list(CS_GPR_WIN)}];
pub const C_CALL_CALLEE_SAVE_REGISTERS_SYSV: [Register; ${CS_GPR_SYSV.length}] = [${list(CS_GPR_SYSV)}];
pub const C_CALL_CALLEE_SAVE_FP_REGISTERS_WIN: [XMMRegister; ${CS_FP_WIN.length}] = [${list(CS_FP_WIN)}];
pub const C_CALL_CALLEE_SAVE_FP_REGISTERS_SYSV: [XMMRegister; 0] = [];

// ===== Register 别名常量 (调用约定命名) =====
${aliasMatches.map((a) => `pub const ${rustConst(a.name)}: Register = ${aliasRef(a.val)};`).join("\n")}
/// kPtrComprCageBaseRegister (V8_COMPRESS_POINTERS 支): r14
pub const K_PTR_COMPR_CAGE_BASE_REGISTER_WITH_COMPRESSION: Register = R14;
/// kPtrComprCageBaseRegister (无压缩支): no_reg
pub const K_PTR_COMPR_CAGE_BASE_REGISTER_WITHOUT_COMPRESSION: Register = NO_REG;

// ===== XMM / YMM =====

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XMMRegister {
  code: i8,
}

impl XMMRegister {
  pub const fn with_code(code: i8) -> Self {
    Self { code }
  }
  pub const fn no_reg() -> Self {
    Self { code: K_CODE_NO_REG }
  }
  pub const fn from_code(code: i8) -> Self {
    debug_assert!(code >= 0 && (code as usize) < double_code::K_DOUBLE_AFTER_LAST as usize);
    Self { code }
  }
  pub const fn is_valid(self) -> bool {
    self.code != K_CODE_NO_REG
  }
  pub const fn code(self) -> i8 {
    self.code
  }
  /// EVEX 前缀用: 与 high_bit 联合编码 xmm16-31
  #[inline]
  pub const fn bit4(self) -> u8 {
    ((self.code >> 4) & 0x1) as u8
  }
  /// 取 AVX10/APX 掩码支 (cc:287): 默认构建合法域 0-15 内与 cc:291 非掩码支等价;
  /// AVX10 域 code16-23 两支语义分叉 (掩码=0/非掩码=2), 本表以掩码支为准 (偏差已登记)
  #[inline]
  pub const fn high_bit(self) -> u8 {
    ((self.code >> 3) & 0x1) as u8
  }
  #[inline]
  pub const fn low_bits(self) -> u8 {
    (self.code & 0x7) as u8
  }
}

const _: () = assert!(size_of::<XMMRegister>() <= size_of::<i32>());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YMMRegister {
  code: i8,
}

impl YMMRegister {
  #[inline]
  pub const fn with_code(code: i8) -> Self {
    Self { code }
  }
  #[inline]
  pub const fn no_reg() -> Self {
    Self { code: K_CODE_NO_REG }
  }
  #[inline]
  pub const fn from_code(code: i8) -> Self {
    debug_assert!(code >= 0 && (code as usize) < ymm_code::K_YMM_AFTER_LAST as usize);
    Self { code }
  }
  #[inline]
  pub const fn from_xmm(xmm: XMMRegister) -> Self {
    Self { code: xmm.code() }
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
  pub const fn bit4(self) -> u8 {
    ((self.code >> 4) & 0x1) as u8
  }
  #[inline]
  pub const fn high_bit(self) -> u8 {
    ((self.code >> 3) & 0x1) as u8
  }
  #[inline]
  pub const fn low_bits(self) -> u8 {
    (self.code & 0x7) as u8
  }
}

const _: () = assert!(size_of::<YMMRegister>() <= size_of::<i32>());

pub type FloatRegister = XMMRegister;
pub type DoubleRegister = XMMRegister;
pub type Simd128Register = XMMRegister;
pub type Simd256Register = YMMRegister;

${[...D].map((n) => `pub const ${rc(n)}: XMMRegister = XMMRegister::from_code(double_code::${rc(n)});`).join("\n")}
${DE.map((n) => `/// AVX10 配置专用扩展寄存器\npub const ${rc(n)}: XMMRegister = XMMRegister::with_code(double_code::${rc(n)});`).join("\n")}
/// no_dreg
pub const NO_DREG: XMMRegister = XMMRegister::no_reg();
// 注: 原版小写 xmm 别名 (xmm0../no_dreg) 同通用寄存器处理, 统一大写形态承接。

${Y.map((n) => `pub const ${rc(n)}: YMMRegister = YMMRegister::from_code(ymm_code::${rc(n)});`).join("\n")}
${YE.map((n) => `/// AVX10 配置专用扩展寄存器\npub const ${rc(n)}: YMMRegister = YMMRegister::with_code(ymm_code::${rc(n)});`).join("\n")}
/// YMMRegister::no_reg
pub const NO_YMM_REG: YMMRegister = YMMRegister::no_reg();

// ===== XMM/YMM 名字表 =====
pub const XMM_REG_NAMES: [&str; ${D.length}] = [${strs(D)}];
pub const XMM_EXT_REG_NAMES: [&str; ${DE.length}] = [${strs(DE)}];
/// ALL_XMM_REGISTERS (AVX10 支, 32 项; 默认构建仅前 ${D.length} 项有效)
pub const ALL_XMM_REG_NAMES: [&str; ${D.length + DE.length}] = [${strs([...D, ...DE])}];
pub const YMM_REG_NAMES: [&str; ${Y.length}] = [${strs(Y)}];
pub const YMM_EXT_REG_NAMES: [&str; ${YE.length}] = [${strs(YE)}];
/// ALL_YMM_REGISTERS (AVX10 支, 32 项; 默认构建仅前 ${Y.length} 项有效)
pub const ALL_YMM_REG_NAMES: [&str; ${Y.length + YE.length}] = [${strs([...Y, ...YE])}];

/// RegisterName(XMMRegister): 无效编码返回 "invalid"
#[inline]
pub const fn xmm_register_name(reg: XMMRegister) -> &'static str {
  let code = reg.code();
  if code >= 0 && (code as usize) < ALL_XMM_REG_NAMES.len() {
    ALL_XMM_REG_NAMES[code as usize]
  } else {
    "invalid"
  }
}

/// RegisterName(YMMRegister): 无效编码返回 "invalid"
#[inline]
pub const fn ymm_register_name(reg: YMMRegister) -> &'static str {
  let code = reg.code();
  if code >= 0 && (code as usize) < ALL_YMM_REG_NAMES.len() {
    ALL_YMM_REG_NAMES[code as usize]
  } else {
    "invalid"
  }
}

/// kSimdMaskRegisters: x64 无独立 SIMD mask 寄存器组
pub const K_SIMD_MASK_REGISTERS: bool = false;

/// ArgumentPaddingSlots: 返回栈对齐所需 padding 槽数;
/// 参数 x64_16byte_stack_alignment 对应构建开关 V8_X64_16BYTE_STACK_ALIGNMENT_BOOL
pub const fn argument_padding_slots(argument_count: i32, x64_16byte_stack_alignment: bool) -> i32 {
  if x64_16byte_stack_alignment {
    argument_count & 1
  } else {
    0
  }
}

// ===== Double/YMM 别名常量 =====
${dAliases.map((a) => `pub const ${rustConst(a.name)}: ${a.t === "YMMRegister" ? "YMMRegister" : "XMMRegister"} = ${rc(a.val)};`).join("\n")}

pub const ALLOCATABLE_DOUBLE_REGISTERS: [XMMRegister; ${ALLOC_D[0].length}] = [${list(ALLOC_D[0])}];

// ===== 跨表静态断言 (逐条对齐头内 static_assert / arraysize 不变式) =====
const _: () = assert!(reg_code::K_REG_AFTER_LAST as usize == K_NUM_REGS);
const _: () = assert!(GENERAL_REG_NAMES.len() == K_NUM_REGS);
const _: () = assert!(double_code::K_DOUBLE_AFTER_LAST == ymm_code::K_YMM_AFTER_LAST); // kDoubleAfterLast == kYMMAfterLast
const _: () = assert!(double_code::K_DOUBLE_AFTER_LAST_AVX10 == ymm_code::K_YMM_AFTER_LAST_AVX10);
const _: () = assert!(ALL_XMM_REG_NAMES.len() == ALL_YMM_REG_NAMES.len());
const _: () = assert!(XMM_REG_NAMES.len() == YMM_REG_NAMES.len());
const _: () = assert!(XMM_EXT_REG_NAMES.len() == YMM_EXT_REG_NAMES.len());
const _: () = assert!(ALLOCATABLE_GENERAL_REGISTERS_WITH_COMPRESSION.len() == ALWAYS_ALLOCATABLE_GENERAL_REGISTERS.len());
const _: () = assert!(ALLOCATABLE_GENERAL_REGISTERS_WITHOUT_COMPRESSION.len() == ALWAYS_ALLOCATABLE_GENERAL_REGISTERS.len() + MAYBE_ALLOCATABLE_GENERAL_REGISTERS_WITHOUT_COMPRESSION.len());
const _: () = assert!(K_C_ARG_REGS_WIN.len() == ${CARG_WIN.length} && K_C_ARG_REGS_SYSV.len() == ${CARG_SYSV.length});
const _: () = assert!(K_REGISTER_PASSED_ARGUMENTS_WIN == ${CARG_WIN.length});
const _: () = assert!(K_REGISTER_PASSED_ARGUMENTS_SYSV == ${CARG_SYSV.length});
const _: () = assert!(C_CALL_CALLEE_SAVE_FP_REGISTERS_SYSV.is_empty());
const _: () = assert!(ALLOCATABLE_DOUBLE_REGISTERS.len() == ${ALLOC_D[0].length});
`;

writeSrc(CRATE, "registers.rs", s);
fmtCrate();

// ---------- 对数汇总 ----------
const counted = [
  ["GENERAL_REGISTERS", G.length],
  ["ALLOCATABLE(GPR,compress)", ALWAYS[0].length + MAYBE[0].length],
  ["ALLOCATABLE(GPR,no-compress)", ALWAYS[0].length + MAYBE[1].length],
  ["XMM base/ext", `${D.length}/${DE.length}`],
  ["YMM base/ext", `${Y.length}/${YE.length}`],
  ["ALLOCATABLE_DOUBLE", ALLOC_D[0].length],
  ["Register 别名", aliasMatches.length],
  ["Double/YMM 别名", dAliases.length],
  ["kCArgRegs WIN/SYSV", `${CARG_WIN.length}/${CARG_SYSV.length}`],
  ["callee-save GPR WIN/SYSV", `${CS_GPR_WIN.length}/${CS_GPR_SYSV.length}`],
  ["callee-save FP WIN/SYSV", `${CS_FP_WIN.length}/${CS_FP_SYSV.length}`],
  ["标量常量", 4],
  ["静态断言 (产物全量 const _:)", (s.match(/const _: \(\) =/g) || []).length],
];
console.log(`register-x64.h.js: 通过! 输出 ${CRATE}/src/registers.rs`);
for (const [k, v] of counted) console.log(`  ${k} = ${v}`);
