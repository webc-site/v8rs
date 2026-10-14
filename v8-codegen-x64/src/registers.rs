// 由 tool/src/codegen/x64/register-x64.h.js 从 v8/src/codegen/x64/register-x64.h 生成; 修订就地改, 复跑覆盖
// wave1 纯数据面: 零依赖寄存器表; MacroAssembler 耦合面见 readme 推迟清单。

use std::mem::size_of;

// ===== RegisterBase (v8/src/codegen/register-base.h) 数据面基准 =====
/// RegisterBase::kCode_no_reg
pub const K_CODE_NO_REG: i8 = -1;
/// kNumRegs: 通用寄存器数 (register-x64.h:112; C++ 型别 int, Rust 侧用作数组维故 usize, 型别偏差在此登记)
pub const K_NUM_REGS: usize = 16;
// 承接 register-base.h:70 static_assert(kAfterLastRegister <= kMaxInt8)——主语系 kAfterLastRegister, 与 kNumRegs 数值相等由跨表断言钉死
const _: () = assert!(K_NUM_REGS <= i8::MAX as usize);

// ===== enum RegisterCode (kRegCode_* 逐项) =====
pub mod reg_code {
  //! RegisterCode: 按 GENERAL_REGISTERS(V) 展开序编号
  pub const RAX: i8 = 0;
  pub const RCX: i8 = 1;
  pub const RDX: i8 = 2;
  pub const RBX: i8 = 3;
  pub const RSP: i8 = 4;
  pub const RBP: i8 = 5;
  pub const RSI: i8 = 6;
  pub const RDI: i8 = 7;
  pub const R8: i8 = 8;
  pub const R9: i8 = 9;
  pub const R10: i8 = 10;
  pub const R11: i8 = 11;
  pub const R12: i8 = 12;
  pub const R13: i8 = 13;
  pub const R14: i8 = 14;
  pub const R15: i8 = 15;
  pub const K_REG_AFTER_LAST: i8 = 16;
}

pub mod double_code {
  //! DoubleRegisterCode: 按 ALL_XMM_REGISTERS 展开序编号 (xmm16-31 仅 AVX10 配置启用)
  pub const XMM0: i8 = 0;
  pub const XMM1: i8 = 1;
  pub const XMM2: i8 = 2;
  pub const XMM3: i8 = 3;
  pub const XMM4: i8 = 4;
  pub const XMM5: i8 = 5;
  pub const XMM6: i8 = 6;
  pub const XMM7: i8 = 7;
  pub const XMM8: i8 = 8;
  pub const XMM9: i8 = 9;
  pub const XMM10: i8 = 10;
  pub const XMM11: i8 = 11;
  pub const XMM12: i8 = 12;
  pub const XMM13: i8 = 13;
  pub const XMM14: i8 = 14;
  pub const XMM15: i8 = 15;
  pub const XMM16: i8 = 16;
  pub const XMM17: i8 = 17;
  pub const XMM18: i8 = 18;
  pub const XMM19: i8 = 19;
  pub const XMM20: i8 = 20;
  pub const XMM21: i8 = 21;
  pub const XMM22: i8 = 22;
  pub const XMM23: i8 = 23;
  pub const XMM24: i8 = 24;
  pub const XMM25: i8 = 25;
  pub const XMM26: i8 = 26;
  pub const XMM27: i8 = 27;
  pub const XMM28: i8 = 28;
  pub const XMM29: i8 = 29;
  pub const XMM30: i8 = 30;
  pub const XMM31: i8 = 31;
  /// 默认构建 (非 AVX10) 的 kDoubleAfterLast
  pub const K_DOUBLE_AFTER_LAST: i8 = 16;
  /// AVX10 构建的 kDoubleAfterLast
  pub const K_DOUBLE_AFTER_LAST_AVX10: i8 = 32;
}

