// 由 tool/src/d8/memory-access-information.h.js 从 v8/src/d8/memory-access-information.{h,cc} 生成; 修订就地改, 复跑覆盖
// 内存访问信息: 枚举 + 宽度表 + 寄存器别名表 + 助记符表

//! 内存访问信息 — 原版 v8::MemoryAccessInformation 族 (观察点命中侧解析)
//!
//! 语义: 反汇编文本指令 → 访问种类/宽度/目标寄存器; 用于观察点命中时
//! 判定读写与读值落点。门控同 V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT
//! (V8_ENABLE_MEMORY_CORRUPTION_API && V8_OS_LINUX && V8_TARGET_ARCH_X64);
//! arm64 目标整体跳过。

/// 寄存器值类型 (原版 using reg_value_type = unsigned long long)
pub type RegValue = u64;

/// 访问种类 (原版 enum Kind, 5 值)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
  /// 原版 kRead
  Read,
  /// 原版 kWrite
  Write,
  /// 原版 kCmp
  Cmp,
  /// 原版 kCmpxchg
  Cmpxchg,
  /// 原版 kRepMovs
  RepMovs,
}

/// 扩展方式 (原版 enum Extension, 3 值)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
  /// 原版 kZeroExtend
  ZeroExtend,
  /// 原版 kSignExtend
  SignExtend,
  /// 原版 kNoExtend
  NoExtend,
}

/// 内存访问信息 (原版 struct MemoryAccessInformation, 7 字段 1:1)
#[derive(Debug, Clone, Copy)]
pub struct MemoryAccessInformation {
  pub kind: Kind,
  /// kRead 时二选一: user_regs_struct 内目标 GPR (原版 result_reg)
  pub result_reg: Option<*mut RegValue>,
  /// XMM 寄存器下标 0-15, 非 XMM 为 -1 (原版 xmm_reg_index)
  pub xmm_reg_index: i32,
  /// AVX-512 掩码寄存器下标 0-7, 非掩码为 -1 (原版 k_reg_index)
  pub k_reg_index: i32,
  /// 访问宽度, 默认 8 (原版 access_width)
  pub access_width: i32,
  /// 目标寄存器宽度 — 仅 movsx/movzx 与 access_width 不同 (原版 dest_width)
  pub dest_width: i32,
  pub extension: Extension,
}

impl MemoryAccessInformation {
  /// 创建只读内存访问信息默认配置
  pub const fn default_read(access_width: i32) -> Self {
    Self {
      kind: Kind::Read,
      result_reg: None,
      xmm_reg_index: -1,
      k_reg_index: -1,
      access_width,
      dest_width: access_width,
      extension: Extension::NoExtend,
    }
  }

  /// 创建写内存访问信息默认配置
  pub const fn default_write(access_width: i32) -> Self {
    Self {
      kind: Kind::Write,
      result_reg: None,
      xmm_reg_index: -1,
      k_reg_index: -1,
      access_width,
      dest_width: access_width,
      extension: Extension::NoExtend,
    }
  }
}

/// 宽度后缀表 (原版 width_from_suffix: b=1, w=2, l/d=4, q=8, 其他=8)
pub const WIDTH_FROM_SUFFIX: [(&str, i32); 5] = [("b", 1), ("w", 2), ("l", 4), ("d", 4), ("q", 8)];

/// 根据指令后缀获取访问宽度 (b=1, w=2, l/d=4, q=8, 默认 8)
pub fn width_from_suffix(suffix: &str) -> i32 {
  WIDTH_FROM_SUFFIX
    .iter()
    .find(|(s, _)| *s == suffix)
    .map(|(_, w)| *w)
    .unwrap_or(8)
}

/// 匹配通用寄存器别名对应的 64 位基名 (例如 "eax" -> "rax")
pub fn find_gpr_base(reg_name: &str) -> Option<&'static str> {
  GPR_ALIASES.iter().find_map(|(base, aliases)| {
    if aliases.contains(&reg_name) {
      Some(*base)
    } else {
      None
    }
  })
}

/// 判定助记符是否为比较类指令
pub fn is_cmp_mnemonic(mnem: &str) -> bool {
  CMP_MNEMONICS.contains(&mnem)
}

/// GPR 别名表 (原版 match_reg 链 16 组, 64 位名 → 别名集)
pub const GPR_ALIASES: [(&str, [&str; 5]); 16] = [
  ("rax", ["rax", "eax", "ax", "al", "ah"]),
  ("rcx", ["rcx", "ecx", "cx", "cl", "ch"]),
  ("rdx", ["rdx", "edx", "dx", "dl", "dh"]),
  ("rbx", ["rbx", "ebx", "bx", "bl", "bh"]),
  ("rsp", ["rsp", "esp", "sp", "spl", ""]),
  ("rbp", ["rbp", "ebp", "bp", "bpl", ""]),
  ("rsi", ["rsi", "esi", "si", "sil", ""]),
  ("rdi", ["rdi", "edi", "di", "dil", ""]),
  ("r8", ["r8", "r8d", "r8w", "r8b", ""]),
  ("r9", ["r9", "r9d", "r9w", "r9b", ""]),
  ("r10", ["r10", "r10d", "r10w", "r10b", ""]),
  ("r11", ["r11", "r11d", "r11w", "r11b", ""]),
  ("r12", ["r12", "r12d", "r12w", "r12b", ""]),
  ("r13", ["r13", "r13d", "r13w", "r13b", ""]),
  ("r14", ["r14", "r14d", "r14w", "r14b", ""]),
  ("r15", ["r15", "r15d", "r15w", "r15b", ""]),
];

/// 可识别助记符表 (原版 match_mnem 列表, 38 项)
pub const MNEMONICS: [&str; 38] = [
  "mov",
  "sub",
  "add",
  "vmov",
  "or",
  "xor",
  "and",
  "vcmp",
  "vpcmp",
  "pcmp",
  "vadd",
  "vsub",
  "vmul",
  "vdiv",
  "padd",
  "psub",
  "pmul",
  "vor",
  "vxor",
  "vand",
  "por",
  "pxor",
  "pand",
  "vpbroadcast",
  "vpinsr",
  "vpextr",
  "vpmov",
  "pmov",
  "vshuf",
  "pshuf",
  "vunpck",
  "punpck",
  "vpack",
  "pack",
  "vblend",
  "pblend",
  "lddqu",
  "vlddqu",
];

/// 比较类指令族 (cmp/test/comi/ucomi/vcomi/vucomi/ptest/vptest → kCmp;
/// 其中 cmpxchg → kCmpxchg)
pub const CMP_MNEMONICS: [&str; 8] = [
  "cmp", "test", "comi", "ucomi", "vcomi", "vucomi", "ptest", "vptest",
];

// —— 签名骨架 ——
// parse_memory_access_information_from_instruction(insn_pos: &CStr, regs: &mut UserRegs)
//   -> MemoryAccessInformation
//   "rep movs" → kRepMovs (后缀定宽) / 其他 rep 前缀 → kWrite (宽 8, 忽略语义)
//   movzx/movsx → 双后缀 (源宽+目标宽, mnem_len>=7)
//   'l' 后缀 → kZeroExtend (x64 32 位子寄存器写全清零语义)
//   左操作数为内存 → kWrite; GPR/XMM/掩码 → kRead
//   未识别 → FATAL

// 平台分支台账:
//   V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT 门控 (x64+Linux) — arm64 跳过
//   user_regs_struct (sys/user.h) — Linux 专有, FFI 承接