pub mod ymm_code {
  //! YMMRegisterCode: 按 ALL_YMM_REGISTERS 展开序编号 (ymm16-31 仅 AVX10 配置启用)
  pub const YMM0: i8 = 0;
  pub const YMM1: i8 = 1;
  pub const YMM2: i8 = 2;
  pub const YMM3: i8 = 3;
  pub const YMM4: i8 = 4;
  pub const YMM5: i8 = 5;
  pub const YMM6: i8 = 6;
  pub const YMM7: i8 = 7;
  pub const YMM8: i8 = 8;
  pub const YMM9: i8 = 9;
  pub const YMM10: i8 = 10;
  pub const YMM11: i8 = 11;
  pub const YMM12: i8 = 12;
  pub const YMM13: i8 = 13;
  pub const YMM14: i8 = 14;
  pub const YMM15: i8 = 15;
  pub const YMM16: i8 = 16;
  pub const YMM17: i8 = 17;
  pub const YMM18: i8 = 18;
  pub const YMM19: i8 = 19;
  pub const YMM20: i8 = 20;
  pub const YMM21: i8 = 21;
  pub const YMM22: i8 = 22;
  pub const YMM23: i8 = 23;
  pub const YMM24: i8 = 24;
  pub const YMM25: i8 = 25;
  pub const YMM26: i8 = 26;
  pub const YMM27: i8 = 27;
  pub const YMM28: i8 = 28;
  pub const YMM29: i8 = 29;
  pub const YMM30: i8 = 30;
  pub const YMM31: i8 = 31;
  /// 默认构建 (非 AVX10) 的 kYMMAfterLast
  pub const K_YMM_AFTER_LAST: i8 = 16;
  /// AVX10 构建的 kYMMAfterLast
  pub const K_YMM_AFTER_LAST_AVX10: i8 = 32;
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
    Self {
      code: K_CODE_NO_REG,
    }
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
pub const RAX: Register = Register::with_code(reg_code::RAX); // code=0
pub const RCX: Register = Register::with_code(reg_code::RCX); // code=1
pub const RDX: Register = Register::with_code(reg_code::RDX); // code=2
pub const RBX: Register = Register::with_code(reg_code::RBX); // code=3
pub const RSP: Register = Register::with_code(reg_code::RSP); // code=4
pub const RBP: Register = Register::with_code(reg_code::RBP); // code=5
pub const RSI: Register = Register::with_code(reg_code::RSI); // code=6
pub const RDI: Register = Register::with_code(reg_code::RDI); // code=7
pub const R8: Register = Register::with_code(reg_code::R8); // code=8
pub const R9: Register = Register::with_code(reg_code::R9); // code=9
pub const R10: Register = Register::with_code(reg_code::R10); // code=10
pub const R11: Register = Register::with_code(reg_code::R11); // code=11
pub const R12: Register = Register::with_code(reg_code::R12); // code=12
pub const R13: Register = Register::with_code(reg_code::R13); // code=13
pub const R14: Register = Register::with_code(reg_code::R14); // code=14
pub const R15: Register = Register::with_code(reg_code::R15); // code=15
/// no_reg
pub const NO_REG: Register = Register::no_reg();
// 注: 原版 C++ 侧寄存器常量为小写 (rax/rcx/...), Rust 侧统一大写形态承接 (本表上方),
// 小写别名已按 non_upper_case_globals 清障移除。

// ===== 名字表 (DEFINE_REGISTER_NAMES 展开) =====
pub const GENERAL_REG_NAMES: [&str; K_NUM_REGS] = [
  "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13",
  "r14", "r15",
];

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
pub const ALWAYS_ALLOCATABLE_GENERAL_REGISTERS: [Register; 11] =
  [RAX, RBX, RDX, RCX, RSI, RDI, R8, R9, R11, R12, R15];
/// V8_COMPRESS_POINTERS 启用时为空
pub const MAYBE_ALLOCATABLE_GENERAL_REGISTERS_WITH_COMPRESSION: [Register; 0] = [];
/// V8_COMPRESS_POINTERS 未启用时为 [r14]
pub const MAYBE_ALLOCATABLE_GENERAL_REGISTERS_WITHOUT_COMPRESSION: [Register; 1] = [R14];
/// ALLOCATABLE = ALWAYS + MAYBE (V8_COMPRESS_POINTERS 支)
pub const ALLOCATABLE_GENERAL_REGISTERS_WITH_COMPRESSION: [Register; 11] =
  [RAX, RBX, RDX, RCX, RSI, RDI, R8, R9, R11, R12, R15];
/// ALLOCATABLE = ALWAYS + MAYBE (无压缩支)
pub const ALLOCATABLE_GENERAL_REGISTERS_WITHOUT_COMPRESSION: [Register; 12] =
  [RAX, RBX, RDX, RCX, RSI, RDI, R8, R9, R11, R12, R15, R14];

// ===== C 调用约定 (双 ABI 物化) =====
pub const K_C_ARG_REGS_WIN: [Register; 4] = [RCX, RDX, R8, R9];
pub const K_C_ARG_REGS_SYSV: [Register; 6] = [RDI, RSI, RDX, RCX, R8, R9];
/// kRegisterPassedArguments = arraysize(kCArgRegs) (WIN 支)
pub const K_REGISTER_PASSED_ARGUMENTS_WIN: usize = K_C_ARG_REGS_WIN.len();
/// kRegisterPassedArguments = arraysize(kCArgRegs) (SysV 支)
pub const K_REGISTER_PASSED_ARGUMENTS_SYSV: usize = K_C_ARG_REGS_SYSV.len();
/// WIN ABI: 恒定保留 4 个 spill 槽 (home space)
pub const K_WINDOWS_HOME_STACK_SLOTS: i32 = 4;

pub const C_CALL_CALLEE_SAVE_REGISTERS_WIN: [Register; 7] = [RBX, RDI, RSI, R12, R13, R14, R15];
pub const C_CALL_CALLEE_SAVE_REGISTERS_SYSV: [Register; 5] = [RBX, R12, R13, R14, R15];
pub const C_CALL_CALLEE_SAVE_FP_REGISTERS_WIN: [XMMRegister; 10] = [
  XMM6, XMM7, XMM8, XMM9, XMM10, XMM11, XMM12, XMM13, XMM14, XMM15,
];
pub const C_CALL_CALLEE_SAVE_FP_REGISTERS_SYSV: [XMMRegister; 0] = [];

// ===== Register 别名常量 (调用约定命名) =====
pub const K_STACK_POINTER_REGISTER: Register = RSP;
pub const K_RETURN_REGISTER0: Register = RAX;
pub const K_RETURN_REGISTER1: Register = RDX;
pub const K_RETURN_REGISTER2: Register = R8;
pub const K_JS_FUNCTION_REGISTER: Register = RDI;
pub const K_CONTEXT_REGISTER: Register = RSI;
pub const K_ALLOCATE_SIZE_REGISTER: Register = RDX;
pub const K_INTERPRETER_ACCUMULATOR_REGISTER: Register = RAX;
pub const K_INTERPRETER_BYTECODE_OFFSET_REGISTER: Register = R9;
pub const K_INTERPRETER_BYTECODE_ARRAY_REGISTER: Register = R12;
pub const K_INTERPRETER_DISPATCH_TABLE_REGISTER: Register = R15;
pub const K_JAVA_SCRIPT_CALL_ARG_COUNT_REGISTER: Register = RAX;
pub const K_JAVA_SCRIPT_CALL_CODE_START_REGISTER: Register = RCX;
pub const K_JAVA_SCRIPT_CALL_TARGET_REGISTER: Register = K_JS_FUNCTION_REGISTER;
pub const K_JAVA_SCRIPT_CALL_NEW_TARGET_REGISTER: Register = RDX;
pub const K_JAVA_SCRIPT_CALL_EXTRA_ARG1_REGISTER: Register = RBX;
pub const K_JAVA_SCRIPT_CALL_DISPATCH_HANDLE_REGISTER: Register = R15;
pub const K_RUNTIME_CALL_FUNCTION_REGISTER: Register = RBX;
pub const K_RUNTIME_CALL_ARG_COUNT_REGISTER: Register = RAX;
pub const K_RUNTIME_CALL_ARGV_REGISTER: Register = R15;
pub const K_WASM_IMPLICIT_ARG_REGISTER: Register = RSI;
pub const K_WASM_TRAP_HANDLER_FAULT_ADDRESS_REGISTER: Register = R10;
pub const K_SCRATCH_REGISTER: Register = R10;
pub const K_ROOT_REGISTER: Register = R13;
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
    Self {
      code: K_CODE_NO_REG,
    }
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
    Self {
      code: K_CODE_NO_REG,
    }
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

pub const XMM0: XMMRegister = XMMRegister::from_code(double_code::XMM0);
pub const XMM1: XMMRegister = XMMRegister::from_code(double_code::XMM1);
pub const XMM2: XMMRegister = XMMRegister::from_code(double_code::XMM2);
pub const XMM3: XMMRegister = XMMRegister::from_code(double_code::XMM3);
pub const XMM4: XMMRegister = XMMRegister::from_code(double_code::XMM4);
pub const XMM5: XMMRegister = XMMRegister::from_code(double_code::XMM5);
pub const XMM6: XMMRegister = XMMRegister::from_code(double_code::XMM6);
pub const XMM7: XMMRegister = XMMRegister::from_code(double_code::XMM7);
pub const XMM8: XMMRegister = XMMRegister::from_code(double_code::XMM8);
pub const XMM9: XMMRegister = XMMRegister::from_code(double_code::XMM9);
pub const XMM10: XMMRegister = XMMRegister::from_code(double_code::XMM10);
pub const XMM11: XMMRegister = XMMRegister::from_code(double_code::XMM11);
pub const XMM12: XMMRegister = XMMRegister::from_code(double_code::XMM12);
pub const XMM13: XMMRegister = XMMRegister::from_code(double_code::XMM13);
pub const XMM14: XMMRegister = XMMRegister::from_code(double_code::XMM14);
pub const XMM15: XMMRegister = XMMRegister::from_code(double_code::XMM15);
/// AVX10 配置专用扩展寄存器
pub const XMM16: XMMRegister = XMMRegister::with_code(double_code::XMM16);
/// AVX10 配置专用扩展寄存器
pub const XMM17: XMMRegister = XMMRegister::with_code(double_code::XMM17);
/// AVX10 配置专用扩展寄存器
pub const XMM18: XMMRegister = XMMRegister::with_code(double_code::XMM18);
/// AVX10 配置专用扩展寄存器
pub const XMM19: XMMRegister = XMMRegister::with_code(double_code::XMM19);
/// AVX10 配置专用扩展寄存器
pub const XMM20: XMMRegister = XMMRegister::with_code(double_code::XMM20);
/// AVX10 配置专用扩展寄存器
pub const XMM21: XMMRegister = XMMRegister::with_code(double_code::XMM21);
/// AVX10 配置专用扩展寄存器
pub const XMM22: XMMRegister = XMMRegister::with_code(double_code::XMM22);
/// AVX10 配置专用扩展寄存器
pub const XMM23: XMMRegister = XMMRegister::with_code(double_code::XMM23);
/// AVX10 配置专用扩展寄存器
pub const XMM24: XMMRegister = XMMRegister::with_code(double_code::XMM24);
/// AVX10 配置专用扩展寄存器
pub const XMM25: XMMRegister = XMMRegister::with_code(double_code::XMM25);
/// AVX10 配置专用扩展寄存器
pub const XMM26: XMMRegister = XMMRegister::with_code(double_code::XMM26);
/// AVX10 配置专用扩展寄存器
pub const XMM27: XMMRegister = XMMRegister::with_code(double_code::XMM27);
/// AVX10 配置专用扩展寄存器
pub const XMM28: XMMRegister = XMMRegister::with_code(double_code::XMM28);
/// AVX10 配置专用扩展寄存器
pub const XMM29: XMMRegister = XMMRegister::with_code(double_code::XMM29);
/// AVX10 配置专用扩展寄存器
pub const XMM30: XMMRegister = XMMRegister::with_code(double_code::XMM30);
/// AVX10 配置专用扩展寄存器
pub const XMM31: XMMRegister = XMMRegister::with_code(double_code::XMM31);
/// no_dreg
pub const NO_DREG: XMMRegister = XMMRegister::no_reg();
// 注: 原版小写 xmm 别名 (xmm0../no_dreg) 同通用寄存器处理, 统一大写形态承接。

pub const YMM0: YMMRegister = YMMRegister::from_code(ymm_code::YMM0);
pub const YMM1: YMMRegister = YMMRegister::from_code(ymm_code::YMM1);
pub const YMM2: YMMRegister = YMMRegister::from_code(ymm_code::YMM2);
pub const YMM3: YMMRegister = YMMRegister::from_code(ymm_code::YMM3);
pub const YMM4: YMMRegister = YMMRegister::from_code(ymm_code::YMM4);
pub const YMM5: YMMRegister = YMMRegister::from_code(ymm_code::YMM5);
pub const YMM6: YMMRegister = YMMRegister::from_code(ymm_code::YMM6);
pub const YMM7: YMMRegister = YMMRegister::from_code(ymm_code::YMM7);
pub const YMM8: YMMRegister = YMMRegister::from_code(ymm_code::YMM8);
pub const YMM9: YMMRegister = YMMRegister::from_code(ymm_code::YMM9);
pub const YMM10: YMMRegister = YMMRegister::from_code(ymm_code::YMM10);
pub const YMM11: YMMRegister = YMMRegister::from_code(ymm_code::YMM11);
pub const YMM12: YMMRegister = YMMRegister::from_code(ymm_code::YMM12);
pub const YMM13: YMMRegister = YMMRegister::from_code(ymm_code::YMM13);
pub const YMM14: YMMRegister = YMMRegister::from_code(ymm_code::YMM14);
pub const YMM15: YMMRegister = YMMRegister::from_code(ymm_code::YMM15);
/// AVX10 配置专用扩展寄存器
pub const YMM16: YMMRegister = YMMRegister::with_code(ymm_code::YMM16);
/// AVX10 配置专用扩展寄存器
pub const YMM17: YMMRegister = YMMRegister::with_code(ymm_code::YMM17);
/// AVX10 配置专用扩展寄存器
pub const YMM18: YMMRegister = YMMRegister::with_code(ymm_code::YMM18);
/// AVX10 配置专用扩展寄存器
pub const YMM19: YMMRegister = YMMRegister::with_code(ymm_code::YMM19);
/// AVX10 配置专用扩展寄存器
pub const YMM20: YMMRegister = YMMRegister::with_code(ymm_code::YMM20);
/// AVX10 配置专用扩展寄存器
pub const YMM21: YMMRegister = YMMRegister::with_code(ymm_code::YMM21);
/// AVX10 配置专用扩展寄存器
pub const YMM22: YMMRegister = YMMRegister::with_code(ymm_code::YMM22);
/// AVX10 配置专用扩展寄存器
pub const YMM23: YMMRegister = YMMRegister::with_code(ymm_code::YMM23);
/// AVX10 配置专用扩展寄存器
pub const YMM24: YMMRegister = YMMRegister::with_code(ymm_code::YMM24);
/// AVX10 配置专用扩展寄存器
pub const YMM25: YMMRegister = YMMRegister::with_code(ymm_code::YMM25);
/// AVX10 配置专用扩展寄存器
pub const YMM26: YMMRegister = YMMRegister::with_code(ymm_code::YMM26);
/// AVX10 配置专用扩展寄存器
pub const YMM27: YMMRegister = YMMRegister::with_code(ymm_code::YMM27);
/// AVX10 配置专用扩展寄存器
pub const YMM28: YMMRegister = YMMRegister::with_code(ymm_code::YMM28);
/// AVX10 配置专用扩展寄存器
pub const YMM29: YMMRegister = YMMRegister::with_code(ymm_code::YMM29);
/// AVX10 配置专用扩展寄存器
pub const YMM30: YMMRegister = YMMRegister::with_code(ymm_code::YMM30);
/// AVX10 配置专用扩展寄存器
pub const YMM31: YMMRegister = YMMRegister::with_code(ymm_code::YMM31);
/// YMMRegister::no_reg
pub const NO_YMM_REG: YMMRegister = YMMRegister::no_reg();

// ===== XMM/YMM 名字表 =====
pub const XMM_REG_NAMES: [&str; 16] = [
  "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7", "xmm8", "xmm9", "xmm10", "xmm11",
  "xmm12", "xmm13", "xmm14", "xmm15",
];
pub const XMM_EXT_REG_NAMES: [&str; 16] = [
  "xmm16", "xmm17", "xmm18", "xmm19", "xmm20", "xmm21", "xmm22", "xmm23", "xmm24", "xmm25",
  "xmm26", "xmm27", "xmm28", "xmm29", "xmm30", "xmm31",
];
/// ALL_XMM_REGISTERS (AVX10 支, 32 项; 默认构建仅前 16 项有效)
pub const ALL_XMM_REG_NAMES: [&str; 32] = [
  "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7", "xmm8", "xmm9", "xmm10", "xmm11",
  "xmm12", "xmm13", "xmm14", "xmm15", "xmm16", "xmm17", "xmm18", "xmm19", "xmm20", "xmm21",
  "xmm22", "xmm23", "xmm24", "xmm25", "xmm26", "xmm27", "xmm28", "xmm29", "xmm30", "xmm31",
];
pub const YMM_REG_NAMES: [&str; 16] = [
  "ymm0", "ymm1", "ymm2", "ymm3", "ymm4", "ymm5", "ymm6", "ymm7", "ymm8", "ymm9", "ymm10", "ymm11",
  "ymm12", "ymm13", "ymm14", "ymm15",
];
pub const YMM_EXT_REG_NAMES: [&str; 16] = [
  "ymm16", "ymm17", "ymm18", "ymm19", "ymm20", "ymm21", "ymm22", "ymm23", "ymm24", "ymm25",
  "ymm26", "ymm27", "ymm28", "ymm29", "ymm30", "ymm31",
];
/// ALL_YMM_REGISTERS (AVX10 支, 32 项; 默认构建仅前 16 项有效)
pub const ALL_YMM_REG_NAMES: [&str; 32] = [
  "ymm0", "ymm1", "ymm2", "ymm3", "ymm4", "ymm5", "ymm6", "ymm7", "ymm8", "ymm9", "ymm10", "ymm11",
  "ymm12", "ymm13", "ymm14", "ymm15", "ymm16", "ymm17", "ymm18", "ymm19", "ymm20", "ymm21",
  "ymm22", "ymm23", "ymm24", "ymm25", "ymm26", "ymm27", "ymm28", "ymm29", "ymm30", "ymm31",
];

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
pub const K_SCRATCH_DOUBLE_REG: XMMRegister = XMM15;
pub const K_SCRATCH_SIMD256_REG: YMMRegister = YMM15;
pub const K_FP_RETURN_REGISTER0: XMMRegister = XMM0;

pub const ALLOCATABLE_DOUBLE_REGISTERS: [XMMRegister; 15] = [
  XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7, XMM8, XMM9, XMM10, XMM11, XMM12, XMM13, XMM14,
];

// ===== 跨表静态断言 (逐条对齐头内 static_assert / arraysize 不变式) =====
const _: () = assert!(reg_code::K_REG_AFTER_LAST as usize == K_NUM_REGS);
const _: () = assert!(GENERAL_REG_NAMES.len() == K_NUM_REGS);
const _: () = assert!(double_code::K_DOUBLE_AFTER_LAST == ymm_code::K_YMM_AFTER_LAST); // kDoubleAfterLast == kYMMAfterLast
const _: () = assert!(double_code::K_DOUBLE_AFTER_LAST_AVX10 == ymm_code::K_YMM_AFTER_LAST_AVX10);
const _: () = assert!(ALL_XMM_REG_NAMES.len() == ALL_YMM_REG_NAMES.len());
const _: () = assert!(XMM_REG_NAMES.len() == YMM_REG_NAMES.len());
const _: () = assert!(XMM_EXT_REG_NAMES.len() == YMM_EXT_REG_NAMES.len());
const _: () = assert!(
  ALLOCATABLE_GENERAL_REGISTERS_WITH_COMPRESSION.len()
    == ALWAYS_ALLOCATABLE_GENERAL_REGISTERS.len()
);
const _: () = assert!(
  ALLOCATABLE_GENERAL_REGISTERS_WITHOUT_COMPRESSION.len()
    == ALWAYS_ALLOCATABLE_GENERAL_REGISTERS.len()
      + MAYBE_ALLOCATABLE_GENERAL_REGISTERS_WITHOUT_COMPRESSION.len()
);
const _: () = assert!(K_C_ARG_REGS_WIN.len() == 4 && K_C_ARG_REGS_SYSV.len() == 6);
const _: () = assert!(K_REGISTER_PASSED_ARGUMENTS_WIN == 4);
const _: () = assert!(K_REGISTER_PASSED_ARGUMENTS_SYSV == 6);
const _: () = assert!(C_CALL_CALLEE_SAVE_FP_REGISTERS_SYSV.is_empty());
const _: () = assert!(ALLOCATABLE_DOUBLE_REGISTERS.len() == 15);
