// v8-regexp-exec: IrregexpInterpreter — v8/src/regexp/regexp-interpreter.{h,cc}
// 骨架票 (REG-1b1) + 操作数基座与 P-3a 批段票 (REG-1b2)
// + P-3b/P-3c/P-3d 38 handler 落地票 (REG-1b3)
// 由 tool/src/regexp/regexp-interpreter.h.js / regexp-interpreter.cc.js 对数审计
// (骨架: regexp_interpreter.rs.skel + regexp_interpreter_cc.skel; 本文件为 Developer 就地修订版)
//
// ═══════════════════════════════════════════════════════════════════════════
// 本票落地面 (非 isolate 核心语义, 逐方法 1:1 对照原版 file:line):
// - Result 枚举 5 值                    h:21-27  (值源 regexp.h:150-154)
// - CallOrigin 枚举                     regexp.h:112-113 (h:55 签名依赖, 双引用)
// - Bytecode 枚举 61 项 + 纯读函数族    regexp-bytecodes.h:90/93/188/245/336-373
// - 派发寻址纯函数 ADVANCE/SET_PC       cc:367-373, 位置变更 cc:376-381
// - BacktrackStack                      cc:129-165
// - InterpreterRegisters                cc:172-213 (输出寄存器 [0,output_count) =
//   捕获 {start,end} 对, 内部寄存器 [output_count,total) — cc:167-171 注释语义)
// - BackRefMatchesNoCase (Latin1 重载)  cc:63-81 (原版带 Isolate* 形参但未使用,
//   落地删参; Unicode 重载依赖宏汇编器 compare, 见推迟)
// - BitsPerChar/Load2/Load4Characters   cc:105-122
// - CheckBitInTable                     cc:321-326 (kTableMask 双引用
//   regexp-macro-assembler.h:48-50)
// - IndexIsInBounds                     cc:329-332
// - CheckSpecialClassRanges             cc:418-497 (word_character_map 参数注入,
//   原版宏汇编器静态 256 表 regexp-macro-assembler.h:405-406/435/439 — 表内容
//   已随 REG-1b3 落 K_WORD_CHARACTER_MAP, ≡regexp-macro-assembler.cc:720-761)
//
// ═══════════════════════════════════════════════════════════════════════════
// REG-1b2 落地面 (操作数基座 + RawMatch 派发主体 + P-3a 批段 23 handler):
// - OperandType 全集 11 项 + 宽度/对齐/
//   kIsBasic                         regexp-bytecodes.h:30-51 +
//                                    regexp-bytecodes-inl.h:23-69 (traits)
// - 操作数限幅常量 (Offset/BoundsCheckOffset/Register)  bytecodes.h:39-42 +
//   macro-assembler.h:42-46
// - CalculateAlignedOffsets + kSize     inl.h:107-136/149-151 (4 字节块对齐排布,
//   K_BYTECODE_ALIGNMENT h:348); 61 项布局表 BYTECODE_LAYOUTS ≡ inl.h:312-316
//   kBytecodeSizes + 各 BytecodeOperands<bc> 特化 (inl.h:292-303)
// - Bytecodes::Size / Operands::Offset/Size/Type/kCount   inl.h:177-188/357-365
// - 操作数读取 Get<op> (basic reinterpret_cast + BitTable 指针分支)  inl.h:244-273
// - INIT 解码基座 (cc:402-414) → handler 侧按 bytecode_operands::<bc>::NAME 下标
//   取值 (≡ 原版 using Operands/using enum Operand + Get<NAME>)
// - Bytecode::name 面? 未落 — kBytecodeNames (inl.h:307-310) 属 [P-4] 诊断批
// - RawMatch 派发主体 (cc:501-576/568-575 循环 + DISPATCH) → RawMatch::run,
//   switch 等价物 (cc:349-357 兜底轨; computed-goto cc:338-345/510-556 性能轨不落地,
//   裁定见票面) ; 帧状态 (pc/current/current_char/backtrack_count) +
//   registers/stack/output/code/subject 注入面见 RawMatch 文档
// - P-3a 23 handler (见下方批段行号表) 全部落地, 语义逐行比对原版;
//   isolate 消费点 (MaybeThrowStackOverflow cc:227-234 / HandleInterrupts
//   cc:260-319 / counters cc:663-664,668-669) 以 Hooks trait 注入承接 [P-2]
// - 票面差异登记 (实测原版, 禁按票单臆造):
//   ① 票单所述 kOpSizeNone/One/Two/Four + ReadOneOperand 为上游 2024 前旧布局,
//      本仓库版 (inl.h 2025) 全仓 0 命中; 现版按操作数类型自身对齐 + 4 字节块
//      不跨界自动排布 (inl.h:123-129), 本票按现版 1:1。
//   ② 票单所述 "Backtrack 空栈 ⇒ Fail 语义" 与原版不符: 空栈 pop 走
//      BacktrackStack::peek 的 SBXCHECK(!data_.empty()) (cc:139-142) = fatal,
//      非 FAILURE 返回; 已按原版承接 (生成器保证 Backtrack 前必有 PushBacktrack,
//      bytecode-generator.cc:307-311/324-326)。
//   ③ AdvanceCpAndGoto (cc:682-686) 与 bits_per_char 无关 — 原版 by 为 Char
//      元素偏移, 一/两字节两态同码; bits_per_char (cc:105-108) 只消费于
//      Load2/Load4 移位量 (已落纯函数)。两态等价性由单测断言。
//   ④ PushBacktrack 的 limit 判定在本批 (backtrack_stack.push 返回 bool,
//      cc:587-590), Throw 本体 (cc:215-223) 仍属 [P-2] — 见 Hooks。
//
// ═══════════════════════════════════════════════════════════════════════════
// REG-1b3 落地面 (P-3b/P-3c/P-3d 38 handler 全量直译, RegExpExec 内部执行面):
// - exec_char_ops 22 项 (Load*/Check* 字符装载/比较/表查/类集, cc:696-1009) /
//   exec_backref_ops 9 项 (反向引用对/锚点, cc:880-990) / exec_skip_ops 7 项
//   (peephole skip-until, cc:1010-1196) 三组派发, 并入 exec() 六组 or_else 链;
//   deferred_handler_of 台账函数随 38 项全落而退役 (派发失配即 fatal, 禁静默)
// - Subject trait 扩面: ONE_BYTE 关联常量 (≡ 原版 template <Char> 的
//   sizeof(Char), cc:421) + load2_at/load4_at (Load2/4Characters 两态分发,
//   cc:110-122) + chars_equal (CompareCharsEqual, utils.h:611-617 元素相等) +
//   backref_matches_no_case (cc:46-81 双重载: &[u8] → Some(Latin1 核);
//   &[u16] → None 显式登记 P-3c-icu 运行态通道)
// - K_WORD_CHARACTER_MAP: [u8; 256] 编译期常量 ≡
//   RegExpMacroAssembler::word_character_map_ (regexp-macro-assembler.cc:720-
//   761: [0-9A-Za-z_] → 0xFF 余 0x00), 供 CheckSpecialClassRanges handler 的
//   word/notword 分支 (原版宏汇编器静态表读取, cc:440/442/453/455)
// - 唯一运行态推迟面: NoCase 四 handler 两字节态 Step::NotImplemented
//   { batch: "P-3c-icu" } (ICU 折叠硬墙, 见 [P-3c] 段); 其余出口面不变
//   (Done/IllegalResultCode/IllegalCode/StepLimitExceeded)
//
// ═══════════════════════════════════════════════════════════════════════════
// 推迟清单 (isolate/CodeStream/handles/frame 耦合, 不入本票; 归批建议见下,
// 均为 regexp-interpreter.cc 除标注外):
// [P-1 入口批]
// - IrregexpInterpreter::Match                    cc:1215-1296 + h:67-70
//   (v8_flags.regexp_tier_up / JSRegExp::AsRegExpFlags / IrRegExpData 直读 /
//    Utils::AdvanceStringIndex — 后者已落 regexp_utils.rs, 接驳点)
// - IrregexpInterpreter::MatchInternal            cc:1298-1338 + h:58-64
//   (String::FlatContent / DisallowGarbageCollection)
// - IrregexpInterpreter::MatchForCallFromJs       cc:1348-1376 + h:51-56
//   (ExitSandboxScope / SbxCast / MarkedForTierUp; COMPILING_IRREGEXP_FOR_
//   EXTERNAL_EMBEDDER 守卫 cc:1340/1378 排除, ENABLE_SANDBOX_HARDWARE cc:1344
//   分支排除)
// - IrregexpInterpreter::MatchForCallFromRuntime  cc:1380-1387 + h:31-36
// [P-2 栈溢出/中断批]
// - ThrowStackOverflow                            cc:215-223
// - MaybeThrowStackOverflow                       cc:227-234 (RawMatch 各 push
//   handler 的 MaybeThrowStackOverflow 调用点随 P-3a 一起落)
// - UpdateCodeAndSubjectReferences                cc:236-256
// - HandleInterrupts                              cc:260-319 (StackLimitCheck /
//   stack_guard / IsExceptionHole; RegExp::CallOrigin 分支语义已在本文件
//   Result 文档区登记 h:38-50)
// [P-3 opcode handler 主体批 — RawMatch cc:501-1196 全体 61 handler, 4 批已全部落地]:
// 前置: 操作数解码基座 INIT 宏 cc:402-414 + BytecodeOperands<bc> 特化
//   (regexp-bytecodes-inl.h) 与 computed-goto 派发面 cc:338-358/510-556
//   (threaded dispatch 为性能选项, rust 落地取 switch 等价物, cc:349-357 注释)
//   → 前置基座已随 REG-1b2 落本文件 (见上"REG-1b2 落地面")
// - P-3a 寄存器/回溯栈/控制/终止 (23) — 已落地 REG-1b2 (rs 侧 exec_stack_ops/
//   exec_reg_ops/exec_ctrl_ops 三组 + run 循环, 逐名行号台账见 cc.skel):
//   Break cc:577, PushCurrentPosition 578,
//   PushBacktrack 585, PushRegister 592, SetRegister 600, ClearRegisters 605,
//   AdvanceRegister 613, WriteCurrentPositionToRegister 618,
//   ReadCurrentPositionFromRegister 623, WriteStackPointerToRegister 628,
//   ReadStackPointerFromRegister 633, PopCurrentPosition 638, Backtrack 643,
//   PopRegister 657, Fail 662, Succeed 667, AdvanceCurrentPosition 673, GoTo 678,
//   AdvanceCpAndGoto 682, CheckFixedLengthLoop 687, IfRegisterLT 856,
//   IfRegisterGE 864, IfRegisterEqPos 872
//   (Fail/Succeed 的 counters()->regexp_backtracks() cc:663-664/668-669 为
//    isolate 面, 落批时参数注入或暂记)
// - P-3b 字符装载/比较/表查/类集 (22) — 已落地 REG-1b3 (rs 侧 exec_char_ops 组):
//   LoadCurrentCharacter 696,
//   LoadCurrentCharacterUnchecked 705, Load2CurrentChars 711,
//   Load2CurrentCharsUnchecked 720, Load4CurrentChars 726,
//   Load4CurrentCharsUnchecked 736, Check4Chars 743, CheckCharacter 751,
//   CheckNot4Chars 759, CheckNotCharacter 767, AndCheck4Chars 775,
//   CheckCharacterAfterAnd 783, AndCheckNot4Chars 791,
//   CheckNotCharacterAfterAnd 799, CheckNotCharacterAfterMinusAnd 807,
//   CheckCharacterInRange 816, CheckCharacterNotInRange 824,
//   CheckBitInTable 832 (核已落本文件 cc:321-326), CheckCharacterLT 840,
//   CheckCharacterGT 848, CheckPosition 991, CheckSpecialClassRanges 1000
//   (核已落本文件 cc:418-497; 其 word_character_map 参数注入面已随本批落
//   K_WORD_CHARACTER_MAP ≡ regexp-macro-assembler.cc:720-761 256 项静态表)
//   装载面经 Subject::load2_at/load4_at 两态分发 — 原版 Load4Characters(uc16)
//   cc:120-122 UNREACHABLE 承接为 v8_fatal (本文件 load_4_characters_two_byte)
// - P-3c 反向引用/锚点 (9) — 已落地 REG-1b3 (rs 侧 exec_backref_ops 组):
//   CheckNotBackRef 880, CheckNotBackRefBackward 894,
//   CheckNotBackRefNoCaseUnicode 908, CheckNotBackRefNoCase 922,
//   CheckNotBackRefNoCaseUnicodeBackward 937, CheckNotBackRefNoCaseBackward 952,
//   CheckAtStart 967, CheckNotAtStart 975, SetCurrentPositionFromEnd 983
//   运行态推迟通道 (唯一残留, 禁伪装): 四个 NoCase handler 的**两字节
//   subject 态**依赖 BackRefMatchesNoCase(uc16) cc:46-61 →
//   RegExpMacroAssembler::CaseInsensitiveCompare{NonUnicode cc:60,
//   Unicode cc:91} (regexp-macro-assembler.cc) → ICU CaseFolding
//   EquivalenceKey (special-case.h:66-80) + 构建期生成物 special-case.cc
//   (IgnoreSet) 本仓库缺席 → Subject::backref_matches_no_case 对 &[u16]
//   返回 None, handler 显式 Halt Step::NotImplemented { batch: "P-3c-icu" };
//   单字节态走本文件已落 Latin1 核 (cc:63-81, unicode 标志无差别 — 原版
//   cc:65 注释), 四 handler 全量执行
// - P-3d skip-until peephole (7) — 已落地 REG-1b3 (rs 侧 exec_skip_ops 组):
//   SkipUntilChar 1010, SkipUntilCharAnd 1023,
//   SkipUntilBitInTable 1036, SkipUntilGtOrNotBitInTable 1049,
//   SkipUntilCharOrChar 1066, SkipUntilOneOfMasked 1086,
//   SkipUntilOneOfMasked3 1111
//   原版 SKIP_AND_CHECK/CHECK_AND_ADVANCE 类 peephole 均为 while 界内循环 +
//   装载比较跳转, 1:1 直译; SkipUntilOneOfMasked3 的 bc0..bc7 复合体按
//   cc:1111-1196 控制流逐臂复刻 (外层 while(true) + 内层 skip 循环 break)
// [P-4 诊断批]
// - MaybeTraceInterpreter / ENABLE_DISASSEMBLER   cc:83-103 (v8_flags.
//   trace_regexp_bytecodes + RegExpBytecodeDisassembleSingle 越界批)
// - Match 内 trace 打印块                          cc:1241-1255
// ═══════════════════════════════════════════════════════════════════════════

use std::mem::{size_of, transmute};

use v8_base::logging::v8_fatal;
use v8_common::{K_BITS_PER_BYTE, K_BITS_PER_BYTE_LOG2, K_INT32_SIZE};
use v8_regexp_syntax::StandardCharacterSet;

use crate::regexp_stack::K_MAXIMUM_STACK_SIZE;

/// 原版 SBXCHECK 族承接: 原版 debug/release 均生效 (sandbox check),
/// 失败走 v8-base fatal (abort 语义, 非 panic 路径; 与 REG-1a 冻结件一致)
#[inline]
fn sbx_check(cond: bool, msg: &str) {
  if !cond {
    v8_fatal(format_args!(
      "SBXCHECK({msg}) failed (regexp-interpreter.cc)"
    ));
  }
}

// ── 结果协议 (h:21-27; 值源 regexp.h:150-154) ──────────────────────────────

/// 原版 IrregexpInterpreter::Result (h:21-27) = RegExp::kInternalRegExp*
/// (regexp.h:150-154)。MatchForCallFromJs/Match 系的 int 返回值共享本协议:
/// 正数 = 命中个数 (Match h:1295 num_matches), 其余为本枚举值 (h:38-50 注释:
/// StackOverflow→EXCEPTION [kFromRuntime 自抛 / kFromJs 由调用方抛, cc:215-234];
/// 中断/tier-up→RETRY; results 数组仅在 ≥1 命中时被改写)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Result {
  /// 原版 FAILURE = RegExp::kInternalRegExpFailure (h:22; regexp.h:150 = 0)
  Failure = 0,
  /// 原版 SUCCESS = RegExp::kInternalRegExpSuccess (h:23; regexp.h:151 = 1)
  Success = 1,
  /// 原版 EXCEPTION = RegExp::kInternalRegExpException (h:24; regexp.h:152 = -1)
  Exception = -1,
  /// 原版 RETRY = RegExp::kInternalRegExpRetry (h:25; regexp.h:153 = -2)
  Retry = -2,
  /// 原版 FALLBACK_TO_EXPERIMENTAL (h:26; regexp.h:154 = -3)
  FallbackToExperimental = -3,
}

impl Result {
  /// 原版 kInternalRegExpSmallestResult (regexp.h:155) — 结果协议下界
  pub const SMALLEST: i32 = -3;

  /// 原版 `static_cast<IrregexpInterpreter::Result>(int)` 的承接 (cc:646
  /// Backtrack 的 return_code; h:38-50 int 返回值协议) — 仅 5 个协议值可表示,
  /// 域外值由调用方显式处理 (原版盲转 = 未定义, Rust 禁伪造)
  #[must_use]
  pub const fn from_code(code: i32) -> Option<Result> {
    match code {
      0 => Some(Self::Failure),
      1 => Some(Self::Success),
      -1 => Some(Self::Exception),
      -2 => Some(Self::Retry),
      -3 => Some(Self::FallbackToExperimental),
      _ => None,
    }
  }
}

/// 原版 RegExp::CallOrigin (regexp.h:111-114) — h:55/h:63/h:70 签名依赖,
/// 决定栈溢出的抛出归属 (cc:217 CHECK kFromRuntime; cc:272-283 kFromJs 直返
/// EXCEPTION/RETRY, 由调用方建异常)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CallOrigin {
  /// kFromRuntime = 0 (regexp.h:112)
  FromRuntime = 0,
  /// kFromJs = 1 (regexp.h:113)
  FromJs = 1,
}

// ── opcode 枚举与纯读取面 (regexp-bytecodes.h:90-373) ──────────────────────

/// 原版 enum class Bytecode : uint8_t (regexp-bytecodes.h:336-344,
/// 由 REGEXP_BYTECODE_LIST h:330-334 四表展开; 编号即枚举序, 与
/// cc 端 BYTECODE handler 名一一对应 — 对数见 regexp-interpreter.cc.js)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Bytecode {
  /// INVALID_BYTECODE_LIST (bytecodes.h:90) — 必须为 0 (h:88 注释: 跳转 0
  /// 视为非法), RawMatch 侧 UNREACHABLE (cc:577)
  Break = 0,
  // BASIC_BYTECODE_LIST (bytecodes.h:93-186) ↓1..=35
  PushCurrentPosition = 1,
  PushBacktrack = 2,
  WriteCurrentPositionToRegister = 3,
  ReadCurrentPositionFromRegister = 4,
  WriteStackPointerToRegister = 5,
  ReadStackPointerFromRegister = 6,
  SetRegister = 7,
  ClearRegisters = 8,
  AdvanceRegister = 9,
  PopCurrentPosition = 10,
  PushRegister = 11,
  PopRegister = 12,
  Fail = 13,
  Succeed = 14,
  AdvanceCurrentPosition = 15,
  GoTo = 16,
  LoadCurrentCharacter = 17,
  CheckPosition = 18,
  CheckSpecialClassRanges = 19,
  CheckCharacter = 20,
  CheckNotCharacter = 21,
  CheckCharacterAfterAnd = 22,
  CheckNotCharacterAfterAnd = 23,
  CheckNotCharacterAfterMinusAnd = 24,
  CheckCharacterInRange = 25,
  CheckCharacterNotInRange = 26,
  CheckCharacterLT = 27,
  CheckCharacterGT = 28,
  IfRegisterLT = 29,
  IfRegisterGE = 30,
  IfRegisterEqPos = 31,
  CheckAtStart = 32,
  CheckNotAtStart = 33,
  CheckFixedLengthLoop = 34,
  SetCurrentPositionFromEnd = 35,
  // SPECIAL_BYTECODE_LIST (bytecodes.h:188-243) ↓36..=53
  Backtrack = 36,
  LoadCurrentCharacterUnchecked = 37,
  CheckBitInTable = 38,
  Load2CurrentChars = 39,
  Load2CurrentCharsUnchecked = 40,
  Load4CurrentChars = 41,
  Load4CurrentCharsUnchecked = 42,
  Check4Chars = 43,
  CheckNot4Chars = 44,
  AndCheck4Chars = 45,
  AndCheckNot4Chars = 46,
  AdvanceCpAndGoto = 47,
  CheckNotBackRef = 48,
  CheckNotBackRefNoCase = 49,
  CheckNotBackRefNoCaseUnicode = 50,
  CheckNotBackRefBackward = 51,
  CheckNotBackRefNoCaseBackward = 52,
  CheckNotBackRefNoCaseUnicodeBackward = 53,
  // PEEPHOLE_BYTECODE_LIST (bytecodes.h:245-328) ↓54..=60
  SkipUntilBitInTable = 54,
  SkipUntilCharAnd = 55,
  SkipUntilChar = 56,
  SkipUntilCharOrChar = 57,
  SkipUntilGtOrNotBitInTable = 58,
  SkipUntilOneOfMasked = 59,
  SkipUntilOneOfMasked3 = 60,
}

/// 原版 Bytecodes::kCount (bytecodes.h:355) = kLast+1, 实测 61
pub const K_BYTECODE_COUNT: u8 = Bytecode::SkipUntilOneOfMasked3 as u8 + 1;
/// 原版 kBytecodeAlignment (bytecodes.h:348) — opcode 4 字节对齐
pub const K_BYTECODE_ALIGNMENT: usize = 4;
/// 原版 kPaddedBytecodeCount (cc:517-518) = bit_ceil(kCount), 实测 64
pub const K_PADDED_BYTECODE_COUNT: usize = (K_BYTECODE_COUNT as usize).next_power_of_two();
/// 原版 kBytecodeMask (cc:519) = kPaddedBytecodeCount - 1 — DECODE()
/// (cc:340-345) 查派发表前掩码, 保证 OOB 不可能的结构不变式
pub const K_BYTECODE_MASK: usize = K_PADDED_BYTECODE_COUNT - 1;
// 原版 static_assert(uint8_max >= kBytecodeMask) (cc:520)
const _: () = assert!(u8::MAX as usize >= K_BYTECODE_MASK);
// 原版 dispatch 表尾自动填 kBreak (cc:528-530/548-552) 的可行性前提:
const _: () = assert!(K_PADDED_BYTECODE_COUNT > K_BYTECODE_COUNT as usize);

/// 原版 regexp::Bytecodes 纯函数子集 (bytecodes.h:356-373)。
/// Size()/Flags()/Name()/DispatchOnBytecode() 与操作数解码基座 (cc:402-414)
/// 属 P-3 前置批, 推迟登记见模块头。
impl Bytecode {
  /// 原版 Bytecodes::ToByte (bytecodes.h:356-358)
  #[inline]
  #[must_use]
  pub const fn to_byte(self) -> u8 {
    self as u8
  }

  /// 原版 Bytecodes::IsValid (bytecodes.h:371)
  #[inline]
  #[must_use]
  pub const fn is_valid(byte: u8) -> bool {
    byte < K_BYTECODE_COUNT
  }

  /// 原版 Bytecodes::FromByte (bytecodes.h:359-363) — 原 DCHECK(IsValid),
  /// Rust 类型无法表示非法值, 以 Option 承接 DCHECK 面
  #[inline]
  #[must_use]
  pub const fn from_byte(byte: u8) -> Option<Bytecode> {
    if Self::is_valid(byte) {
      Some(unsafe { transmute::<u8, Bytecode>(byte) })
    } else {
      None
    }
  }

  /// 原版 Bytecodes::FromPtr (bytecodes.h:365-370): 从 pc 指向的 4 字节对齐区
  /// 取 opcode 字节 (小端语义等价 *ptr)。调用方须保证 pc 可读且
  /// 对齐 (原版 DCHECK IsAligned, bytecodes.h:367)。
  ///
  /// # Safety
  /// pc 必须指向 code_array 内有效的 opcode 起始字节。
  #[inline]
  #[must_use]
  pub unsafe fn from_ptr(pc: *const u8) -> Option<Bytecode> {
    Self::from_byte(unsafe { *pc })
  }

  /// 原版 Bytecodes::IsValidJumpTarget (bytecodes.h:372-374) — kBreak
  /// (编号 0) 不是合法跳转目标
  #[inline]
  #[must_use]
  pub const fn is_valid_jump_target(byte: u8) -> bool {
    Self::is_valid(byte) && byte != Self::Break as u8
  }
}

// ── 派发/位置寻址纯函数 (cc:367-381 宏的 1:1 函数化) ───────────────────────

/// 原版 ADVANCE() (cc:367-369): next_pc = pc + Bytecodes::Size(current_bc)。
/// pc 以 code_base 相对偏移表示 (SET_PC_FROM_OFFSET 同为偏移, 帧内相对寻址)。
#[inline]
#[must_use]
pub const fn advance_pc(pc: u32, current_bc_size: u8) -> u32 {
  pc + current_bc_size as u32
}

/// 原版 SET_PC_FROM_OFFSET(offset) (cc:371-373): next_pc = code_base + offset —
/// 跳转目标即字节码绝对偏移 (相对 code_base), 与 pc 偏移表示同构
#[inline]
#[must_use]
pub const fn pc_from_offset(offset: u32) -> u32 {
  offset
}

/// 原版 SET_CURRENT_POSITION(value) (cc:376-380) — DCHECK IsInRange
/// (current, 0, subject.length()) 以 debug_assert 承接
#[inline]
#[must_use]
pub fn set_current_position(value: i32, subject_len: i32) -> i32 {
  debug_assert!((0..=subject_len).contains(&value));
  value
}

/// 原版 ADVANCE_CURRENT_POSITION(by) (cc:381) = SET_CURRENT_POSITION(current+by)
#[inline]
#[must_use]
pub fn advance_current_position(current: i32, by: i32, subject_len: i32) -> i32 {
  set_current_position(current + by, subject_len)
}

// ── 回溯栈 (cc:129-165) ────────────────────────────────────────────────────

/// 原版 BacktrackStack (cc:129-165) — 回溯栈通用化: 同时存 pc(字节码偏移)与
/// 整数值 (cc:126-128 注释); 'sp' = 首个空槽下标 (cc:149)
pub struct BacktrackStack {
  /// 原版 data_ (cc:162) base::SmallVector<int, 64> → Vec<i32>
  /// (静态前置容量语义以 with_capacity 承接)
  data_: Vec<i32>,
}

impl Default for BacktrackStack {
  fn default() -> Self {
    Self::new()
  }
}

impl BacktrackStack {
  /// 原版 kStaticCapacity (cc:159) = 64 — 静态内联区容量 (SmallVector 前置)
  pub const K_STATIC_CAPACITY: usize = 64;
  /// 原版 ValueT = int (cc:161) → i32 (SBXCHECK sizeof 一致, cc:185 同款前提)
  const VALUE_SIZE: usize = size_of::<i32>();
  /// 原版 kMaxSize (cc:164) = Stack::kMaximumStackSize / sizeof(ValueT)
  pub const K_MAX_SIZE: usize = K_MAXIMUM_STACK_SIZE / Self::VALUE_SIZE;

  /// 原版默认构造 (cc:131); 拷贝禁用 (cc:132-133) = Rust 默认 move-only
  #[must_use]
  pub fn new() -> Self {
    Self {
      data_: Vec::with_capacity(Self::K_STATIC_CAPACITY),
    }
  }

  /// 原版 push (cc:135-138): 压入并返回是否仍在预算内
  /// (false → 调用方走 MaybeThrowStackOverflow, cc:581 等, P-2/P-3a 批)
  #[must_use]
  pub fn push(&mut self, v: i32) -> bool {
    self.data_.push(v);
    (self.data_.len() as i32) <= Self::K_MAX_SIZE as i32
  }

  /// 原版 peek (cc:139-142) — SBXCHECK 非空
  #[must_use]
  pub fn peek(&self) -> i32 {
    sbx_check(!self.data_.is_empty(), "!data_.empty()");
    *self
      .data_
      .last()
      .unwrap_or_else(|| v8_fatal(format_args!("empty peek race")))
  }

  /// 原版 pop (cc:143-147) = peek + 出栈
  #[must_use]
  pub fn pop(&mut self) -> i32 {
    let v = self.peek();
    self.data_.pop();
    v
  }

  /// 原版 sp (cc:150) — 首个空槽下标
  #[inline]
  #[must_use]
  pub fn sp(&self) -> i32 {
    self.data_.len() as i32
  }

  /// 原版 set_sp (cc:151-154) — DCHECK_LE(new_sp, sp()): 只截断不扩张。
  /// 原版该不变式仅 debug 生效, release 下越界值直接进 SmallVector::resize
  /// (巨值分配/UB); Rust 侧升级为常驻 SBXCHECK 承接 debug 语义
  /// (ReadStackPointerFromRegister cc:635 的操作数即来自寄存器, 非法码流可致巨值)
  pub fn set_sp(&mut self, new_sp: u32) {
    sbx_check(
      usize::try_from(new_sp).unwrap_or(usize::MAX) <= self.data_.len(),
      "new_sp <= sp()",
    );
    self.data_.resize(new_sp as usize, Self::K_NO_FILL_VALUE);
  }

  /// set_sp 扩张分支不可达 (DCHECK 约束); resize 填充值仅防御占位,
  /// 正常路径不读取
  const K_NO_FILL_VALUE: i32 = 0;
}

// ── 解释器寄存器组 (cc:172-213) ────────────────────────────────────────────

/// 原版 RegExpMacroAssembler::kMaxRegisterCount (macro-assembler.h:37) = 1<<16 —
/// InterpreterRegisters 构造约束 (cc:188)
pub const K_MAX_REGISTER_COUNT: usize = 1 << 16;
/// 原版 kMaxRegister (macro-assembler.h:38) = kMaxRegisterCount - 1
pub const K_MAX_REGISTER: usize = K_MAX_REGISTER_COUNT - 1;
/// 原版 InterpreterRegisters::RegisterT = int (cc:174) → i32
pub type RegisterT = i32;

/// 原版 InterpreterRegisters (cc:172-213)。帧布局语义 (cc:167-171 注释):
/// [0, output_register_count) = 输出寄存器 — 每个捕获 (整体匹配算隐式捕获 0)
/// 的 {start,end} 对, 即 capture i ↔ 索引 2i/2i+1 (接驳 regexp_utils::
/// capture_start_index/capture_end_index);
/// [output_register_count, total_register_count) = 内部寄存器。
/// Register 编号即该数组下标 (操作数面 kRegister 限幅 0..=kMaxRegister,
/// bytecodes.h:45-46), 字节码侧偏移换算 = index * K_INT32_SIZE。
pub struct InterpreterRegisters {
  /// 原版 registers_ (cc:209) SmallVector<RegisterT, 64> → Vec<i32>,
  /// 构造即全填 kNoMatchValue
  registers_: Vec<i32>,
  /// 原版 total_register_count_ / output_register_count_ (cc:211-212)
  total_register_count_: usize,
  output_register_count_: usize,
}

impl InterpreterRegisters {
  /// 原版 kNoMatchValue = -1 (cc:175)
  pub const K_NO_MATCH_VALUE: i32 = -1;
  /// 原版 kStaticCapacity = 64 (cc:208)
  pub const K_STATIC_CAPACITY: usize = 64;

  /// 原版构造 (cc:177-191)。前提: output_register_count ≥ 2 (cc:186,
  /// 整体匹配占 2 槽) 且 ≤ total ≤ kMaxRegisterCount (cc:187-188)。
  /// 原版输出侧为裸指针 (cc:210) + MemCopy; Rust 以 copy_to_output_registers
  /// 的切片实参承接借用检查 (指针成员不落地)。
  pub fn new(total_register_count: i32, output_register_count: i32) -> Self {
    // 原版 static_assert(sizeof(int)==sizeof(int32_t)) (cc:185) 的 Rust 前提
    const _: () = assert!(size_of::<i32>() == K_INT32_SIZE);
    sbx_check(output_register_count >= 2, "output_register_count >= 2");

    sbx_check(
      total_register_count >= output_register_count,
      "total >= output",
    );
    sbx_check(
      (total_register_count as usize) <= K_MAX_REGISTER_COUNT,
      "total <= kMaxRegisterCount",
    );
    let total = total_register_count as usize;
    Self {
      registers_: vec![Self::K_NO_MATCH_VALUE; total.max(Self::K_STATIC_CAPACITY)],
      total_register_count_: total,
      output_register_count_: output_register_count as usize,
    }
  }

  /// 原版 operator[] 只读 (cc:193-196) — SBXCHECK_LT(index, total)
  #[must_use]
  pub fn get(&self, index: usize) -> i32 {
    sbx_check(
      index < self.total_register_count_,
      "index < total_register_count_",
    );
    self.registers_[index]
  }

  /// 原版 operator[] 可写 (cc:197-200) — SBXCHECK_LT 同上
  pub fn set(&mut self, index: usize, value: i32) {
    sbx_check(
      index < self.total_register_count_,
      "index < total_register_count_",
    );
    self.registers_[index] = value;
  }

  /// 输出寄存器槽数 (原版 output_register_count_ 消费面, Succeed handler
  /// cc:670 触发拷贝)
  #[inline]
  #[must_use]
  pub fn output_register_count(&self) -> usize {
    self.output_register_count_
  }

  /// 原版 CopyToOutputRegisters (cc:202-205) — MemCopy(output_registers_,
  /// registers_.data(), output_register_count * sizeof(RegisterT)): 仅拷前
  /// output_register_count 槽 (Succeed cc:670 / 命中回写面)
  pub fn copy_to_output_registers(&self, output: &mut [i32]) {
    debug_assert!(output.len() >= self.output_register_count_);
    let n = self.output_register_count_.min(output.len());
    output[..n].copy_from_slice(&self.registers_[..n]);
  }

  /// 寄存器编号 → 帧内字节偏移换算 (cc:181-185 注 TODO 的 int32 一致性面):
  /// 每个寄存器定宽 4 字节, index * K_INT32_SIZE
  #[inline]
  #[must_use]
  pub const fn register_byte_offset(index: usize) -> usize {
    index * K_INT32_SIZE
  }
}

// ── 纯辅助函数 (cc 匿名 namespace + 静态工具) ──────────────────────────────

/// 原版 BackRefMatchesNoCase Latin1 重载 (cc:63-81) — 逐字符小写比较:
/// ASCII a-z (|=0x20 后) 与 Latin-1 区 224..254 (≠247 ÷) 视同不敏感;
/// Unicode 标志无差别 (cc:65 "For Latin1 characters the unicode flag makes
/// no difference")。Unicode 重载 (cc:46-61) 依赖宏汇编器
/// CaseInsensitiveCompare* (ICU), 见 [P-3c] P-3c-icu 运行态通道。
#[must_use]
pub fn back_ref_matches_no_case_latin1(
  from: usize,
  current: usize,
  len: usize,
  subject: &[u8],
) -> bool {
  for i in 0..len {
    let old_char = subject[from + i];
    let new_char = subject[current + i];
    if old_char == new_char {
      continue;
    }
    // 双方折小写 (cc:71-72)
    let old_char = int_or_ascii_lower(old_char);
    let new_char = int_or_ascii_lower(new_char);
    if old_char != new_char {
      return false;
    }
    // 非 ASCII/Latin-1 字母 → 不匹配 (cc:75-78; 原版 unsigned 减法回绕, 故 wrapping_sub)
    if !(old_char.wrapping_sub(u32::from(b'a')) <= u32::from(b'z' - b'a'))
      && !(old_char.wrapping_sub(224) <= 254 - 224 && old_char != 247)
    {
      return false;
    }
  }
  true
}

/// cc:71 `unsigned int old_char |= 0x20` 的折小写位或
#[inline]
fn int_or_ascii_lower(c: u8) -> u32 {
  u32::from(c) | 0x20
}

/// 原版 BitsPerChar<uint8_t> (cc:105-108) = kBitsPerByte * 1
#[must_use]
pub const fn bits_per_char_one_byte() -> u32 {
  K_BITS_PER_BYTE as u32
}

/// 原版 BitsPerChar<uc16> (cc:105-108) = kBitsPerByte * 2
#[must_use]
pub const fn bits_per_char_two_byte() -> u32 {
  (K_BITS_PER_BYTE * 2) as u32
}

/// 原版 Load2Characters<uint8_t> (cc:110-113): s[i] | s[i+1] << 8
#[must_use]
pub fn load_2_characters_one_byte(string: &[u8], index: usize) -> u32 {
  u32::from(string[index]) | (u32::from(string[index + 1]) << bits_per_char_one_byte())
}

/// 原版 Load2Characters<uc16> (cc:110-113): s[i] | s[i+1] << 16
#[must_use]
pub fn load_2_characters_two_byte(string: &[u16], index: usize) -> u32 {
  u32::from(string[index]) | (u32::from(string[index + 1]) << bits_per_char_two_byte())
}

/// 原版 Load4Characters<uint8_t> (cc:115-118): 定宽移位 8/16/24 组装 4 字符
#[must_use]
pub fn load_4_characters_one_byte(string: &[u8], index: usize) -> u32 {
  u32::from(string[index])
    | (u32::from(string[index + 1]) << 8)
    | (u32::from(string[index + 2]) << 16)
    | (u32::from(string[index + 3]) << 24)
}

/// 原版 Load4Characters<uc16> (cc:120-122): UNREACHABLE
#[must_use]
pub fn load_4_characters_two_byte(_string: &[u16], _index: usize) -> u32 {
  v8_fatal(format_args!("UNREACHABLE: Load4Characters(uc16) (cc:121)"));
}

/// 原版 RegExpMacroAssembler::kTableSizeBits (macro-assembler.h:48) = 7 —
/// 128 字符位图窗口
const K_TABLE_SIZE_BITS: u32 = 7;
/// 原版 kTableSize (macro-assembler.h:49) = 1<<7 = 128
const K_TABLE_SIZE: u32 = 1 << K_TABLE_SIZE_BITS;
/// 原版 kTableMask (macro-assembler.h:50) = kTableSize - 1 = 127
const K_TABLE_MASK: u32 = K_TABLE_SIZE - 1;

/// 原版 CheckBitInTable (cc:321-326): 位图窗口 = (char & 127) >> 3 字节,
/// 位号 = char & 7 (table 为 SPECIAL kBitTable 操作数, 16 字节定长,
/// bytecodes.h:43)。K_BITS_PER_BYTE_LOG2 (v8-common globals.h:1447) 即原版
/// kBitsPerByteLog2。
#[must_use]
pub fn check_bit_in_table(current_char: u32, table: &[u8]) -> bool {
  let byte =
    usize::try_from((current_char & K_TABLE_MASK) >> K_BITS_PER_BYTE_LOG2).unwrap_or(usize::MAX);
  let bit = current_char & (K_BITS_PER_BYTE as u32 - 1);
  table.get(byte).is_some_and(|b| (b & (1u8 << bit)) != 0)
}

/// 原版 IndexIsInBounds (cc:329-332): uintptr 回绕判定 — 负 index 折成巨值
/// 恒 false; DCHECK_GE(length, 0) (cc:330) 以 debug_assert 承接
#[must_use]
pub fn index_is_in_bounds(index: i32, length: i32) -> bool {
  debug_assert!(length >= 0);
  (index as usize) < (length as usize)
}

/// 原版 CheckSpecialClassRanges<Char> (cc:418-497) — 9 个命名字符集的核判定
/// (is_one_byte = sizeof(Char)==1, cc:421 以 const 单参数泛化)。
/// word/notword 分支原版读 RegExpMacroAssembler::word_character_map()
/// (macro-assembler.h:403-407, 256 项静态表 h:435/439, 定义在 .cc 汇编器侧),
/// 落地按参数注入 (词字符表内容属宏汇编器面, 越界批)。
/// kNotWhitespace 分支原版 UNREACHABLE (cc:430-431): 该集恒在解析期折成正向
/// 集, 不达派发。
#[must_use]
pub fn check_special_class_ranges<const ONE_BYTE: bool>(
  current_char: u32,
  character_set: StandardCharacterSet,
  word_character_map: &[u8],
) -> bool {
  match character_set {
    StandardCharacterSet::Whitespace => {
      // DCHECK(is_one_byte) (cc:424)
      debug_assert!(ONE_BYTE);
      current_char == u32::from(b' ')
        || (u32::from(b'\t')..=u32::from(b'\r')).contains(&current_char)
        || current_char == 0xA0
    }
    StandardCharacterSet::NotWhitespace => {
      v8_fatal(format_args!("UNREACHABLE: kNotWhitespace (cc:431)"));
    }
    StandardCharacterSet::Word => {
      if !ONE_BYTE && current_char > u32::from(b'z') {
        // cc:433-437 (两字节模式 > 'z' 直 false)
        return false;
      }
      // DCHECK_EQ(0, map[0]) (cc:440-441): '\0' 非词字符
      debug_assert_eq!(word_character_map.first(), Some(&0));
      // cc:442 (cc:443 死码 return true 不可达, 1:1 不落)
      word_character_map
        .get(current_char as usize)
        .is_some_and(|m| *m != 0)
    }
    StandardCharacterSet::NotWord => {
      if !ONE_BYTE && current_char > u32::from(b'z') {
        return true; // cc:446-450
      }
      debug_assert_eq!(word_character_map.first(), Some(&0));
      !word_character_map
        .get(current_char as usize)
        .is_some_and(|m| m != &0) // cc:455
    }
    StandardCharacterSet::Digit => (u32::from(b'0')..=u32::from(b'9')).contains(&current_char), // cc:457-461
    StandardCharacterSet::NotDigit => !(u32::from(b'0')..=u32::from(b'9')).contains(&current_char), // cc:462-466
    StandardCharacterSet::LineTerminator => {
      if current_char == u32::from(b'\n') || current_char == u32::from(b'\r') {
        return true; // cc:468-470
      }
      if !ONE_BYTE && (current_char == 0x2028 || current_char == 0x2029) {
        return true; // cc:471-473
      }
      false // cc:476
    }
    StandardCharacterSet::NotLineTerminator => {
      let is_one_byte_match = current_char != u32::from(b'\n') && current_char != u32::from(b'\r');
      if is_one_byte_match && (ONE_BYTE || (current_char != 0x2028 && current_char != 0x2029)) {
        return true; // cc:479-490 两分支合一 (one_byte 短视 true 语义等价)
      }
      false // cc:491
    }
    StandardCharacterSet::Everything => true, // cc:493-494
  }
}

/// 原版 RegExpMacroAssembler::word_character_map_
/// (regexp-macro-assembler.cc:720-761, 256 项静态表) 的 1:1 编译期复刻 —
/// [0-9] [A-Z] '_' [a-z] → 0xFF, 其余 (含 Latin-1 区 128..255) → 0x00。
/// 消费面: CheckSpecialClassRanges handler (cc:1000) 的 word/notword 分支
/// (原版宏汇编器侧词表读取 cc:442/455); 亦供单测注入
/// check_special_class_ranges 的 word_character_map 参数。
pub const K_WORD_CHARACTER_MAP: [u8; 256] = {
  // const fn 无法局部可变, 就地块内构造 (与 cc 字表逐段对位)
  let mut map = [0u8; 256];
  let mut i = 0usize;
  // 原版 728-729: '0'(48) - '9'(57) → 0xFF
  while i < 10 {
    map[48 + i] = 0xFF;
    i += 1;
  }
  // 原版 731-734: 'A'(65) - 'Z'(90) → 0xFF; '_'(95) → 0xFF
  i = 0;
  while i < 26 {
    map[65 + i] = 0xFF;
    i += 1;
  }
  map[95] = 0xFF;
  // 原版 736-739: 'a'(97) - 'z'(122) → 0xFF
  i = 0;
  while i < 26 {
    map[97 + i] = 0xFF;
    i += 1;
  }
  // 表内容不变式与原版逐字面锚: 首项 DCHECK_EQ(0, map[0]) (cc:440-441),
  // 非词字符样例 (NUL/空格/'@'/'{'/Latin-1 'à') 均 0x00, 词字符样例 0xFF
  // (const 求值期内 assert! 失败 = 编译错误, 表内容冻结)
  assert!(map[0] == 0x00);
  assert!(map[32] == 0x00); // ' '
  assert!(map[48] == 0xFF); // '0'
  assert!(map[57] == 0xFF); // '9'
  assert!(map[58] == 0x00); // ':'
  assert!(map[64] == 0x00); // '@'
  assert!(map[65] == 0xFF); // 'A'
  assert!(map[90] == 0xFF); // 'Z'
  assert!(map[94] == 0x00); // '^'
  assert!(map[95] == 0xFF); // '_'
  assert!(map[96] == 0x00); // '`'
  assert!(map[97] == 0xFF); // 'a'
  assert!(map[122] == 0xFF); // 'z'
  assert!(map[123] == 0x00); // '{'
  assert!(map[224] == 0x00); // Latin-1 'à' 非词字符 (cc:741 起恒 0x00)
  map
};

/// 原版 operand `character_set: StandardCharacterSet`（regexp-bytecodes.h:132）
/// 的 u8 承接：原版 `StandardCharacterSet::Type` 以 char 为编码底座
/// （regexp-ast.h:102-112: kWhitespace='s', kNotWhitespace='S', kWord='w',
/// kNotWord='W', kDigit='d', kNotDigit='D', kLineTerminator='n',
/// kNotLineTerminator='.', kEverything='*'），字节码操作数存的是该 char 值;
/// 经 [`StandardCharacterSet::from_escape`]（与 regexp-ast.h 字母表 1:1）还原,
/// 非字母表字节 → None（原版由生成器保证不发, 承接面在 handler 处 IllegalCode）
#[must_use]
fn scs_from_u8(v: u8) -> Option<StandardCharacterSet> {
  StandardCharacterSet::from_escape(char::from(v))
}

// ── 操作数体系与 INIT 解码基座 ─────────────────────────────────────────────
// 原版: BytecodeOperandType 全集 (regexp-bytecodes.h:30-51 三表) + 宽度/对齐 traits
// (regexp-bytecodes-inl.h:23-69) + 布局算法 CalculateAlignedOffsets (inl.h:107-136)
// + 指令总宽 kSize (inl.h:149-151) + BytecodeOperands<bc> 特化 (inl.h:292-303)
// + Get<op> 读取 (inl.h:244-273) + RawMatch 侧 INIT 宏 (cc:402-414: 每 handler
// 入口 constexpr current_bc / using Operands = BytecodeOperands<current_bc> /
// 按操作数名解出 argument_tuple)。
// 票面差异登记: 票单提到的 kOpSizeNone/kOpSizeOne/kOpSizeTwo/kOpSizeFour 与
// ReadOneOperand 系上游旧版 (2024 前) 手写布局; 本仓库版 (2025 inl.h) 已改为
// 按操作数类型 1/2/4 字节 + 自身对齐 + 4 字节块不跨界 (inl.h:123-129) 自动排布,
// 全仓 grep kOpSize/ReadOneOperand = 0 命中。本票按现版逐条 1:1 承接。

/// 原版 `enum class BytecodeOperandType : uint8_t` (bytecodes.h:53-60, 由
/// BYTECODE_OPERAND_TYPE_LIST h:30-51 = BASIC 5 + LIMITS 5 + SPECIAL 1 展开) —
/// 逐名逐号 (编号即 inl.h 侧 kOperandTypeSizes 下标, inl.h:324-328)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OperandType {
  /// BASIC (h:32) int16_t — Backtrack 的 return_code
  Int16 = 0,
  /// BASIC (h:33) int32_t — SetRegister/IfRegister* 的 value/comparand
  Int32 = 1,
  /// BASIC (h:34) uint32_t — Check*4Chars 的 characters/mask
  Uint32 = 2,
  /// BASIC (h:35) base::uc16 — 字符比较类操作数
  Char = 3,
  /// BASIC (h:36) uint32_t — 字节码绝对偏移 (相对 code_base), 跳转目标
  JumpTarget = 4,
  /// LIMITS (h:39) int16_t, 限幅 [kMinCPOffset, kMaxCPOffset] = [-32767, 32767]
  Offset = 5,
  /// LIMITS (h:40) int32_t, 限幅 [kMinCPOffset, kMaxCPOffset + kMaxEatsAtLeastValue]
  BoundsCheckOffset = 6,
  /// LIMITS (h:42) uint16_t, 限幅 [0, kMaxRegister] (macro-assembler.h:38)
  Register = 7,
  /// LIMITS (h:43) RegExpMacroAssembler::StackCheckFlag : uint8_t
  /// (macro-assembler.h:275-278) — 宽度由 enum 底层类型定, 非 sizeof(int)
  StackCheckFlag = 8,
  /// LIMITS (h:44) StandardCharacterSet : char (regexp-ast.h:102, 值为转义字母)
  StandardCharacterSet = 9,
  /// SPECIAL (h:47) `V(BitTable, 16, 1)` — 16 字节定宽位图, 对齐 1
  BitTable = 10,
}

/// 原版 Bytecodes::kOperandTypeCount 面 (无同名常量; 11 = 三表展开实测,
/// 与 inl.h:326 kOperandTypeSizes 长度一致)
pub const K_OPERAND_TYPE_COUNT: usize = 11;

impl OperandType {
  /// 原版 OperandTypeTraits<T>::kSize (inl.h:30/45 = sizeof(CType);
  /// inl.h:63 = SPECIAL 专列 16) — 逐型实测:
  /// int16_t/uc16/uint16_t = 2, int32_t/uint32_t = 4,
  /// StackCheckFlag(uint8_t 底层)/StandardCharacterSet(char) = 1, BitTable = 16
  #[must_use]
  pub const fn size(self) -> u8 {
    match self {
      Self::Int16 | Self::Char | Self::Offset | Self::Register => 2,
      Self::Int32 | Self::Uint32 | Self::JumpTarget | Self::BoundsCheckOffset => 4,
      Self::StackCheckFlag | Self::StandardCharacterSet => 1,
      Self::BitTable => 16,
    }
  }

  /// 原版 kAlignment (inl.h:35/54: basic 型 = kSize; inl.h:65: BitTable = 1)
  #[must_use]
  pub const fn alignment(self) -> u8 {
    match self {
      Self::BitTable => 1,
      other => other.size(),
    }
  }

  /// 原版 kIsBasic (inl.h:32/53 = true, inl.h:64 = false) — false 仅 BitTable,
  /// 其 Get 走"返回 pc+offset 指针"分支 (inl.h:265-273) 而非 reinterpret_cast
  #[must_use]
  pub const fn is_basic(self) -> bool {
    !matches!(self, Self::BitTable)
  }
}

// 原版限幅常量 (macro-assembler.h:42-46; 经 inl.h:40-55 进入 traits) —
// Offset 的 min 取负对称值, 保证取反后仍合法 (macro-assembler.h:41 注释)
/// 原版 kMaxCPOffset (macro-assembler.h:42) = (1<<15)-1
pub const K_MAX_CP_OFFSET: i32 = (1 << 15) - 1;
/// 原版 kMinCPOffset (macro-assembler.h:43) = -kMaxCPOffset
pub const K_MIN_CP_OFFSET: i32 = -K_MAX_CP_OFFSET;
/// 原版 kMaxEatsAtLeastValue (macro-assembler.h:45-46) = uint8 全域
pub const K_MAX_EATS_AT_LEAST_VALUE: i32 = u8::MAX as i32;
/// 原版 BoundsCheckOffset 上界 (bytecodes.h:40-42) = kMaxCPOffset + kMaxEatsAtLeastValue
pub const K_MAX_BOUNDS_CHECK_OFFSET: i32 = K_MAX_CP_OFFSET + K_MAX_EATS_AT_LEAST_VALUE;

/// 原版 SPECIAL kBitTable 宽度 16 = kTableSize(128)/kBitsPerByte(8)
/// (inl.h:269 static_assert; bytecodes.h:47)
pub const K_BIT_TABLE_BYTES: usize = K_TABLE_SIZE as usize / K_BITS_PER_BYTE;
const _: () = assert!(K_BIT_TABLE_BYTES == OperandType::BitTable.size() as usize);

/// 原版 first_offset = sizeof(Bytecode) (inl.h:116) — opcode 占 1 字节,
/// 操作数区自 pc+1 起按各自对齐排布
const K_OPERAND_REGION_START: usize = size_of::<Bytecode>();

/// 原版 SkipUntilOneOfMasked3 的 20 个操作数 (bytecodes.h:294-327) = 全集最大;
/// Rust 定长槽承接 inl.h:110-113 的变长 std::array (count 之外的槽无效)
pub const K_MAX_OPERAND_COUNT: usize = 20;

/// 原版 detail::BytecodeOperandsTraits<OpTypes...> (inl.h:138-152) — 单条
/// bytecode 的操作数布局: 类型序列 + 帧内偏移 + 指令总宽 (= 模板特化的数据面)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BytecodeOperandLayout {
  /// kOperandCount (inl.h:140)
  pub count: u8,
  /// kSize (inl.h:149-151) — RoundUp<4>(末操作数偏移 + 宽度); 零操作数时
  /// RoundUp<4>(sizeof(Bytecode)) = 4
  pub total_size: u8,
  /// kOperandOffsets (inl.h:147, 由 CalculateAlignedOffsets 得) — 相对 pc 的字节偏移
  offsets: [u8; K_MAX_OPERAND_COUNT],
  /// kOperandTypes (inl.h:141-142)
  types: [OperandType; K_MAX_OPERAND_COUNT],
}

/// 原版 base::RoundUp (inl.h:123/128 消费) — 正数区间无溢出风险
const fn round_up(value: usize, align: usize) -> usize {
  value.div_ceil(align) * align
}

/// 原版 detail::CalculateAlignedOffsets (inl.h:107-136) + kSize (inl.h:149-151)
/// 的 1:1 移植: 逐操作数先按自身对齐进位, 若"当前 4 字节块内剩余 < 操作数宽度"
/// 则另起一块 (inl.h:125-129 注释), 最终宽度向上取整到 4。
const fn operand_layout(types: &[OperandType]) -> BytecodeOperandLayout {
  // count ≤ K_MAX_OPERAND_COUNT 由 BYTECODE_LAYOUTS 构造期的数组长度保证
  let mut offsets = [0u8; K_MAX_OPERAND_COUNT];
  // 槽位填充值: count 之外不读 (与 inl.h:110-113 变长 array 语义等价)
  let mut slots = [OperandType::Int16; K_MAX_OPERAND_COUNT];
  let mut offset = K_OPERAND_REGION_START; // inl.h:117 first_offset
  let mut i = 0;
  while i < types.len() {
    let size = types[i].size() as usize;
    let alignment = types[i].alignment() as usize;
    offset = round_up(offset, alignment); // inl.h:123
    if offset % K_BYTECODE_ALIGNMENT + size > K_BYTECODE_ALIGNMENT {
      // inl.h:127-129: 块内放不下 → RoundUp<kBytecodeAlignment>(offset)
      offset = round_up(offset, K_BYTECODE_ALIGNMENT);
    }
    offsets[i] = offset as u8; // 偏移实测 ≤ 88 (SkipUntilOneOfMasked3), u8 充分
    slots[i] = types[i];
    offset += size;
    i += 1;
  }
  let region_end = if types.is_empty() {
    K_OPERAND_REGION_START
  } else {
    // inl.h:150-151: kOperandOffsets.back() + kOperandSizes.back()
    offsets[i - 1] as usize + types[i - 1].size() as usize
  };
  BytecodeOperandLayout {
    count: i as u8,
    total_size: round_up(region_end, K_BYTECODE_ALIGNMENT) as u8,
    offsets,
    types: slots,
  }
}

/// 原版 detail::kBytecodeSizes (inl.h:312-316) 与各 BytecodeOperands<bc> 特化
/// (inl.h:292-303, 由 REGEXP_BYTECODE_LIST 宏逐项展开) 的全集台账 — 枚举序 61 项,
/// 操作数类型/名逐条对齐 regexp-bytecodes.h 四表 (h:90/93-186/188-243/245-328);
/// 由 tool/src/regexp/regexp-interpreter.cc.js 逐名逐类型核对 (缺项 throw)
const BYTECODE_LAYOUTS: [BytecodeOperandLayout; K_BYTECODE_COUNT as usize] = [
  operand_layout(&[]),                                             // [0] Break
  operand_layout(&[]),                                             // [1] PushCurrentPosition
  operand_layout(&[OperandType::JumpTarget]),                      // [2] PushBacktrack (label)
  operand_layout(&[OperandType::Register, OperandType::Offset]), // [3] WriteCurrentPositionToRegister (register_index, cp_offset)
  operand_layout(&[OperandType::Register]), // [4] ReadCurrentPositionFromRegister (register_index)
  operand_layout(&[OperandType::Register]), // [5] WriteStackPointerToRegister (register_index)
  operand_layout(&[OperandType::Register]), // [6] ReadStackPointerFromRegister (register_index)
  operand_layout(&[OperandType::Register, OperandType::Int32]), // [7] SetRegister (register_index, value)
  operand_layout(&[OperandType::Register, OperandType::Register]), // [8] ClearRegisters (from_register, to_register)
  operand_layout(&[OperandType::Register, OperandType::Offset]), // [9] AdvanceRegister (register_index, by)
  operand_layout(&[]),                                           // [10] PopCurrentPosition
  operand_layout(&[OperandType::Register, OperandType::StackCheckFlag]), // [11] PushRegister (register_index, stack_check)
  operand_layout(&[OperandType::Register]), // [12] PopRegister (register_index)
  operand_layout(&[]),                      // [13] Fail
  operand_layout(&[]),                      // [14] Succeed
  operand_layout(&[OperandType::Offset]),   // [15] AdvanceCurrentPosition (by)
  operand_layout(&[OperandType::JumpTarget]), // [16] GoTo (label)
  operand_layout(&[
    OperandType::Offset,
    OperandType::BoundsCheckOffset,
    OperandType::JumpTarget,
  ]), // [17] LoadCurrentCharacter (cp_offset, bounds_check_offset, on_failure)
  operand_layout(&[OperandType::Offset, OperandType::JumpTarget]), // [18] CheckPosition (cp_offset, on_failure)
  operand_layout(&[OperandType::StandardCharacterSet, OperandType::JumpTarget]), // [19] CheckSpecialClassRanges (character_set, on_no_match)
  operand_layout(&[OperandType::Char, OperandType::JumpTarget]), // [20] CheckCharacter (character, on_equal)
  operand_layout(&[OperandType::Char, OperandType::JumpTarget]), // [21] CheckNotCharacter (character, on_not_equal)
  operand_layout(&[
    OperandType::Char,
    OperandType::Uint32,
    OperandType::JumpTarget,
  ]), // [22] CheckCharacterAfterAnd (character, mask, on_equal)
  operand_layout(&[
    OperandType::Char,
    OperandType::Uint32,
    OperandType::JumpTarget,
  ]), // [23] CheckNotCharacterAfterAnd (character, mask, on_not_equal)
  operand_layout(&[
    OperandType::Char,
    OperandType::Char,
    OperandType::Char,
    OperandType::JumpTarget,
  ]), // [24] CheckNotCharacterAfterMinusAnd (character, minus, mask, on_not_equal)
  operand_layout(&[
    OperandType::Char,
    OperandType::Char,
    OperandType::JumpTarget,
  ]), // [25] CheckCharacterInRange (from, to, on_in_range)
  operand_layout(&[
    OperandType::Char,
    OperandType::Char,
    OperandType::JumpTarget,
  ]), // [26] CheckCharacterNotInRange (from, to, on_not_in_range)
  operand_layout(&[OperandType::Char, OperandType::JumpTarget]), // [27] CheckCharacterLT (limit, on_less)
  operand_layout(&[OperandType::Char, OperandType::JumpTarget]), // [28] CheckCharacterGT (limit, on_greater)
  operand_layout(&[
    OperandType::Register,
    OperandType::Int32,
    OperandType::JumpTarget,
  ]), // [29] IfRegisterLT (register_index, comparand, on_less_than)
  operand_layout(&[
    OperandType::Register,
    OperandType::Int32,
    OperandType::JumpTarget,
  ]), // [30] IfRegisterGE (register_index, comparand, on_greater_or_equal)
  operand_layout(&[OperandType::Register, OperandType::JumpTarget]), // [31] IfRegisterEqPos (register_index, on_eq)
  operand_layout(&[OperandType::Offset, OperandType::JumpTarget]), // [32] CheckAtStart (cp_offset, on_at_start)
  operand_layout(&[OperandType::Offset, OperandType::JumpTarget]), // [33] CheckNotAtStart (cp_offset, on_not_at_start)
  operand_layout(&[OperandType::JumpTarget]), // [34] CheckFixedLengthLoop (on_tos_equals_current_position)
  operand_layout(&[OperandType::Offset]),     // [35] SetCurrentPositionFromEnd (by)
  operand_layout(&[OperandType::Int16]),      // [36] Backtrack (return_code)
  operand_layout(&[OperandType::Offset]),     // [37] LoadCurrentCharacterUnchecked (cp_offset)
  operand_layout(&[OperandType::JumpTarget, OperandType::BitTable]), // [38] CheckBitInTable (on_bit_set, table)
  operand_layout(&[
    OperandType::Offset,
    OperandType::BoundsCheckOffset,
    OperandType::JumpTarget,
  ]), // [39] Load2CurrentChars (cp_offset, bounds_check_offset, on_failure)
  operand_layout(&[OperandType::Offset]), // [40] Load2CurrentCharsUnchecked (cp_offset)
  operand_layout(&[
    OperandType::Offset,
    OperandType::BoundsCheckOffset,
    OperandType::JumpTarget,
  ]), // [41] Load4CurrentChars (cp_offset, bounds_check_offset, on_failure)
  operand_layout(&[OperandType::Offset]), // [42] Load4CurrentCharsUnchecked (cp_offset)
  operand_layout(&[OperandType::Uint32, OperandType::JumpTarget]), // [43] Check4Chars (characters, on_equal)
  operand_layout(&[OperandType::Uint32, OperandType::JumpTarget]), // [44] CheckNot4Chars (characters, on_not_equal)
  operand_layout(&[
    OperandType::Uint32,
    OperandType::Uint32,
    OperandType::JumpTarget,
  ]), // [45] AndCheck4Chars (characters, mask, on_equal)
  operand_layout(&[
    OperandType::Uint32,
    OperandType::Uint32,
    OperandType::JumpTarget,
  ]), // [46] AndCheckNot4Chars (characters, mask, on_not_equal)
  operand_layout(&[OperandType::Offset, OperandType::JumpTarget]), // [47] AdvanceCpAndGoto (by, on_goto)
  operand_layout(&[OperandType::Register, OperandType::JumpTarget]), // [48] CheckNotBackRef (start_reg, on_not_equal)
  operand_layout(&[OperandType::Register, OperandType::JumpTarget]), // [49] CheckNotBackRefNoCase (start_reg, on_not_equal)
  operand_layout(&[OperandType::Register, OperandType::JumpTarget]), // [50] CheckNotBackRefNoCaseUnicode (start_reg, on_not_equal)
  operand_layout(&[OperandType::Register, OperandType::JumpTarget]), // [51] CheckNotBackRefBackward (start_reg, on_not_equal)
  operand_layout(&[OperandType::Register, OperandType::JumpTarget]), // [52] CheckNotBackRefNoCaseBackward (start_reg, on_not_equal)
  operand_layout(&[OperandType::Register, OperandType::JumpTarget]), // [53] CheckNotBackRefNoCaseUnicodeBackward (start_reg, on_not_equal)
  operand_layout(&[
    OperandType::Offset,
    OperandType::Offset,
    OperandType::BitTable,
    OperandType::BoundsCheckOffset,
    OperandType::JumpTarget,
    OperandType::JumpTarget,
  ]), // [54] SkipUntilBitInTable (cp_offset, advance_by, table, bounds_check_offset, on_match, on_no_match)
  operand_layout(&[
    OperandType::Offset,
    OperandType::Offset,
    OperandType::Char,
    OperandType::Uint32,
    OperandType::BoundsCheckOffset,
    OperandType::JumpTarget,
    OperandType::JumpTarget,
  ]), // [55] SkipUntilCharAnd (cp_offset, advance_by, character, mask, bounds_check_offset, on_match, on_no_match)
  operand_layout(&[
    OperandType::Offset,
    OperandType::Offset,
    OperandType::Char,
    OperandType::BoundsCheckOffset,
    OperandType::JumpTarget,
    OperandType::JumpTarget,
  ]), // [56] SkipUntilChar (cp_offset, advance_by, character, bounds_check_offset, on_match, on_no_match)
  operand_layout(&[
    OperandType::Offset,
    OperandType::Offset,
    OperandType::Char,
    OperandType::Char,
    OperandType::BoundsCheckOffset,
    OperandType::JumpTarget,
    OperandType::JumpTarget,
  ]), // [57] SkipUntilCharOrChar (cp_offset, advance_by, char1, char2, bounds_check_offset, on_match, on_no_match)
  operand_layout(&[
    OperandType::Offset,
    OperandType::Offset,
    OperandType::Char,
    OperandType::BitTable,
    OperandType::BoundsCheckOffset,
    OperandType::JumpTarget,
    OperandType::JumpTarget,
  ]), // [58] SkipUntilGtOrNotBitInTable (cp_offset, advance_by, character, table, bounds_check_offset, on_match, on_no_match)
  operand_layout(&[
    OperandType::Offset,
    OperandType::Offset,
    OperandType::Uint32,
    OperandType::Uint32,
    OperandType::BoundsCheckOffset,
    OperandType::Uint32,
    OperandType::Uint32,
    OperandType::Uint32,
    OperandType::Uint32,
    OperandType::JumpTarget,
    OperandType::JumpTarget,
    OperandType::JumpTarget,
  ]), // [59] SkipUntilOneOfMasked (cp_offset, advance_by, both_chars, both_mask, max_offset, chars1, mask1, chars2, mask2, on_match1, on_match2, on_failure)
  operand_layout(&[
    OperandType::Offset,
    OperandType::Offset,
    OperandType::BitTable,
    OperandType::BoundsCheckOffset,
    OperandType::JumpTarget,
    OperandType::Offset,
    OperandType::Uint32,
    OperandType::Uint32,
    OperandType::Offset,
    OperandType::BoundsCheckOffset,
    OperandType::Offset,
    OperandType::Uint32,
    OperandType::Uint32,
    OperandType::JumpTarget,
    OperandType::Uint32,
    OperandType::Uint32,
    OperandType::JumpTarget,
    OperandType::Uint32,
    OperandType::Uint32,
    OperandType::JumpTarget,
  ]), // [60] SkipUntilOneOfMasked3 (bc0_cp_offset, bc0_advance_by, bc0_table, bc1_bounds_check_offset, bc1_on_failure, bc1_cp_offset, bc2_characters, bc2_mask, bc3_by, bc4_bounds_check_offset, bc4_cp_offset, bc5_characters, bc5_mask, bc5_on_equal, bc6_characters, bc6_mask, bc6_on_equal, bc7_characters, bc7_mask, fallthrough_jump_target)
];

/// 原版 Bytecodes::Size(Bytecode) (bytecodes.h:361-362 声明 /
/// regexp-bytecodes-inl.h:357-365 定义: kBytecodeSizes[ToByte(bc)]) —
/// cc:368 ADVANCE() 的唯一消费面
#[must_use]
pub const fn bytecode_size(bc: Bytecode) -> u8 {
  BYTECODE_LAYOUTS[bc as usize].total_size
}

/// 原版 Operands::kCount (inl.h:177 = Traits::kOperandCount)
#[must_use]
pub const fn operand_count(bc: Bytecode) -> usize {
  BYTECODE_LAYOUTS[bc as usize].count as usize
}

/// 原版 Operands::Offset(op) (inl.h:183-185 = Traits::kOperandOffsets[Index(op)])
#[must_use]
pub const fn operand_offset(bc: Bytecode, op: usize) -> usize {
  debug_assert!(op < operand_count(bc));
  BYTECODE_LAYOUTS[bc as usize].offsets[op] as usize
}

/// 原版 Operands::Size(op) (inl.h:180-182 = Traits::kOperandSizes[Index(op)])
#[must_use]
pub const fn operand_size(bc: Bytecode, op: usize) -> u8 {
  debug_assert!(op < operand_count(bc));
  BYTECODE_LAYOUTS[bc as usize].types[op].size()
}

/// 原版 Operands::Type(op) (inl.h:186-188 = Traits::kOperandTypes[Index(op)])
#[must_use]
pub const fn operand_type(bc: Bytecode, op: usize) -> OperandType {
  debug_assert!(op < operand_count(bc));
  BYTECODE_LAYOUTS[bc as usize].types[op]
}

/// 原版 Operands::Get<op>(pc, no_gc) 的 basic 分支 (inl.h:244-253):
/// `*reinterpret_cast<const CType*>(pc + Offset(op))` — 宿主字节序直读 (inl.h:251
/// DCHECK IsAligned(offset, sizeof(CType)) 由布局算法保证), Rust 以 from_ne_bytes
/// 等价 (x64/arm64 小端; s390/mips64/ppc64 大端随宿主, 原版同为宿主序)。
/// 宽度须与 Operands::Size(op) 全等 — C++ 模板侧为编译期保证, 此处运行期
/// SBXCHECK 承接 (操作数下标越界或宽度不符即票单级错误)。
#[inline]
fn operand_bytes<const N: usize>(code: &[u8], pc: u32, bc: Bytecode, op: usize) -> [u8; N] {
  // DCHECK_EQ(Bytecodes::FromPtr(pc), bc) (inl.h:247): 调用方 pc 处 opcode 一致
  debug_assert_eq!(
    Bytecode::from_byte(*code.get(pc as usize).unwrap_or(&u8::MAX)),
    Some(bc)
  );
  sbx_check(
    operand_size(bc, op) as usize == N,
    "operand width == Operands::Size(op)",
  );
  let abs = pc as usize + operand_offset(bc, op);
  // 原版定长 TrustedByteArray 内 reinterpret_cast (cc:560/inl.h:252);
  // Rust 越界 → 非法码流 (取代 UB, 禁静默补零)
  let src = code.get(abs..abs + N).unwrap_or_else(|| {
    v8_fatal(format_args!(
      "bytecode operand OOB: bc={bc:?} op={op} abs={abs}"
    ))
  });
  let mut out = [0u8; N];
  out.copy_from_slice(src);
  out
}

/// Get<op> int16_t 分支 (inl.h:250-252; kInt16 — Backtrack 的 return_code)
#[must_use]
pub fn operand_i16(code: &[u8], pc: u32, bc: Bytecode, op: usize) -> i16 {
  i16::from_ne_bytes(operand_bytes::<2>(code, pc, bc, op))
}

/// Get<op> kChar/kRegister 分支 (base::uc16 / uint16_t, 均 2 字节)
#[must_use]
pub fn operand_u16(code: &[u8], pc: u32, bc: Bytecode, op: usize) -> u16 {
  u16::from_ne_bytes(operand_bytes::<2>(code, pc, bc, op))
}

/// Get<op> int32_t 分支 (kInt32/kOffset 语义值; inl.h:250-252)
#[must_use]
pub fn operand_i32(code: &[u8], pc: u32, bc: Bytecode, op: usize) -> i32 {
  i32::from_ne_bytes(operand_bytes::<4>(code, pc, bc, op))
}

/// Get<op> uint32_t 分支 (kUint32/kJumpTarget/kBoundsCheckOffset 的无符号面)
#[must_use]
pub fn operand_u32(code: &[u8], pc: u32, bc: Bytecode, op: usize) -> u32 {
  u32::from_ne_bytes(operand_bytes::<4>(code, pc, bc, op))
}

/// Get<op> 1 字节枚举底层分支 (kStackCheckFlag : uint8_t / kStandardCharacterSet
/// : char) — 原版经 traits 的 CType 直读 (inl.h:252), 此处保留原始字节,
/// 语义映射 (StackCheckFlag 二值 / StandardCharacterSet 转义字母表) 随各批 handler 落
#[must_use]
pub fn operand_u8(code: &[u8], pc: u32, bc: Bytecode, op: usize) -> u8 {
  operand_bytes::<1>(code, pc, bc, op)[0]
}

/// Get 的 kBitTable 特化分支 (inl.h:265-273): `static_assert(Size(op) ==
/// kTableSize/kBitsPerByte)` + 返回 pc + Offset(op) 指针 → Rust 定长切片
#[must_use]
pub fn operand_bit_table(code: &[u8], pc: u32, bc: Bytecode, op: usize) -> &[u8] {
  sbx_check(
    operand_type(bc, op) == OperandType::BitTable,
    "operand type == kBitTable",
  );
  let abs = pc as usize + operand_offset(bc, op);
  code
    .get(abs..abs + K_BIT_TABLE_BYTES)
    .unwrap_or_else(|| v8_fatal(format_args!("bit table operand OOB: bc={bc:?} abs={abs}")))
}

/// 原版 BytecodeOperands<bc>::Operand 枚举 + `using enum Operand`
/// (inl.h:157-169/292-300) 的下标面承接: 全 61 项 handler 中有操作数的 56 项,
/// 逐名 (名 = bytecodes.h 操作数名 snake_case) 逐序 — P-3a 18 项随 REG-1b2,
/// P-3b/c/d 38 项随 REG-1b3 (零操作数的 5 项: Break/PushCurrentPosition/
/// PopCurrentPosition/Fail/Succeed 无需下标面)。
/// handler 内 `use bytecode_operands::<bc>::<NAME>;` 即 C++ 端
/// `Get<NAME>(pc, no_gc)` 的等价。
pub mod bytecode_operands {
  /// kPushBacktrack (bytecodes.h:96-98)
  pub mod push_backtrack {
    pub const LABEL: usize = 0;
  }
  /// kWriteCurrentPositionToRegister (bytecodes.h:99-100)
  pub mod write_current_position_to_register {
    pub const REGISTER_INDEX: usize = 0;
    pub const CP_OFFSET: usize = 1;
  }
  /// kReadCurrentPositionFromRegister (bytecodes.h:101-102)
  pub mod read_current_position_from_register {
    pub const REGISTER_INDEX: usize = 0;
  }
  /// kWriteStackPointerToRegister (bytecodes.h:103-104)
  pub mod write_stack_pointer_to_register {
    pub const REGISTER_INDEX: usize = 0;
  }
  /// kReadStackPointerFromRegister (bytecodes.h:105-106)
  pub mod read_stack_pointer_from_register {
    pub const REGISTER_INDEX: usize = 0;
  }
  /// kSetRegister (bytecodes.h:107-109)
  pub mod set_register {
    pub const REGISTER_INDEX: usize = 0;
    pub const VALUE: usize = 1;
  }
  /// kClearRegisters (bytecodes.h:110-112)
  pub mod clear_registers {
    pub const FROM_REGISTER: usize = 0;
    pub const TO_REGISTER: usize = 1;
  }
  /// kAdvanceRegister (bytecodes.h:113-114)
  pub mod advance_register {
    pub const REGISTER_INDEX: usize = 0;
    pub const BY: usize = 1;
  }
  /// kPushRegister (bytecodes.h:118-120)
  pub mod push_register {
    pub const REGISTER_INDEX: usize = 0;
    pub const STACK_CHECK: usize = 1;
  }
  /// kPopRegister (bytecodes.h:121)
  pub mod pop_register {
    pub const REGISTER_INDEX: usize = 0;
  }
  /// kAdvanceCurrentPosition (bytecodes.h:125)
  pub mod advance_current_position {
    pub const BY: usize = 0;
  }
  /// kGoTo (bytecodes.h:127)
  pub mod go_to {
    pub const LABEL: usize = 0;
  }
  /// kAdvanceCpAndGoto (bytecodes.h:222-223)
  pub mod advance_cp_and_goto {
    pub const BY: usize = 0;
    pub const ON_GOTO: usize = 1;
  }
  /// kCheckFixedLengthLoop (bytecodes.h:178-180)
  pub mod check_fixed_length_loop {
    pub const ON_TOS_EQUALS_CURRENT_POSITION: usize = 0;
  }
  /// kBacktrack (bytecodes.h:189-190)
  pub mod backtrack {
    pub const RETURN_CODE: usize = 0;
  }
  /// kIfRegisterLT (bytecodes.h:164-165)
  pub mod if_register_lt {
    pub const REGISTER_INDEX: usize = 0;
    pub const COMPARAND: usize = 1;
    pub const ON_LESS_THAN: usize = 2;
  }
  /// kIfRegisterGE (bytecodes.h:166-168)
  pub mod if_register_ge {
    pub const REGISTER_INDEX: usize = 0;
    pub const COMPARAND: usize = 1;
    pub const ON_GREATER_OR_EQUAL: usize = 2;
  }
  /// kIfRegisterEqPos (bytecodes.h:169)
  pub mod if_register_eq_pos {
    pub const REGISTER_INDEX: usize = 0;
    pub const ON_EQ: usize = 1;
  }
  // ── P-3b 字符装载/比较/表查/类集 (22) — REG-1b3 ──────────────────────────
  /// kLoadCurrentCharacter (bytecodes.h:124-125)
  pub mod load_current_character {
    pub const CP_OFFSET: usize = 0;
    pub const BOUNDS_CHECK_OFFSET: usize = 1;
    pub const ON_FAILURE: usize = 2;
  }
  /// kLoadCurrentCharacterUnchecked (bytecodes.h:192)
  pub mod load_current_character_unchecked {
    pub const CP_OFFSET: usize = 0;
  }
  /// kLoad2CurrentChars (bytecodes.h:199-201)
  pub mod load2_current_chars {
    pub const CP_OFFSET: usize = 0;
    pub const BOUNDS_CHECK_OFFSET: usize = 1;
    pub const ON_FAILURE: usize = 2;
  }
  /// kLoad2CurrentCharsUnchecked (bytecodes.h:203)
  pub mod load2_current_chars_unchecked {
    pub const CP_OFFSET: usize = 0;
  }
  /// kLoad4CurrentChars (bytecodes.h:205-207)
  pub mod load4_current_chars {
    pub const CP_OFFSET: usize = 0;
    pub const BOUNDS_CHECK_OFFSET: usize = 1;
    pub const ON_FAILURE: usize = 2;
  }
  /// kLoad4CurrentCharsUnchecked (bytecodes.h:209)
  pub mod load4_current_chars_unchecked {
    pub const CP_OFFSET: usize = 0;
  }
  /// kCheck4Chars (bytecodes.h:211-212)
  pub mod check4_chars {
    pub const CHARACTERS: usize = 0;
    pub const ON_EQUAL: usize = 1;
  }
  /// kCheckCharacter (bytecodes.h:136-137)
  pub mod check_character {
    pub const CHARACTER: usize = 0;
    pub const ON_EQUAL: usize = 1;
  }
  /// kCheckNot4Chars (bytecodes.h:213-214)
  pub mod check_not4_chars {
    pub const CHARACTERS: usize = 0;
    pub const ON_NOT_EQUAL: usize = 1;
  }
  /// kCheckNotCharacter (bytecodes.h:138-139)
  pub mod check_not_character {
    pub const CHARACTER: usize = 0;
    pub const ON_NOT_EQUAL: usize = 1;
  }
  /// kAndCheck4Chars (bytecodes.h:215-217)
  pub mod and_check4_chars {
    pub const CHARACTERS: usize = 0;
    pub const MASK: usize = 1;
    pub const ON_EQUAL: usize = 2;
  }
  /// kCheckCharacterAfterAnd (bytecodes.h:144-146)
  pub mod check_character_after_and {
    pub const CHARACTER: usize = 0;
    pub const MASK: usize = 1;
    pub const ON_EQUAL: usize = 2;
  }
  /// kAndCheckNot4Chars (bytecodes.h:218-220)
  pub mod and_check_not4_chars {
    pub const CHARACTERS: usize = 0;
    pub const MASK: usize = 1;
    pub const ON_NOT_EQUAL: usize = 2;
  }
  /// kCheckNotCharacterAfterAnd (bytecodes.h:148-150)
  pub mod check_not_character_after_and {
    pub const CHARACTER: usize = 0;
    pub const MASK: usize = 1;
    pub const ON_NOT_EQUAL: usize = 2;
  }
  /// kCheckNotCharacterAfterMinusAnd (bytecodes.h:151-153)
  pub mod check_not_character_after_minus_and {
    pub const CHARACTER: usize = 0;
    pub const MINUS: usize = 1;
    pub const MASK: usize = 2;
    pub const ON_NOT_EQUAL: usize = 3;
  }
  /// kCheckCharacterInRange (bytecodes.h:155-156)
  pub mod check_character_in_range {
    pub const FROM: usize = 0;
    pub const TO: usize = 1;
    pub const ON_IN_RANGE: usize = 2;
  }
  /// kCheckCharacterNotInRange (bytecodes.h:158-159)
  pub mod check_character_not_in_range {
    pub const FROM: usize = 0;
    pub const TO: usize = 1;
    pub const ON_NOT_IN_RANGE: usize = 2;
  }
  /// kCheckBitInTable (bytecodes.h:197-198)
  pub mod check_bit_in_table {
    pub const ON_BIT_SET: usize = 0;
    pub const TABLE: usize = 1;
  }
  /// kCheckCharacterLT (bytecodes.h:161-162)
  pub mod check_character_lt {
    pub const LIMIT: usize = 0;
    pub const ON_LESS: usize = 1;
  }
  /// kCheckCharacterGT (bytecodes.h:163-164)
  pub mod check_character_gt {
    pub const LIMIT: usize = 0;
    pub const ON_GREATER: usize = 1;
  }
  /// kCheckPosition (bytecodes.h:130-131)
  pub mod check_position {
    pub const CP_OFFSET: usize = 0;
    pub const ON_FAILURE: usize = 1;
  }
  /// kCheckSpecialClassRanges (bytecodes.h:132-134)
  pub mod check_special_class_ranges {
    pub const CHARACTER_SET: usize = 0;
    pub const ON_NO_MATCH: usize = 1;
  }
  // ── P-3c 反向引用/锚点 (9) — REG-1b3 ──────────────────────────────────────
  /// kCheckNotBackRef (bytecodes.h:226-227)
  pub mod check_not_back_ref {
    pub const START_REG: usize = 0;
    pub const ON_NOT_EQUAL: usize = 1;
  }
  /// kCheckNotBackRefNoCase (bytecodes.h:228-229)
  pub mod check_not_back_ref_no_case {
    pub const START_REG: usize = 0;
    pub const ON_NOT_EQUAL: usize = 1;
  }
  /// kCheckNotBackRefNoCaseUnicode (bytecodes.h:230-231)
  pub mod check_not_back_ref_no_case_unicode {
    pub const START_REG: usize = 0;
    pub const ON_NOT_EQUAL: usize = 1;
  }
  /// kCheckNotBackRefBackward (bytecodes.h:232-233)
  pub mod check_not_back_ref_backward {
    pub const START_REG: usize = 0;
    pub const ON_NOT_EQUAL: usize = 1;
  }
  /// kCheckNotBackRefNoCaseBackward (bytecodes.h:234-235)
  pub mod check_not_back_ref_no_case_backward {
    pub const START_REG: usize = 0;
    pub const ON_NOT_EQUAL: usize = 1;
  }
  /// kCheckNotBackRefNoCaseUnicodeBackward (bytecodes.h:236-237)
  pub mod check_not_back_ref_no_case_unicode_backward {
    pub const START_REG: usize = 0;
    pub const ON_NOT_EQUAL: usize = 1;
  }
  /// kCheckAtStart (bytecodes.h:171-172)
  pub mod check_at_start {
    pub const CP_OFFSET: usize = 0;
    pub const ON_AT_START: usize = 1;
  }
  /// kCheckNotAtStart (bytecodes.h:173-174)
  pub mod check_not_at_start {
    pub const CP_OFFSET: usize = 0;
    pub const ON_NOT_AT_START: usize = 1;
  }
  /// kSetCurrentPositionFromEnd (bytecodes.h:179)
  pub mod set_current_position_from_end {
    pub const BY: usize = 0;
  }
  // ── P-3d peephole skip 系 (7) — REG-1b3 ───────────────────────────────────
  /// kSkipUntilBitInTable (bytecodes.h:248-254)
  pub mod skip_until_bit_in_table {
    pub const CP_OFFSET: usize = 0;
    pub const ADVANCE_BY: usize = 1;
    pub const TABLE: usize = 2;
    pub const BOUNDS_CHECK_OFFSET: usize = 3;
    pub const ON_MATCH: usize = 4;
    pub const ON_NO_MATCH: usize = 5;
  }
  /// kSkipUntilCharAnd (bytecodes.h:258-264)
  pub mod skip_until_char_and {
    pub const CP_OFFSET: usize = 0;
    pub const ADVANCE_BY: usize = 1;
    pub const CHARACTER: usize = 2;
    pub const MASK: usize = 3;
    pub const BOUNDS_CHECK_OFFSET: usize = 4;
    pub const ON_MATCH: usize = 5;
    pub const ON_NO_MATCH: usize = 6;
  }
  /// kSkipUntilChar (bytecodes.h:267-273)
  pub mod skip_until_char {
    pub const CP_OFFSET: usize = 0;
    pub const ADVANCE_BY: usize = 1;
    pub const CHARACTER: usize = 2;
    pub const BOUNDS_CHECK_OFFSET: usize = 3;
    pub const ON_MATCH: usize = 4;
    pub const ON_NO_MATCH: usize = 5;
  }
  /// kSkipUntilCharOrChar (bytecodes.h:277-284)
  pub mod skip_until_char_or_char {
    pub const CP_OFFSET: usize = 0;
    pub const ADVANCE_BY: usize = 1;
    pub const CHAR1: usize = 2;
    pub const CHAR2: usize = 3;
    pub const BOUNDS_CHECK_OFFSET: usize = 4;
    pub const ON_MATCH: usize = 5;
    pub const ON_NO_MATCH: usize = 6;
  }
  /// kSkipUntilGtOrNotBitInTable (bytecodes.h:287-294)
  pub mod skip_until_gt_or_not_bit_in_table {
    pub const CP_OFFSET: usize = 0;
    pub const ADVANCE_BY: usize = 1;
    pub const CHARACTER: usize = 2;
    pub const TABLE: usize = 3;
    pub const BOUNDS_CHECK_OFFSET: usize = 4;
    pub const ON_MATCH: usize = 5;
    pub const ON_NO_MATCH: usize = 6;
  }
  /// kSkipUntilOneOfMasked (bytecodes.h:299-308)
  pub mod skip_until_one_of_masked {
    pub const CP_OFFSET: usize = 0;
    pub const ADVANCE_BY: usize = 1;
    pub const BOTH_CHARS: usize = 2;
    pub const BOTH_MASK: usize = 3;
    pub const MAX_OFFSET: usize = 4;
    pub const CHARS1: usize = 5;
    pub const MASK1: usize = 6;
    pub const CHARS2: usize = 7;
    pub const MASK2: usize = 8;
    pub const ON_MATCH1: usize = 9;
    pub const ON_MATCH2: usize = 10;
    pub const ON_FAILURE: usize = 11;
  }
  /// kSkipUntilOneOfMasked3 (bytecodes.h:315-327) — 20 操作数, 全集最大
  pub mod skip_until_one_of_masked3 {
    pub const BC0_CP_OFFSET: usize = 0;
    pub const BC0_ADVANCE_BY: usize = 1;
    pub const BC0_TABLE: usize = 2;
    pub const BC1_BOUNDS_CHECK_OFFSET: usize = 3;
    pub const BC1_ON_FAILURE: usize = 4;
    pub const BC1_CP_OFFSET: usize = 5;
    pub const BC2_CHARACTERS: usize = 6;
    pub const BC2_MASK: usize = 7;
    pub const BC3_BY: usize = 8;
    pub const BC4_BOUNDS_CHECK_OFFSET: usize = 9;
    pub const BC4_CP_OFFSET: usize = 10;
    pub const BC5_CHARACTERS: usize = 11;
    pub const BC5_MASK: usize = 12;
    pub const BC5_ON_EQUAL: usize = 13;
    pub const BC6_CHARACTERS: usize = 14;
    pub const BC6_MASK: usize = 15;
    pub const BC6_ON_EQUAL: usize = 16;
    pub const BC7_CHARACTERS: usize = 17;
    pub const BC7_MASK: usize = 18;
    pub const FALLTHROUGH_JUMP_TARGET: usize = 19;
  }
}

// ── RawMatch 派发主体 (cc:501-1183 骨架) ───────────────────────────────────
// 原版 RawMatch<Char> 形参面 (cc:501-507) 的非 isolate 部分逐参承接:
//   code_array (cc:502) → `code: &[u8]` (cc:560 *code_array->begin())
//   subject_string (cc:503) → [P-1] 推迟 (Tagged<String>/FlatContent 解引用,
//     cc:1298-1338 MatchInternal 侧), 此处只注入其视图 subject (cc:504)
//   subject (cc:504) → `S: Subject` (base::Vector<const Char> 等价)
//   output_registers/output_register_count (cc:505) → `output: &mut [i32]`
//     (原版裸指针 + 计数; Rust 以切片借用承接, InterpreterRegisters 构造期取 len)
//   total_register_count/current/current_char/call_origin/backtrack_limit
//     (cc:505-507) → RawMatchParams
// isolate (cc:502) 消费面 = [P-2] 三项 (MaybeThrowStackOverflow cc:227 /
// HandleInterrupts cc:260 / counters cc:663-664,668-669) → `H: Hooks` 注入,
// P-1 落地时由真实 Isolate 实现替换 (见模块头推迟清单)。

/// 原版 `base::Vector<const Char> subject` (cc:504) 的 Rust 承接 — 两态
/// (uint8_t = Latin1 / base::uc16 = Unicode, 对应 RawMatch template <Char> cc:501)。
/// 索引单位与 current 一致, 均为 Char 元素个 (非字节偏移); 原版 bits_per_char
/// (cc:105-108) 只在 Load2/Load4 的移位量上消费 (已落纯函数), 与 current 换算无关。
pub trait Subject {
  /// 原版 template <Char> 的 sizeof(Char)==1 判定面 (cc:421 is_one_byte /
  /// cc:726 等 DCHECK_EQ(1, sizeof(Char))) — 编译期常量, 两 impl 各自定值
  const ONE_BYTE: bool;

  /// subject.length() (cc:379 DCHECK 面 / cc:700 等装载面)
  fn len(&self) -> i32;

  /// rust 惯例补位 (clippy len_without_is_empty): 原版 `base::Vector`
  /// (v8/src/base/vector.h) 无 IsEmpty 方法, regexp-interpreter.cc 全文件亦无
  /// 空 subject 快路径消费点 (length() 仅作边界上界, 空串自然由比较式短路),
  /// 故无 1:1 原版对应位, 转发 len() == 0 的纯语义判空
  #[inline]
  #[must_use]
  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// subject[index] — P-3b 装载 handler 的接驳点 (cc:696-710 起);
  /// 调用方须先过 index_is_in_bounds (cc:329-332)
  fn char_at(&self, index: i32) -> u32;

  /// Load2Characters(subject, index) (cc:110-113) — 两态共用实现:
  /// s[i] | s[i+1] << BitsPerChar<Char> 与已落纯函数
  /// load_2_characters_one/two_byte (cc:105-113) 同式 (元素读出再移位)
  #[inline]
  #[must_use]
  fn load2_at(&self, index: i32) -> u32 {
    let bits = if Self::ONE_BYTE {
      bits_per_char_one_byte()
    } else {
      bits_per_char_two_byte()
    };
    self.char_at(index) | (self.char_at(index + 1) << bits)
  }

  /// Load4Characters(subject, index) (cc:115-122) — 两字节态原版
  /// UNREACHABLE (cc:120-122), 由各 impl 定夺
  fn load4_at(&self, index: i32) -> u32;

  /// CompareCharsEqual(&subject[a], &subject[b], len) (utils.h:611-617,
  /// 消费点 cc:885/899) — 逐元素相等; 调用方保证两窗口均在 subject 内
  #[must_use]
  fn chars_equal(&self, a: i32, b: i32, len: i32) -> bool {
    (0..len).all(|i| self.char_at(a + i) == self.char_at(b + i))
  }

  /// BackRefMatchesNoCase (cc:46-81) 的模板分发面 —
  /// &[u8]: Some(Latin1 核 cc:63-81, unicode 标志无差别);
  /// &[u16]: None (cc:46-61 → CaseInsensitiveCompare{Unicode,NonUnicode}
  /// 依赖 ICU 折叠, P-3c-icu 运行态通道显式登记, 禁伪装)
  #[must_use]
  fn backref_matches_no_case(
    &self,
    from: i32,
    current: i32,
    len: i32,
    unicode: bool,
  ) -> Option<bool>;
}

/// 一字节 (Latin1) 态: 原版 `base::Vector<const uint8_t>`
impl Subject for &[u8] {
  const ONE_BYTE: bool = true;

  /// subject.length() — 显式走 `<[u8]>`, 免与 trait 方法 `len` 递归
  fn len(&self) -> i32 {
    <[u8]>::len(self) as i32
  }

  fn char_at(&self, index: i32) -> u32 {
    debug_assert!(index_is_in_bounds(index, self.len()));
    u32::from(self[usize::try_from(index).unwrap_or(usize::MAX)])
  }

  /// Load4Characters<uint8_t> (cc:115-118) — 定宽移位组装, 复用已落纯函数
  fn load4_at(&self, index: i32) -> u32 {
    load_4_characters_one_byte(self, usize::try_from(index).unwrap_or(usize::MAX))
  }

  /// uint8 重载 (cc:63-81): Isolate 未使用 (删参), unicode 标志无差别
  fn backref_matches_no_case(
    &self,
    from: i32,
    current: i32,
    len: i32,
    _unicode: bool,
  ) -> Option<bool> {
    let (Ok(from), Ok(current), Ok(len)) = (
      usize::try_from(from),
      usize::try_from(current),
      usize::try_from(len),
    ) else {
      // 调用方 (exec_backref_ops) 已保证 from/current ≥ 0 且 len > 0;
      // 越出 i32 正域即派发级错误, 以 None 显式登记而非静默
      return None;
    };
    Some(back_ref_matches_no_case_latin1(from, current, len, self))
  }
}

/// 两字节 (UC16) 态: 原版 `base::Vector<const base::uc16>`
impl Subject for &[u16] {
  const ONE_BYTE: bool = false;

  fn len(&self) -> i32 {
    <[u16]>::len(self) as i32
  }

  fn char_at(&self, index: i32) -> u32 {
    debug_assert!(index_is_in_bounds(index, self.len()));
    u32::from(self[usize::try_from(index).unwrap_or(usize::MAX)])
  }

  /// Load4Characters<uc16> (cc:120-122): UNREACHABLE → v8_fatal
  /// (生成器只在单字节态发 Load4 系列; 到达即码流/派发级错误)
  fn load4_at(&self, index: i32) -> u32 {
    load_4_characters_two_byte(self, usize::try_from(index).unwrap_or(usize::MAX))
  }

  /// uc16 重载 (cc:46-61) 依赖 RegExpMacroAssembler::CaseInsensitiveCompare*
  /// (regexp-macro-assembler.cc:60/91) → ICU CaseFolding EquivalenceKey
  /// (special-case.h:66-80) + 构建期生成物 special-case.cc 缺席 →
  /// None = P-3c-icu 运行态推迟通道 (调用方 Halt NotImplemented, 禁伪装)
  fn backref_matches_no_case(
    &self,
    _from: i32,
    _current: i32,
    _len: i32,
    _unicode: bool,
  ) -> Option<bool> {
    None
  }
}

/// [P-2] isolate 面可注入承接 (原版自由函数本体推迟, 见模块头):
/// 三个调用点在 handler 侧逐名 1:1, 落地批只需把本 trait 换成真实 Isolate 实现。
pub trait Hooks {
  /// 原版 MaybeThrowStackOverflow (cc:227-234) — 调用点 cc:582/589/596
  /// (PushCurrentPosition/PushBacktrack/PushRegister 的 push 预算失败分支);
  /// ThrowStackOverflow 本体 (cc:215-223, isolate->StackOverflow()) 属 P-2
  fn maybe_throw_stack_overflow(&mut self, call_origin: CallOrigin) -> Result;

  /// 原版 HandleInterrupts (cc:260-319) — 调用点 cc:649 (Backtrack);
  /// 返回非 Success 即终止 (原版 cc:650-652)。code/subject 引用更新本体
  /// (UpdateCodeAndSubjectReferences cc:236-256) 属 P-2
  fn handle_interrupts(&mut self, pc: u32, current: i32, backtrack_count: u32) -> Result;

  /// 原版 `isolate->counters()->regexp_backtracks()->AddSample(backtrack_count)`
  /// — 调用点 cc:663-664 (Fail) / cc:668-669 (Succeed)
  fn add_backtrack_sample(&mut self, backtrack_count: u32);
}

/// 无 Isolate 时的最小注入实现: 只承接原版**非抛出**分支的返回码并记录调用点,
/// 使 cc 侧三个 isolate 消费点可被单测逐项断言 (抛出/中断本体仍属 P-2, 见上)
#[derive(Debug, Default)]
pub struct TraceHooks {
  /// MaybeThrowStackOverflow 调用点计数 (cc:582/589/596)
  pub stack_overflow_calls: u32,
  /// 最近一次溢出调用点的 CallOrigin (原版 cc:229 分支判定依据)
  pub last_overflow_origin: Option<CallOrigin>,
  /// HandleInterrupts 调用点计数 (cc:649)
  pub interrupt_calls: u32,
  /// Fail/Succeed 的 regexp_backtracks 采样序列 (cc:663-669)
  pub backtrack_samples: Vec<u32>,
  /// Some(n): 第 n 次 Backtrack 之后返回 Retry — 单测用于驱动原版 cc:650-652
  /// 的非 SUCCESS 直返路径 (None = 恒 Success, 即无待处理中断的常态)
  pub interrupt_after: Option<u32>,
}

impl Hooks for TraceHooks {
  fn maybe_throw_stack_overflow(&mut self, call_origin: CallOrigin) -> Result {
    self.stack_overflow_calls += 1;
    self.last_overflow_origin = Some(call_origin);
    // 两支共同返回 EXCEPTION (cc:231/233); kFromRuntime 的抛出动作本体
    // (cc:215-223) 由 P-1 调用方以真实 Isolate 承接, 此处不伪造
    Result::Exception
  }

  fn handle_interrupts(&mut self, _pc: u32, _current: i32, backtrack_count: u32) -> Result {
    self.interrupt_calls += 1;
    match self.interrupt_after {
      Some(n) if backtrack_count > n => Result::Retry,
      _ => Result::Success,
    }
  }

  fn add_backtrack_sample(&mut self, backtrack_count: u32) {
    self.backtrack_samples.push(backtrack_count);
  }
}

/// 原版 JSRegExp::kNoBacktrackLimit (objects/js-regexp.h:157) = 0 — Backtrack
/// handler 的 static_assert 前提 (cc:644): limit=0 时 `++count == limit` 永不成立
pub const K_NO_BACKTRACK_LIMIT: u32 = 0;
const _: () = assert!(K_NO_BACKTRACK_LIMIT == 0);

/// 原版 handler 内 `ADVANCE()` / `SET_PC_FROM_OFFSET()` / `return` 三式 (cc:364-373)
/// 的派发承接: 前者写 next_pc = pc + Size, 次者写 next_pc = code_base + offset,
/// 末者直接结束本次执行
enum Flow {
  /// ADVANCE() (cc:367-369)
  Advance,
  /// SET_PC_FROM_OFFSET(offset) (cc:371-373) — offset 为 code_base 相对偏移
  Jump(u32),
  /// 原版 handler 的 `return IrregexpInterpreter::Result` (cc:582/589/596/647/
  /// 652/665/670) 与未实现分支的显式承接
  Halt(Step),
}

/// RawMatch 的出口面 (原版 return 语义 + 本票显式登记的未落地分支; 禁 todo!/panic 伪装)
#[derive(Debug, PartialEq, Eq)]
pub enum Step {
  /// 原版 `return IrregexpInterpreter::Result` — Succeed/Fail/Backtrack 达
  /// backtrack_limit/栈溢出/中断
  Done(Result),
  /// 运行态推迟通道 — 唯一来源是 P-3c-icu: uc16 态 BackRefMatchesNoCase
  /// (cc:46-61) 依赖 ICU 大小写折叠 (special-case.cc 构建期生成物缺席),
  /// 两字节 NoCase 反向引用比较落此显式登记; cc_line 双引用原版行号, batch
  /// 固定 "P-3c-icu"。P-3a/P-3b/P-3c/P-3d 六组 handler 已全部落地, 该分支
  /// 不再承担"批段未落地"台账职能 (原 deferred_handler_of 已退役)
  NotImplemented {
    bytecode: Bytecode,
    cc_line: u32,
    batch: &'static str,
  },
  /// Backtrack 的 return_code 操作数不是协议内返回码 — 原版 cc:646 为
  /// `static_cast<Result>(int)` 盲转 (生成器只发 RE_FAILURE/RE_FALLBACK_TO_
  /// EXPERIMENTAL, bytecode-generator.cc:307-311); Rust 以显式错误取代静默 UB
  IllegalResultCode { value: i32 },
  /// 非法码流: pc 越出 code 数组 / 指令帧不完整 (原版定长 TrustedByteArray
  /// cc:560 + 帧内 reinterpret_cast 的结构性前提, 越界即 UB)
  IllegalCode { pc: u32, reason: &'static str },
  /// 测试护栏: 超过注入的 step_limit (原版无对应物; None 时永不触发)
  StepLimitExceeded { steps: u32 },
}

/// 原版 RawMatch 形参中的标量面 (cc:505-507) + preamble 构造入参 (cc:563)
#[derive(Debug, Clone, Copy)]
pub struct RawMatchParams {
  /// total_register_count (cc:505) — 寄存器总数, 须 ≥ output 切片长度
  /// (原版 SBXCHECK_GE cc:187-188)
  pub total_register_count: i32,
  /// current (cc:506) — 起始位置 (Match 侧为 start_position, cc:1238)
  pub current: i32,
  /// current_char (cc:506) — 初始当前字符 (P-3b 装载面的起点, 初值 0)
  pub current_char: u32,
  /// call_origin (cc:506) — 栈溢出抛出归属 (cc:217/229)
  pub call_origin: CallOrigin,
  /// backtrack_limit (cc:507) — 0 = K_NO_BACKTRACK_LIMIT (永不触发)
  pub backtrack_limit: u32,
  /// 测试护栏步数上限; None = 关闭 (原版无该量)
  pub step_limit: Option<u32>,
}

impl Default for RawMatchParams {
  fn default() -> Self {
    Self {
      total_register_count: 2,
      current: 0,
      current_char: 0,
      call_origin: CallOrigin::FromRuntime,
      backtrack_limit: K_NO_BACKTRACK_LIMIT,
      step_limit: None,
    }
  }
}

/// 原版 IrregexpInterpreter::RawMatch (cc:501-1196) 的可测形态:
/// 帧状态 (pc/current/backtrack_count) + 寄存器 + 回溯栈 + 注入面同构承载。
/// 派方式为 switch 等价物 (cc:349-357 兜底轨; computed-goto cc:338-345/510-556
/// 为性能轨, rust 不落地, 前席裁定)。
pub struct RawMatch<'a, S: Subject, H: Hooks> {
  /// 原版 `(*code_array)->begin()` (cc:560) — 定长字节码数组内容
  pub code: &'a [u8],
  /// 原版 subject (cc:504)
  pub subject: S,
  /// 原版 registers (cc:563) — InterpreterRegisters (cc:172-213 已落)
  pub registers: InterpreterRegisters,
  /// 原版 output_registers 裸指针 (cc:505/564) — Succeed 定长拷贝目的 (cc:202-205)
  pub output: &'a mut [i32],
  /// 原版 backtrack_stack (cc:564)
  pub stack: BacktrackStack,
  /// 原版 pc (cc:560) — code_base 相对偏移 (0 起点)
  pub pc: u32,
  /// 原版 current (cc:506/376-381)
  pub current: i32,
  /// 原版 current_char (cc:506)
  pub current_char: u32,
  /// 原版 call_origin (cc:506)
  pub call_origin: CallOrigin,
  /// 原版 backtrack_limit (cc:507)
  pub backtrack_limit: u32,
  /// 原版 backtrack_count (cc:566) — Fail/Succeed 采样与 limit 比较用
  pub backtrack_count: u32,
  /// 测试护栏 (原版无)
  pub step_limit: Option<u32>,
  /// [P-2] isolate 面注入
  pub hooks: H,
}

impl<'a, S: Subject, H: Hooks> RawMatch<'a, S, H> {
  /// 原版 preamble (cc:560-566): pc = code_base = code_array->begin();
  /// InterpreterRegisters(total, output_registers, output_register_count);
  /// BacktrackStack 空栈; backtrack_count = 0
  pub fn new(
    code: &'a [u8],
    subject: S,
    output: &'a mut [i32],
    params: RawMatchParams,
    hooks: H,
  ) -> Self {
    let registers = InterpreterRegisters::new(params.total_register_count, output.len() as i32);
    Self {
      code,
      subject,
      registers,
      output,
      stack: BacktrackStack::new(),
      pc: 0,
      current: params.current,
      current_char: params.current_char,
      call_origin: params.call_origin,
      backtrack_limit: params.backtrack_limit,
      backtrack_count: 0,
      step_limit: params.step_limit,
      hooks,
    }
  }

  /// SET_CURRENT_POSITION(value) (cc:376-380) — 经已落纯函数带 subject 长度 DCHECK
  #[inline]
  fn set_current(&mut self, value: i32) {
    self.current = set_current_position(value, self.subject.len());
  }

  /// ADVANCE_CURRENT_POSITION(by) (cc:381)
  #[inline]
  fn advance_current(&mut self, by: i32) {
    self.current = advance_current_position(self.current, by, self.subject.len());
  }

  /// MaybeThrowStackOverflow (cc:227-234) 的三个调用点共用出口 (cc:582/589/596)
  #[inline]
  fn on_stack_overflow(&mut self) -> Step {
    Step::Done(self.hooks.maybe_throw_stack_overflow(self.call_origin))
  }

  /// 原版 RawMatch 主循环 (cc:568-575 while(true) + DECODE/DISPATCH 双宏)。
  /// [P-2] 的 code/subject 引用更新 (HandleInterrupts cc:260-319 内
  /// UpdateCodeAndSubjectReferences cc:236-256) 由 Hooks 注入承接, pc 恒为
  /// code_base 相对偏移, 故重定位不需在此改 code 切片。
  pub fn run(&mut self) -> Step {
    let mut steps: u32 = 0;
    loop {
      if let Some(limit) = self.step_limit {
        if steps >= limit {
          return Step::StepLimitExceeded { steps };
        }
        steps += 1;
      }
      let code = self.code;
      // DECODE() (cc:340-345): Bytecodes::FromPtr(next_pc) — 取 pc 处 opcode 字节
      let Some(&opcode) = code.get(self.pc as usize) else {
        return Step::IllegalCode {
          pc: self.pc,
          reason: "opcode 读越出 code 数组 (cc:560 定长前提)",
        };
      };
      // 原版派发表 [kCount, kPaddedBytecodeCount) 与 kBreak 槽统一落 handler
      // cc:577 UNREACHABLE (cc:517-530 填表 + cc:519 掩码); 此处无效字节 (≥kCount)
      // 与 kBreak 同归该 fatal 面 (from_byte 的 Option 承接 IsValid, h:371)
      let Some(bc) = Bytecode::from_byte(opcode).filter(|b| *b != Bytecode::Break) else {
        v8_fatal(format_args!(
          "UNREACHABLE: opcode {opcode} → kBreak (cc:577)"
        ))
      };
      let size = bytecode_size(bc);
      // 指令帧界: 原版定长数组内 reinterpret_cast 取操作数 (inl.h:246-252)
      if self.pc as usize + size as usize > code.len() {
        return Step::IllegalCode {
          pc: self.pc,
          reason: "指令帧越出 code 数组",
        };
      }
      match self.exec(bc, code) {
        // ADVANCE() (cc:367-369)
        Flow::Advance => self.pc = advance_pc(self.pc, size),
        // SET_PC_FROM_OFFSET() (cc:371-373)
        Flow::Jump(offset) => self.pc = pc_from_offset(offset),
        Flow::Halt(step) => return step,
      }
      // DISPATCH() (cc:364-366): switch 兜底轨等价 = 落位 pc 后回到循环体
      // (原版 switch_dispatch_continuation 标号, cc:1192; 其后无指令, 1:1)
    }
  }

  /// 单条指令派发: 全 61 项六组 (P-3a 23 = 栈6/寄存器10/控制7 三组,
  /// P-3b 22 / P-3c 9 / P-3d 7 各一组, REG-1b2/REG-1b3) 均已落地;
  /// 六组皆 None 即派发分组与 61 全集失配 (结构性错误, fatal 禁静默)。
  /// 运行态 NotImplemented 只在 exec_backref_ops 的 P-3c-icu 通道内产生
  #[inline]
  fn exec(&mut self, bc: Bytecode, code: &[u8]) -> Flow {
    self
      .exec_stack_ops(bc, code)
      .or_else(|| self.exec_reg_ops(bc, code))
      .or_else(|| self.exec_ctrl_ops(bc, code))
      .or_else(|| self.exec_char_ops(bc, code))
      .or_else(|| self.exec_backref_ops(bc, code))
      .or_else(|| self.exec_skip_ops(bc, code))
      .unwrap_or_else(|| v8_fatal(format_args!("派发分组与 61 全集失配: {bc:?}")))
  }

  /// 回溯栈面 6 项: PushCurrentPosition cc:578 / PushBacktrack cc:585 /
  /// PushRegister cc:592 / PopCurrentPosition cc:638 / Backtrack cc:643 /
  /// PopRegister cc:657 (cc 源序)
  fn exec_stack_ops(&mut self, bc: Bytecode, code: &[u8]) -> Option<Flow> {
    match bc {
      Bytecode::PushCurrentPosition => {
        // cc:579 ADVANCE(); cc:580-583 push(current) 预算失败 → MaybeThrowStackOverflow
        if !self.stack.push(self.current) {
          return Some(Flow::Halt(self.on_stack_overflow()));
        }
        Some(Flow::Advance)
      }
      Bytecode::PushBacktrack => {
        use self::bytecode_operands::push_backtrack::LABEL;
        // cc:586 ADVANCE(); cc:587-590 push(label) — label 即回溯目标偏移
        // (kJumpTarget 以 int 入栈, cc:126-128 栈通用化注释)
        let label = operand_u32(code, self.pc, bc, LABEL);
        if !self.stack.push(label as i32) {
          return Some(Flow::Halt(self.on_stack_overflow()));
        }
        Some(Flow::Advance)
      }
      Bytecode::PushRegister => {
        use self::bytecode_operands::push_register::{REGISTER_INDEX, STACK_CHECK};
        // cc:593 ADVANCE(); cc:594 USE(stack_check) — 解释器不消费该标志
        // (仅 CSA/宏汇编器侧用), 此处仍按布局解码以保持帧游标一致
        let stack_check = operand_u8(code, self.pc, bc, STACK_CHECK);
        debug_assert!(
          stack_check <= 1,
          "StackCheckFlag 二值 (macro-assembler.h:275)"
        );
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        let value = self.registers.get(register_index);
        // cc:595-598
        if !self.stack.push(value) {
          return Some(Flow::Halt(self.on_stack_overflow()));
        }
        Some(Flow::Advance)
      }
      Bytecode::PopCurrentPosition => {
        // cc:639 ADVANCE(); cc:640 SET_CURRENT_POSITION(backtrack_stack.pop())
        // — 空栈 pop 由 BacktrackStack::peek 的 SBXCHECK(cc:140) 承接 (fatal)
        let value = self.stack.pop();
        self.set_current(value);
        Some(Flow::Advance)
      }
      Bytecode::Backtrack => {
        use self::bytecode_operands::backtrack::RETURN_CODE;
        // cc:644 static_assert(kNoBacktrackLimit == 0) — 文件级 const assert
        // cc:645-648: ++count == limit → return (Result)return_code
        self.backtrack_count += 1;
        if self.backtrack_count == self.backtrack_limit {
          let value = i32::from(operand_i16(code, self.pc, bc, RETURN_CODE));
          return Some(Flow::Halt(match Result::from_code(value) {
            Some(result) => Step::Done(result),
            None => Step::IllegalResultCode { value },
          }));
        }
        // cc:650-652: HandleInterrupts 非 SUCCESS 直返 [P-2 注入]
        let interrupted = self
          .hooks
          .handle_interrupts(self.pc, self.current, self.backtrack_count);
        if interrupted != Result::Success {
          return Some(Flow::Halt(Step::Done(interrupted)));
        }
        // cc:653-654 SET_PC_FROM_OFFSET(backtrack_stack.pop())
        let offset = self.stack.pop() as u32;
        Some(Flow::Jump(pc_from_offset(offset)))
      }
      Bytecode::PopRegister => {
        use self::bytecode_operands::pop_register::REGISTER_INDEX;
        // cc:658 ADVANCE(); cc:659 registers[reg] = backtrack_stack.pop()
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        let value = self.stack.pop();
        self.registers.set(register_index, value);
        Some(Flow::Advance)
      }
      _ => None,
    }
  }

  /// 寄存器面 7 项: SetRegister cc:600 / ClearRegisters cc:605 /
  /// AdvanceRegister cc:613 / WriteCurrentPositionToRegister cc:618 /
  /// ReadCurrentPositionFromRegister cc:623 / WriteStackPointerToRegister cc:628 /
  /// ReadStackPointerFromRegister cc:633 (cc 源序)
  fn exec_reg_ops(&mut self, bc: Bytecode, code: &[u8]) -> Option<Flow> {
    match bc {
      Bytecode::SetRegister => {
        use self::bytecode_operands::set_register::{REGISTER_INDEX, VALUE};
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        let value = operand_i32(code, self.pc, bc, VALUE);
        // cc:602
        self.registers.set(register_index, value);
        Some(Flow::Advance)
      }
      Bytecode::ClearRegisters => {
        use self::bytecode_operands::clear_registers::{FROM_REGISTER, TO_REGISTER};
        // kRegister = uint16_t (bytecodes.h:42) → 无符号闭区间
        let from_register = usize::from(operand_u16(code, self.pc, bc, FROM_REGISTER));
        let to_register = usize::from(operand_u16(code, self.pc, bc, TO_REGISTER));
        // cc:607 SBXCHECK_LE(from_register, to_register)
        sbx_check(from_register <= to_register, "from_register <= to_register");
        // cc:608-610 for (uint16_t i = from; i <= to; ++i) — 原版 to = kMaxRegister
        // 时 uint16 回绕致死循环 (UB), Rust 闭区间 range 语义等价且无回绕
        for index in from_register..=to_register {
          self
            .registers
            .set(index, InterpreterRegisters::K_NO_MATCH_VALUE);
        }
        Some(Flow::Advance)
      }
      Bytecode::AdvanceRegister => {
        use self::bytecode_operands::advance_register::{BY, REGISTER_INDEX};
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        let by = i32::from(operand_i16(code, self.pc, bc, BY));
        // cc:615 registers[reg] += by — 原版 int 加法; wrapping_add 与 C++
        // 溢出回绕逐位一致 (禁 debug panic 假绿)
        let value = self.registers.get(register_index).wrapping_add(by);
        self.registers.set(register_index, value);
        Some(Flow::Advance)
      }
      Bytecode::WriteCurrentPositionToRegister => {
        use self::bytecode_operands::write_current_position_to_register::{
          CP_OFFSET, REGISTER_INDEX,
        };
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        // cc:620 registers[reg] = current + cp_offset (cp_offset 限幅 ±32767)
        let value = self.current + cp_offset;
        self.registers.set(register_index, value);
        Some(Flow::Advance)
      }
      Bytecode::ReadCurrentPositionFromRegister => {
        use self::bytecode_operands::read_current_position_from_register::REGISTER_INDEX;
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        // cc:625 SET_CURRENT_POSITION(registers[reg]) — 范围 DCHECK 承自纯函数
        let value = self.registers.get(register_index);
        self.set_current(value);
        Some(Flow::Advance)
      }
      Bytecode::WriteStackPointerToRegister => {
        use self::bytecode_operands::write_stack_pointer_to_register::REGISTER_INDEX;
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        // cc:630 registers[reg] = backtrack_stack.sp()
        let sp = self.stack.sp();
        self.registers.set(register_index, sp);
        Some(Flow::Advance)
      }
      Bytecode::ReadStackPointerFromRegister => {
        use self::bytecode_operands::read_stack_pointer_from_register::REGISTER_INDEX;
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        let sp = self.registers.get(register_index);
        // cc:635 set_sp(registers[reg]); 原版形参 uint32_t (cc:151) — 负值在
        // debug 由 DCHECK_LE(new_sp, sp()) 拦下, Rust 侧 set_sp 常驻检查承接
        self.stack.set_sp(sp as u32);
        Some(Flow::Advance)
      }
      _ => None,
    }
  }

  /// 控制/终止/分支 9 项: Fail cc:662 / Succeed cc:667 /
  /// AdvanceCurrentPosition cc:673 / GoTo cc:678 / AdvanceCpAndGoto cc:682 /
  /// CheckFixedLengthLoop cc:687 / IfRegisterLT cc:856 / IfRegisterGE cc:864 /
  /// IfRegisterEqPos cc:872 (cc 源序)
  fn exec_ctrl_ops(&mut self, bc: Bytecode, code: &[u8]) -> Option<Flow> {
    match bc {
      Bytecode::Fail => {
        // cc:663-664 counters()->regexp_backtracks()->AddSample [P-2 注入]
        self.hooks.add_backtrack_sample(self.backtrack_count);
        Some(Flow::Halt(Step::Done(Result::Failure)))
      }
      Bytecode::Succeed => {
        // cc:668-670 同上采样 + CopyToOutputRegisters (cc:202-205) 后 SUCCESS
        self.hooks.add_backtrack_sample(self.backtrack_count);
        self.copy_registers_to_output();
        Some(Flow::Halt(Step::Done(Result::Success)))
      }
      Bytecode::AdvanceCurrentPosition => {
        use self::bytecode_operands::advance_current_position::BY;
        let by = i32::from(operand_i16(code, self.pc, bc, BY));
        // cc:674-675 ADVANCE(); ADVANCE_CURRENT_POSITION(by)
        self.advance_current(by);
        Some(Flow::Advance)
      }
      Bytecode::GoTo => {
        use self::bytecode_operands::go_to::LABEL;
        // cc:679 SET_PC_FROM_OFFSET(label) — 无 ADVANCE (原版 fallthrough 被覆盖)
        let label = operand_u32(code, self.pc, bc, LABEL);
        Some(Flow::Jump(label))
      }
      Bytecode::AdvanceCpAndGoto => {
        use self::bytecode_operands::advance_cp_and_goto::{BY, ON_GOTO};
        let by = i32::from(operand_i16(code, self.pc, bc, BY));
        let on_goto = operand_u32(code, self.pc, bc, ON_GOTO);
        // cc:683-684: SET_PC_FROM_OFFSET(on_goto) 后 ADVANCE_CURRENT_POSITION(by)
        // — 两式写入顺序与原版一致 (next_pc 只被 SET_PC 定值; 位置推进与
        // 字节宽/位数无关, 一/两字节两态同码, 见 Subject 文档)
        self.advance_current(by);
        Some(Flow::Jump(on_goto))
      }
      Bytecode::CheckFixedLengthLoop => {
        use self::bytecode_operands::check_fixed_length_loop::ON_TOS_EQUALS_CURRENT_POSITION;
        let target = operand_u32(code, self.pc, bc, ON_TOS_EQUALS_CURRENT_POSITION);
        // cc:688-695: current == peek → 跳转并弹出 Tos (空栈 peek 由
        // BacktrackStack SBXCHECK cc:140 承接); 否则 ADVANCE()
        if self.current == self.stack.peek() {
          // 原版弹出 Tos 后弃值 (cc:692-693), Rust 侧显式忽略 (pop 带 must_use)
          let _ = self.stack.pop();
          return Some(Flow::Jump(target));
        }
        Some(Flow::Advance)
      }
      Bytecode::IfRegisterLT => {
        use self::bytecode_operands::if_register_lt::{COMPARAND, ON_LESS_THAN, REGISTER_INDEX};
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        let comparand = operand_i32(code, self.pc, bc, COMPARAND);
        let on_less_than = operand_u32(code, self.pc, bc, ON_LESS_THAN);
        // cc:857-862
        if self.registers.get(register_index) < comparand {
          return Some(Flow::Jump(on_less_than));
        }
        Some(Flow::Advance)
      }
      Bytecode::IfRegisterGE => {
        use self::bytecode_operands::if_register_ge::{
          COMPARAND, ON_GREATER_OR_EQUAL, REGISTER_INDEX,
        };
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        let comparand = operand_i32(code, self.pc, bc, COMPARAND);
        let on_greater_or_equal = operand_u32(code, self.pc, bc, ON_GREATER_OR_EQUAL);
        // cc:865-870
        if self.registers.get(register_index) >= comparand {
          return Some(Flow::Jump(on_greater_or_equal));
        }
        Some(Flow::Advance)
      }
      Bytecode::IfRegisterEqPos => {
        use self::bytecode_operands::if_register_eq_pos::{ON_EQ, REGISTER_INDEX};
        let register_index = usize::from(operand_u16(code, self.pc, bc, REGISTER_INDEX));
        let on_eq = operand_u32(code, self.pc, bc, ON_EQ);
        // cc:873-878: 与 current 比较 (非与常量比较)
        if self.registers.get(register_index) == self.current {
          return Some(Flow::Jump(on_eq));
        }
        Some(Flow::Advance)
      }
      _ => None,
    }
  }

  /// P-3b 字符装载/比较/表查/类集 22 项 (cc:696-1009) — REG-1b3 落地
  /// 装载面: LoadCurrentCharacter cc:696 / LoadCurrentCharacterUnchecked cc:705 /
  /// Load2CurrentChars cc:711 / Load2CurrentCharsUnchecked cc:720 /
  /// Load4CurrentChars cc:726 / Load4CurrentCharsUnchecked cc:736;
  /// 比较面: Check4Chars cc:743 / CheckCharacter cc:751 / CheckNot4Chars cc:759 /
  /// CheckNotCharacter cc:767 / AndCheck4Chars cc:775 / CheckCharacterAfterAnd cc:783 /
  /// AndCheckNot4Chars cc:791 / CheckNotCharacterAfterAnd cc:799 /
  /// CheckNotCharacterAfterMinusAnd cc:807 / CheckCharacterInRange cc:816 /
  /// CheckCharacterNotInRange cc:824 / CheckCharacterLT cc:840 /
  /// CheckCharacterGT cc:848;
  /// 表查面: CheckBitInTable cc:832 (核已落本文件 cc:321-326);
  /// 其他: CheckPosition cc:991 / CheckSpecialClassRanges cc:1000
  /// (核已落本文件 cc:418-497, word 面用 K_WORD_CHARACTER_MAP)
  fn exec_char_ops(&mut self, bc: Bytecode, code: &[u8]) -> Option<Flow> {
    match bc {
      // cc:696-704: bounds_check_offset 界内则 current_char=subject[current+cp_offset]
      Bytecode::LoadCurrentCharacter => {
        use self::bytecode_operands::load_current_character::{
          BOUNDS_CHECK_OFFSET, CP_OFFSET, ON_FAILURE,
        };
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let bounds_check_offset = operand_i32(code, self.pc, bc, BOUNDS_CHECK_OFFSET);
        let on_failure = operand_u32(code, self.pc, bc, ON_FAILURE);
        if !index_is_in_bounds(self.current + bounds_check_offset, self.subject.len()) {
          // cc:698: SET_PC_FROM_OFFSET(on_failure) — 无 ADVANCE
          Some(Flow::Jump(on_failure))
        } else {
          // cc:700-701: ADVANCE(); current_char = subject[current+cp_offset]
          self.current_char = self.subject.char_at(self.current + cp_offset);
          Some(Flow::Advance)
        }
      }
      // cc:705-709: 无界检查（调用方已保证界内）
      Bytecode::LoadCurrentCharacterUnchecked => {
        use self::bytecode_operands::load_current_character_unchecked::CP_OFFSET;
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        // cc:706-708: ADVANCE(); pos = current+cp_offset; current_char = subject[pos]
        self.current_char = self.subject.char_at(self.current + cp_offset);
        Some(Flow::Advance)
      }
      // cc:711-718: bounds_check 后 current_char = Load2Characters(subject, current+cp_offset)
      Bytecode::Load2CurrentChars => {
        use self::bytecode_operands::load2_current_chars::{
          BOUNDS_CHECK_OFFSET, CP_OFFSET, ON_FAILURE,
        };
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let bounds_check_offset = operand_i32(code, self.pc, bc, BOUNDS_CHECK_OFFSET);
        let on_failure = operand_u32(code, self.pc, bc, ON_FAILURE);
        if !index_is_in_bounds(self.current + bounds_check_offset, self.subject.len()) {
          // cc:713: SET_PC_FROM_OFFSET(on_failure) — 无 ADVANCE
          Some(Flow::Jump(on_failure))
        } else {
          // cc:715-716: ADVANCE(); current_char = Load2Characters(subject, current+cp_offset)
          self.current_char = self.subject.load2_at(self.current + cp_offset);
          Some(Flow::Advance)
        }
      }
      // cc:720-724: 无界检查
      Bytecode::Load2CurrentCharsUnchecked => {
        use self::bytecode_operands::load2_current_chars_unchecked::CP_OFFSET;
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        // cc:721-723: ADVANCE(); pos=current+cp_offset; current_char=Load2Characters(subject,pos)
        self.current_char = self.subject.load2_at(self.current + cp_offset);
        Some(Flow::Advance)
      }
      // cc:726-734: DCHECK_EQ(1,sizeof(Char)) (S::ONE_BYTE) + bounds_check + Load4Characters
      Bytecode::Load4CurrentChars => {
        use self::bytecode_operands::load4_current_chars::{
          BOUNDS_CHECK_OFFSET, CP_OFFSET, ON_FAILURE,
        };
        // cc:727 DCHECK_EQ(1, sizeof(Char)): 生成器仅在一字节态发 Load4 系列 (cc:120-122 UNREACHABLE)
        debug_assert!(S::ONE_BYTE, "Load4CurrentChars requires one_byte subject");
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let bounds_check_offset = operand_i32(code, self.pc, bc, BOUNDS_CHECK_OFFSET);
        let on_failure = operand_u32(code, self.pc, bc, ON_FAILURE);
        if !index_is_in_bounds(self.current + bounds_check_offset, self.subject.len()) {
          // cc:729: SET_PC_FROM_OFFSET(on_failure) — 无 ADVANCE
          Some(Flow::Jump(on_failure))
        } else {
          // cc:731-732: ADVANCE(); current_char = Load4Characters(subject, current+cp_offset)
          self.current_char = self.subject.load4_at(self.current + cp_offset);
          Some(Flow::Advance)
        }
      }
      // cc:736-741: DCHECK_EQ(1,sizeof(Char)) + 无界检查
      Bytecode::Load4CurrentCharsUnchecked => {
        use self::bytecode_operands::load4_current_chars_unchecked::CP_OFFSET;
        // cc:738 DCHECK_EQ(1, sizeof(Char))
        debug_assert!(
          S::ONE_BYTE,
          "Load4CurrentCharsUnchecked requires one_byte subject"
        );
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        // cc:737-740: ADVANCE(); pos=current+cp_offset; current_char=Load4Characters(subject,pos)
        self.current_char = self.subject.load4_at(self.current + cp_offset);
        Some(Flow::Advance)
      }
      // cc:743-749: characters == current_char 则跳转，否则 ADVANCE
      Bytecode::Check4Chars => {
        use self::bytecode_operands::check4_chars::{CHARACTERS, ON_EQUAL};
        let characters = operand_u32(code, self.pc, bc, CHARACTERS);
        let on_equal = operand_u32(code, self.pc, bc, ON_EQUAL);
        // cc:744: characters == current_char (4 字符拼成的 u32 整数比较)
        if characters == self.current_char {
          return Some(Flow::Jump(on_equal));
        }
        Some(Flow::Advance)
      }
      // cc:751-757: character == current_char 则跳转，否则 ADVANCE
      Bytecode::CheckCharacter => {
        use self::bytecode_operands::check_character::{CHARACTER, ON_EQUAL};
        let character = u32::from(operand_u16(code, self.pc, bc, CHARACTER));
        let on_equal = operand_u32(code, self.pc, bc, ON_EQUAL);
        // cc:752: character == current_char
        if character == self.current_char {
          return Some(Flow::Jump(on_equal));
        }
        Some(Flow::Advance)
      }
      // cc:759-765: characters != current_char 则跳转，否则 ADVANCE
      Bytecode::CheckNot4Chars => {
        use self::bytecode_operands::check_not4_chars::{CHARACTERS, ON_NOT_EQUAL};
        let characters = operand_u32(code, self.pc, bc, CHARACTERS);
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        // cc:760: characters != current_char
        if characters != self.current_char {
          return Some(Flow::Jump(on_not_equal));
        }
        Some(Flow::Advance)
      }
      // cc:767-773: character != current_char 则跳转，否则 ADVANCE
      Bytecode::CheckNotCharacter => {
        use self::bytecode_operands::check_not_character::{CHARACTER, ON_NOT_EQUAL};
        let character = u32::from(operand_u16(code, self.pc, bc, CHARACTER));
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        // cc:768: character != current_char
        if character != self.current_char {
          return Some(Flow::Jump(on_not_equal));
        }
        Some(Flow::Advance)
      }
      // cc:775-781: characters == (current_char & mask) 则跳转，否则 ADVANCE
      Bytecode::AndCheck4Chars => {
        use self::bytecode_operands::and_check4_chars::{CHARACTERS, MASK, ON_EQUAL};
        let characters = operand_u32(code, self.pc, bc, CHARACTERS);
        let mask = operand_u32(code, self.pc, bc, MASK);
        let on_equal = operand_u32(code, self.pc, bc, ON_EQUAL);
        // cc:776: characters == (current_char & mask)
        if characters == (self.current_char & mask) {
          return Some(Flow::Jump(on_equal));
        }
        Some(Flow::Advance)
      }
      // cc:783-789: character == (current_char & mask) 则跳转，否则 ADVANCE
      Bytecode::CheckCharacterAfterAnd => {
        use self::bytecode_operands::check_character_after_and::{CHARACTER, MASK, ON_EQUAL};
        let character = u32::from(operand_u16(code, self.pc, bc, CHARACTER));
        let mask = operand_u32(code, self.pc, bc, MASK);
        let on_equal = operand_u32(code, self.pc, bc, ON_EQUAL);
        // cc:784: character == (current_char & mask)
        if character == (self.current_char & mask) {
          return Some(Flow::Jump(on_equal));
        }
        Some(Flow::Advance)
      }
      // cc:791-797: characters != (current_char & mask) 则跳转，否则 ADVANCE
      Bytecode::AndCheckNot4Chars => {
        use self::bytecode_operands::and_check_not4_chars::{CHARACTERS, MASK, ON_NOT_EQUAL};
        let characters = operand_u32(code, self.pc, bc, CHARACTERS);
        let mask = operand_u32(code, self.pc, bc, MASK);
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        // cc:792: characters != (current_char & mask)
        if characters != (self.current_char & mask) {
          return Some(Flow::Jump(on_not_equal));
        }
        Some(Flow::Advance)
      }
      // cc:799-805: character != (current_char & mask) 则跳转，否则 ADVANCE
      Bytecode::CheckNotCharacterAfterAnd => {
        use self::bytecode_operands::check_not_character_after_and::{
          CHARACTER, MASK, ON_NOT_EQUAL,
        };
        let character = u32::from(operand_u16(code, self.pc, bc, CHARACTER));
        let mask = operand_u32(code, self.pc, bc, MASK);
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        // cc:800: character != (current_char & mask)
        if character != (self.current_char & mask) {
          return Some(Flow::Jump(on_not_equal));
        }
        Some(Flow::Advance)
      }
      // cc:807-814: character != ((current_char - minus) & mask) 则跳转，否则 ADVANCE
      Bytecode::CheckNotCharacterAfterMinusAnd => {
        use self::bytecode_operands::check_not_character_after_minus_and::{
          CHARACTER, MASK, MINUS, ON_NOT_EQUAL,
        };
        let character = u32::from(operand_u16(code, self.pc, bc, CHARACTER));
        let minus = u32::from(operand_u16(code, self.pc, bc, MINUS));
        let mask = u32::from(operand_u16(code, self.pc, bc, MASK));
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        // cc:809: character != ((current_char - minus) & mask)
        // wrapping_sub 承接原版 int 减法溢出 (禁 debug panic)
        if character != (self.current_char.wrapping_sub(minus) & mask) {
          return Some(Flow::Jump(on_not_equal));
        }
        Some(Flow::Advance)
      }
      // cc:816-822: from <= current_char <= to 则跳转，否则 ADVANCE
      Bytecode::CheckCharacterInRange => {
        use self::bytecode_operands::check_character_in_range::{FROM, ON_IN_RANGE, TO};
        let from = u32::from(operand_u16(code, self.pc, bc, FROM));
        let to = u32::from(operand_u16(code, self.pc, bc, TO));
        let on_in_range = operand_u32(code, self.pc, bc, ON_IN_RANGE);
        // cc:817: from <= current_char && current_char <= to
        if from <= self.current_char && self.current_char <= to {
          return Some(Flow::Jump(on_in_range));
        }
        Some(Flow::Advance)
      }
      // cc:824-830: current_char 不在 [from,to] 则跳转，否则 ADVANCE
      Bytecode::CheckCharacterNotInRange => {
        use self::bytecode_operands::check_character_not_in_range::{FROM, ON_NOT_IN_RANGE, TO};
        let from = u32::from(operand_u16(code, self.pc, bc, FROM));
        let to = u32::from(operand_u16(code, self.pc, bc, TO));
        let on_not_in_range = operand_u32(code, self.pc, bc, ON_NOT_IN_RANGE);
        // cc:825: from > current_char || current_char > to
        if from > self.current_char || self.current_char > to {
          return Some(Flow::Jump(on_not_in_range));
        }
        Some(Flow::Advance)
      }
      // cc:832-838: CheckBitInTable(current_char, table) 为真则跳转，否则 ADVANCE
      Bytecode::CheckBitInTable => {
        use self::bytecode_operands::check_bit_in_table::{ON_BIT_SET, TABLE};
        let on_bit_set = operand_u32(code, self.pc, bc, ON_BIT_SET);
        // table 是 SPECIAL kBitTable 操作数 (bytecodes.h:43)，16 字节定宽切片
        let table = operand_bit_table(code, self.pc, bc, TABLE);
        // cc:833: CheckBitInTable(current_char, table) — 核已落本文件 cc:321-326
        if check_bit_in_table(self.current_char, table) {
          return Some(Flow::Jump(on_bit_set));
        }
        Some(Flow::Advance)
      }
      // cc:840-846: current_char < limit 则跳转，否则 ADVANCE
      Bytecode::CheckCharacterLT => {
        use self::bytecode_operands::check_character_lt::{LIMIT, ON_LESS};
        let limit = u32::from(operand_u16(code, self.pc, bc, LIMIT));
        let on_less = operand_u32(code, self.pc, bc, ON_LESS);
        // cc:841: current_char < limit
        if self.current_char < limit {
          return Some(Flow::Jump(on_less));
        }
        Some(Flow::Advance)
      }
      // cc:848-854: current_char > limit 则跳转，否则 ADVANCE
      Bytecode::CheckCharacterGT => {
        use self::bytecode_operands::check_character_gt::{LIMIT, ON_GREATER};
        let limit = u32::from(operand_u16(code, self.pc, bc, LIMIT));
        let on_greater = operand_u32(code, self.pc, bc, ON_GREATER);
        // cc:849: current_char > limit
        if self.current_char > limit {
          return Some(Flow::Jump(on_greater));
        }
        Some(Flow::Advance)
      }
      // cc:991-998: pos=current+cp_offset; pos 越界则 on_failure，否则 ADVANCE
      Bytecode::CheckPosition => {
        use self::bytecode_operands::check_position::{CP_OFFSET, ON_FAILURE};
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let on_failure = operand_u32(code, self.pc, bc, ON_FAILURE);
        let pos = self.current + cp_offset;
        // cc:993: pos >= subject.length() || pos < 0 (原版两条件，index_is_in_bounds 以 uintptr 回绕统一承接)
        if pos >= self.subject.len() || pos < 0 {
          return Some(Flow::Jump(on_failure));
        }
        Some(Flow::Advance)
      }
      // cc:1000-1008: CheckSpecialClassRanges 核，匹配则 ADVANCE，否则 SET_PC_FROM_OFFSET(on_no_match)
      Bytecode::CheckSpecialClassRanges => {
        use self::bytecode_operands::check_special_class_ranges::{CHARACTER_SET, ON_NO_MATCH};
        let cs_byte = operand_u8(code, self.pc, bc, CHARACTER_SET);
        let on_no_match = operand_u32(code, self.pc, bc, ON_NO_MATCH);
        // 原版操作数是 StandardCharacterSet::Type 的 char 编码 (regexp-ast.h:102-112);
        // 非转义字母表字节原版由生成器保证不发, 承接面在此显式IllegalCode (禁静默映射)
        let Some(character_set) = scs_from_u8(cs_byte) else {
          return Some(Flow::Halt(Step::IllegalCode {
            pc: self.pc,
            reason: "StandardCharacterSet 操作数非 regexp-ast.h:102-112 转义字母",
          }));
        };
        // cc:1001-1002: bool match = CheckSpecialClassRanges<Char>(current_char, character_set)
        // 原版 is_one_byte (cc:421) = sizeof(Char)==1 → S::ONE_BYTE
        // word/notword 分支读 RegExpMacroAssembler::word_character_map() (h:403-407)
        // → K_WORD_CHARACTER_MAP (regexp-macro-assembler.cc:720-761，256 项静态表)
        let matched = if S::ONE_BYTE {
          check_special_class_ranges::<true>(
            self.current_char,
            character_set,
            &K_WORD_CHARACTER_MAP,
          )
        } else {
          check_special_class_ranges::<false>(
            self.current_char,
            character_set,
            &K_WORD_CHARACTER_MAP,
          )
        };
        if matched {
          // cc:1004: ADVANCE()
          Some(Flow::Advance)
        } else {
          // cc:1006: SET_PC_FROM_OFFSET(on_no_match)
          Some(Flow::Jump(on_no_match))
        }
      }
      _ => None,
    }
  }

  /// P-3c 反向引用/锚点 9 项 (REG-1b3, cc 源序) —
  /// CheckNotBackRef cc:880 / CheckNotBackRefBackward cc:894 /
  /// CheckNotBackRefNoCaseUnicode cc:908 / CheckNotBackRefNoCase cc:922 /
  /// CheckNotBackRefNoCaseUnicodeBackward cc:937 / CheckNotBackRefNoCaseBackward cc:952 /
  /// CheckAtStart cc:967 / CheckNotAtStart cc:975 / SetCurrentPositionFromEnd cc:983
  /// NoCase 四 handler 的比较核 = BackRefMatchesNoCase (cc:46-81):
  /// 一字节态 Latin1 核 (cc:63-81) 直落; uc16 态依赖 ICU 折叠 (cc:46-61 →
  /// CaseInsensitiveCompare*), 经 subject.backref_matches_no_case 返回 None
  /// 走 P-3c-icu 运行态通道 (Step::NotImplemented, 禁伪装结果)
  fn exec_backref_ops(&mut self, bc: Bytecode, code: &[u8]) -> Option<Flow> {
    match bc {
      // cc:880-892: from/len 取自相邻寄存器; 界检与比较短路同 C++ `||` 序
      Bytecode::CheckNotBackRef => {
        use self::bytecode_operands::check_not_back_ref::{ON_NOT_EQUAL, START_REG};
        // cc:881-882: int from = registers[start_reg]; int len = registers[start_reg + 1] - from
        let start_reg = usize::from(operand_u16(code, self.pc, bc, START_REG));
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        let from = self.registers.get(start_reg);
        let len = self.registers.get(start_reg + 1) - from;
        if from >= 0 && len > 0 {
          // cc:884-885: current + len > subject.length() ||
          //              !CompareCharsEqual(&subject[from], &subject[current], len)
          if self.current + len > self.subject.len()
            || !self.subject.chars_equal(from, self.current, len)
          {
            // cc:886: SET_PC_FROM_OFFSET(on_not_equal)
            return Some(Flow::Jump(on_not_equal));
          }
          // cc:889: ADVANCE_CURRENT_POSITION(len)
          self.advance_current(len);
        }
        // cc:890: ADVANCE()
        Some(Flow::Advance)
      }
      // cc:894-906: 后向变体 — 比较窗口在 current-len, 命中后回拨 current
      Bytecode::CheckNotBackRefBackward => {
        use self::bytecode_operands::check_not_back_ref_backward::{ON_NOT_EQUAL, START_REG};
        let start_reg = usize::from(operand_u16(code, self.pc, bc, START_REG));
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        let from = self.registers.get(start_reg);
        let len = self.registers.get(start_reg + 1) - from;
        if from >= 0 && len > 0 {
          // cc:898-899: current - len < 0 ||
          //              !CompareCharsEqual(&subject[from], &subject[current - len], len)
          if self.current - len < 0 || !self.subject.chars_equal(from, self.current - len, len) {
            // cc:900: SET_PC_FROM_OFFSET(on_not_equal)
            return Some(Flow::Jump(on_not_equal));
          }
          // cc:902: SET_CURRENT_POSITION(current - len)
          self.set_current(self.current - len);
        }
        // cc:904: ADVANCE()
        Some(Flow::Advance)
      }
      // cc:908-920: NoCase + unicode 标志 (前向)
      Bytecode::CheckNotBackRefNoCaseUnicode => {
        use self::bytecode_operands::check_not_back_ref_no_case_unicode::{
          ON_NOT_EQUAL, START_REG,
        };
        let start_reg = usize::from(operand_u16(code, self.pc, bc, START_REG));
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        let from = self.registers.get(start_reg);
        let len = self.registers.get(start_reg + 1) - from;
        if from >= 0 && len > 0 {
          // cc:912: current + len > subject.length() 先行短路, 不打 ICU 比较
          if self.current + len > self.subject.len() {
            return Some(Flow::Jump(on_not_equal));
          }
          // cc:912-913: !BackRefMatchesNoCase(isolate, from, current, len, subject, true)
          let Some(matches) = self
            .subject
            .backref_matches_no_case(from, self.current, len, true)
          else {
            return Some(Flow::Halt(Step::NotImplemented {
              bytecode: bc,
              cc_line: 46,
              batch: "P-3c-icu",
            }));
          };
          if !matches {
            // cc:914: SET_PC_FROM_OFFSET(on_not_equal)
            return Some(Flow::Jump(on_not_equal));
          }
          // cc:916: ADVANCE_CURRENT_POSITION(len)
          self.advance_current(len);
        }
        // cc:918: ADVANCE()
        Some(Flow::Advance)
      }
      // cc:922-935: NoCase 非 unicode (前向); uint8 重载下 unicode 标志无差别
      // (cc:65 注释), 仅 uc16 态影响 CaseInsensitiveCompare 选择 (P-3c-icu)
      Bytecode::CheckNotBackRefNoCase => {
        use self::bytecode_operands::check_not_back_ref_no_case::{ON_NOT_EQUAL, START_REG};
        let start_reg = usize::from(operand_u16(code, self.pc, bc, START_REG));
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        let from = self.registers.get(start_reg);
        let len = self.registers.get(start_reg + 1) - from;
        if from >= 0 && len > 0 {
          if self.current + len > self.subject.len() {
            return Some(Flow::Jump(on_not_equal));
          }
          // cc:926-927: BackRefMatchesNoCase(..., false)
          let Some(matches) = self
            .subject
            .backref_matches_no_case(from, self.current, len, false)
          else {
            return Some(Flow::Halt(Step::NotImplemented {
              bytecode: bc,
              cc_line: 46,
              batch: "P-3c-icu",
            }));
          };
          if !matches {
            // cc:928: SET_PC_FROM_OFFSET(on_not_equal)
            return Some(Flow::Jump(on_not_equal));
          }
          // cc:930: ADVANCE_CURRENT_POSITION(len)
          self.advance_current(len);
        }
        // cc:932: ADVANCE()
        Some(Flow::Advance)
      }
      // cc:937-950: NoCase + unicode (后向) — 比较窗口在 current-len
      Bytecode::CheckNotBackRefNoCaseUnicodeBackward => {
        use self::bytecode_operands::check_not_back_ref_no_case_unicode_backward::{
          ON_NOT_EQUAL, START_REG,
        };
        let start_reg = usize::from(operand_u16(code, self.pc, bc, START_REG));
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        let from = self.registers.get(start_reg);
        let len = self.registers.get(start_reg + 1) - from;
        if from >= 0 && len > 0 {
          // cc:941: current - len < 0 先行短路
          if self.current - len < 0 {
            return Some(Flow::Jump(on_not_equal));
          }
          // cc:941-942: BackRefMatchesNoCase(isolate, from, current - len, len, subject, true)
          let Some(matches) =
            self
              .subject
              .backref_matches_no_case(from, self.current - len, len, true)
          else {
            return Some(Flow::Halt(Step::NotImplemented {
              bytecode: bc,
              cc_line: 46,
              batch: "P-3c-icu",
            }));
          };
          if !matches {
            // cc:943: SET_PC_FROM_OFFSET(on_not_equal)
            return Some(Flow::Jump(on_not_equal));
          }
          // cc:945: SET_CURRENT_POSITION(current - len)
          self.set_current(self.current - len);
        }
        // cc:947: ADVANCE()
        Some(Flow::Advance)
      }
      // cc:952-965: NoCase 非 unicode (后向)
      Bytecode::CheckNotBackRefNoCaseBackward => {
        use self::bytecode_operands::check_not_back_ref_no_case_backward::{
          ON_NOT_EQUAL, START_REG,
        };
        let start_reg = usize::from(operand_u16(code, self.pc, bc, START_REG));
        let on_not_equal = operand_u32(code, self.pc, bc, ON_NOT_EQUAL);
        let from = self.registers.get(start_reg);
        let len = self.registers.get(start_reg + 1) - from;
        if from >= 0 && len > 0 {
          if self.current - len < 0 {
            return Some(Flow::Jump(on_not_equal));
          }
          // cc:956-957: BackRefMatchesNoCase(..., false)
          let Some(matches) =
            self
              .subject
              .backref_matches_no_case(from, self.current - len, len, false)
          else {
            return Some(Flow::Halt(Step::NotImplemented {
              bytecode: bc,
              cc_line: 46,
              batch: "P-3c-icu",
            }));
          };
          if !matches {
            // cc:958: SET_PC_FROM_OFFSET(on_not_equal)
            return Some(Flow::Jump(on_not_equal));
          }
          // cc:960: SET_CURRENT_POSITION(current - len)
          self.set_current(self.current - len);
        }
        // cc:962: ADVANCE()
        Some(Flow::Advance)
      }
      // cc:967-973: current + cp_offset == 0 → on_at_start, 否则 ADVANCE
      Bytecode::CheckAtStart => {
        use self::bytecode_operands::check_at_start::{CP_OFFSET, ON_AT_START};
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let on_at_start = operand_u32(code, self.pc, bc, ON_AT_START);
        if self.current + cp_offset == 0 {
          // cc:968: SET_PC_FROM_OFFSET(on_at_start)
          return Some(Flow::Jump(on_at_start));
        }
        // cc:970: ADVANCE()
        Some(Flow::Advance)
      }
      // cc:975-981: 同式的反向出口
      Bytecode::CheckNotAtStart => {
        use self::bytecode_operands::check_not_at_start::{CP_OFFSET, ON_NOT_AT_START};
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let on_not_at_start = operand_u32(code, self.pc, bc, ON_NOT_AT_START);
        if self.current + cp_offset == 0 {
          // cc:976: ADVANCE()
          return Some(Flow::Advance);
        }
        // cc:978: SET_PC_FROM_OFFSET(on_not_at_start)
        Some(Flow::Jump(on_not_at_start))
      }
      // cc:983-990: 先 ADVANCE, 末尾还剩 >by 个字符则回拨至 len-by 并取前字符
      Bytecode::SetCurrentPositionFromEnd => {
        use self::bytecode_operands::set_current_position_from_end::BY;
        let by = i32::from(operand_i16(code, self.pc, bc, BY));
        // cc:984: ADVANCE() — 在界判之前, 1:1 保持
        let len = self.subject.len();
        if len - self.current > by {
          // cc:986: SET_CURRENT_POSITION(subject.length() - by)
          self.set_current(len - by);
          // cc:988: current_char = subject[current - 1]
          self.current_char = self.subject.char_at(self.current - 1);
        }
        Some(Flow::Advance)
      }
      _ => None,
    }
  }

  /// P-3d peephole skip 系 7 项 (REG-1b3, cc 源序) —
  /// SkipUntilChar cc:1010 / SkipUntilCharAnd cc:1023 / SkipUntilBitInTable cc:1036 /
  /// SkipUntilGtOrNotBitInTable cc:1049 / SkipUntilCharOrChar cc:1066 /
  /// SkipUntilOneOfMasked cc:1086 / SkipUntilOneOfMasked3 cc:1111
  /// 原版 while 循环 + DISPATCH 的循环内跳转, Rust 以 Flow::Jump 提前返回承接
  /// (DISPATCH 语义 = 落位 pc 回主循环, 与中途返回等价); OneOfMasked3 的
  /// bc0..bc7 复合体按 cc:1122-1179 控制流 1:1 (外层 while(true) + break)
  fn exec_skip_ops(&mut self, bc: Bytecode, code: &[u8]) -> Option<Flow> {
    match bc {
      // cc:1010-1021: 逐字符跳到 character 命中, 出口 on_match/on_no_match
      Bytecode::SkipUntilChar => {
        use self::bytecode_operands::skip_until_char::{
          ADVANCE_BY, BOUNDS_CHECK_OFFSET, CHARACTER, CP_OFFSET, ON_MATCH, ON_NO_MATCH,
        };
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let advance_by = i32::from(operand_i16(code, self.pc, bc, ADVANCE_BY));
        let character = u32::from(operand_u16(code, self.pc, bc, CHARACTER));
        let bounds_check_offset = operand_i32(code, self.pc, bc, BOUNDS_CHECK_OFFSET);
        let on_match = operand_u32(code, self.pc, bc, ON_MATCH);
        let on_no_match = operand_u32(code, self.pc, bc, ON_NO_MATCH);
        // cc:1012: while (IndexIsInBounds(current + bounds_check_offset, len))
        while index_is_in_bounds(self.current + bounds_check_offset, self.subject.len()) {
          // cc:1013: current_char = subject[current + cp_offset]
          self.current_char = self.subject.char_at(self.current + cp_offset);
          if character == self.current_char {
            // cc:1014-1015: SET_PC_FROM_OFFSET(on_match); DISPATCH()
            return Some(Flow::Jump(on_match));
          }
          // cc:1017: ADVANCE_CURRENT_POSITION(advance_by)
          self.advance_current(advance_by);
        }
        // cc:1019: SET_PC_FROM_OFFSET(on_no_match)
        Some(Flow::Jump(on_no_match))
      }
      // cc:1023-1034: 命中条件为 (current_char & mask) == character
      Bytecode::SkipUntilCharAnd => {
        use self::bytecode_operands::skip_until_char_and::{
          ADVANCE_BY, BOUNDS_CHECK_OFFSET, CHARACTER, CP_OFFSET, MASK, ON_MATCH, ON_NO_MATCH,
        };
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let advance_by = i32::from(operand_i16(code, self.pc, bc, ADVANCE_BY));
        let character = u32::from(operand_u16(code, self.pc, bc, CHARACTER));
        let mask = operand_u32(code, self.pc, bc, MASK);
        let bounds_check_offset = operand_i32(code, self.pc, bc, BOUNDS_CHECK_OFFSET);
        let on_match = operand_u32(code, self.pc, bc, ON_MATCH);
        let on_no_match = operand_u32(code, self.pc, bc, ON_NO_MATCH);
        while index_is_in_bounds(self.current + bounds_check_offset, self.subject.len()) {
          self.current_char = self.subject.char_at(self.current + cp_offset);
          // cc:1026: character == (current_char & mask) — kUint32 mask (h:262 注 TODO)
          if character == (self.current_char & mask) {
            // cc:1027-1028: SET_PC_FROM_OFFSET(on_match); DISPATCH()
            return Some(Flow::Jump(on_match));
          }
          // cc:1030: ADVANCE_CURRENT_POSITION(advance_by)
          self.advance_current(advance_by);
        }
        // cc:1032: SET_PC_FROM_OFFSET(on_no_match)
        Some(Flow::Jump(on_no_match))
      }
      // cc:1036-1047: 命中条件为表查 (CheckBitInTable cc:321-326 纯函数)
      Bytecode::SkipUntilBitInTable => {
        use self::bytecode_operands::skip_until_bit_in_table::{
          ADVANCE_BY, BOUNDS_CHECK_OFFSET, CP_OFFSET, ON_MATCH, ON_NO_MATCH, TABLE,
        };
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let advance_by = i32::from(operand_i16(code, self.pc, bc, ADVANCE_BY));
        let table = operand_bit_table(code, self.pc, bc, TABLE);
        let bounds_check_offset = operand_i32(code, self.pc, bc, BOUNDS_CHECK_OFFSET);
        let on_match = operand_u32(code, self.pc, bc, ON_MATCH);
        let on_no_match = operand_u32(code, self.pc, bc, ON_NO_MATCH);
        while index_is_in_bounds(self.current + bounds_check_offset, self.subject.len()) {
          self.current_char = self.subject.char_at(self.current + cp_offset);
          // cc:1039: CheckBitInTable(current_char, table)
          if check_bit_in_table(self.current_char, table) {
            // cc:1040-1041: SET_PC_FROM_OFFSET(on_match); DISPATCH()
            return Some(Flow::Jump(on_match));
          }
          // cc:1043: ADVANCE_CURRENT_POSITION(advance_by)
          self.advance_current(advance_by);
        }
        // cc:1045: SET_PC_FROM_OFFSET(on_no_match)
        Some(Flow::Jump(on_no_match))
      }
      // cc:1049-1064: 命中条件为 current_char > character 或 !表查
      Bytecode::SkipUntilGtOrNotBitInTable => {
        use self::bytecode_operands::skip_until_gt_or_not_bit_in_table::{
          ADVANCE_BY, BOUNDS_CHECK_OFFSET, CHARACTER, CP_OFFSET, ON_MATCH, ON_NO_MATCH, TABLE,
        };
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let advance_by = i32::from(operand_i16(code, self.pc, bc, ADVANCE_BY));
        let character = u32::from(operand_u16(code, self.pc, bc, CHARACTER));
        let table = operand_bit_table(code, self.pc, bc, TABLE);
        let bounds_check_offset = operand_i32(code, self.pc, bc, BOUNDS_CHECK_OFFSET);
        let on_match = operand_u32(code, self.pc, bc, ON_MATCH);
        let on_no_match = operand_u32(code, self.pc, bc, ON_NO_MATCH);
        while index_is_in_bounds(self.current + bounds_check_offset, self.subject.len()) {
          self.current_char = self.subject.char_at(self.current + cp_offset);
          // cc:1052-1053: 两 if 分立不合并 (原版注释: 合并影响寄存器分配), 1:1 保持
          if self.current_char > character {
            // cc:1054: SET_PC_FROM_OFFSET(on_match); DISPATCH()
            return Some(Flow::Jump(on_match));
          }
          // cc:1057: !CheckBitInTable(current_char, table)
          if !check_bit_in_table(self.current_char, table) {
            // cc:1058: SET_PC_FROM_OFFSET(on_match); DISPATCH()
            return Some(Flow::Jump(on_match));
          }
          // cc:1061: ADVANCE_CURRENT_POSITION(advance_by)
          self.advance_current(advance_by);
        }
        // cc:1063: SET_PC_FROM_OFFSET(on_no_match)
        Some(Flow::Jump(on_no_match))
      }
      // cc:1066-1084: char1/char2 双 if 分立 (原版注释明示勿合并)
      Bytecode::SkipUntilCharOrChar => {
        use self::bytecode_operands::skip_until_char_or_char::{
          ADVANCE_BY, BOUNDS_CHECK_OFFSET, CHAR1, CHAR2, CP_OFFSET, ON_MATCH, ON_NO_MATCH,
        };
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let advance_by = i32::from(operand_i16(code, self.pc, bc, ADVANCE_BY));
        let char1 = u32::from(operand_u16(code, self.pc, bc, CHAR1));
        let char2 = u32::from(operand_u16(code, self.pc, bc, CHAR2));
        let bounds_check_offset = operand_i32(code, self.pc, bc, BOUNDS_CHECK_OFFSET);
        let on_match = operand_u32(code, self.pc, bc, ON_MATCH);
        let on_no_match = operand_u32(code, self.pc, bc, ON_NO_MATCH);
        while index_is_in_bounds(self.current + bounds_check_offset, self.subject.len()) {
          self.current_char = self.subject.char_at(self.current + cp_offset);
          // cc:1072-1073
          if char1 == self.current_char {
            // cc:1074: SET_PC_FROM_OFFSET(on_match); DISPATCH()
            return Some(Flow::Jump(on_match));
          }
          // cc:1077-1078
          if char2 == self.current_char {
            // cc:1078-1079: SET_PC_FROM_OFFSET(on_match); DISPATCH()
            return Some(Flow::Jump(on_match));
          }
          // cc:1081: ADVANCE_CURRENT_POSITION(advance_by)
          self.advance_current(advance_by);
        }
        // cc:1083: SET_PC_FROM_OFFSET(on_no_match)
        Some(Flow::Jump(on_no_match))
      }
      // cc:1086-1108: 4 字符窗口双掩码匹配 (both → chars1/chars2 两级)
      Bytecode::SkipUntilOneOfMasked => {
        use self::bytecode_operands::skip_until_one_of_masked::{
          ADVANCE_BY, BOTH_CHARS, BOTH_MASK, CHARS1, CHARS2, CP_OFFSET, MASK1, MASK2, MAX_OFFSET,
          ON_FAILURE, ON_MATCH1, ON_MATCH2,
        };
        let cp_offset = i32::from(operand_i16(code, self.pc, bc, CP_OFFSET));
        let advance_by = i32::from(operand_i16(code, self.pc, bc, ADVANCE_BY));
        let both_chars = operand_u32(code, self.pc, bc, BOTH_CHARS);
        let both_mask = operand_u32(code, self.pc, bc, BOTH_MASK);
        let max_offset = operand_i32(code, self.pc, bc, MAX_OFFSET);
        let chars1 = operand_u32(code, self.pc, bc, CHARS1);
        let mask1 = operand_u32(code, self.pc, bc, MASK1);
        let chars2 = operand_u32(code, self.pc, bc, CHARS2);
        let mask2 = operand_u32(code, self.pc, bc, MASK2);
        let on_match1 = operand_u32(code, self.pc, bc, ON_MATCH1);
        let on_match2 = operand_u32(code, self.pc, bc, ON_MATCH2);
        let on_failure = operand_u32(code, self.pc, bc, ON_FAILURE);
        // cc:1091-1092: DCHECK_GE(cp_offset, 0); DCHECK_GE(max_offset, cp_offset + 3)
        debug_assert!(cp_offset >= 0, "DCHECK_GE(cp_offset, 0) (cc:1091)");
        debug_assert!(
          max_offset >= cp_offset + 3,
          "DCHECK_GE(max_offset, cp_offset + 3) (cc:1090)"
        );
        // cc:1092: DCHECK_EQ(1, sizeof(Char)) — 生成器仅单字节态发该指令
        debug_assert!(
          S::ONE_BYTE,
          "SkipUntilOneOfMasked requires one_byte subject"
        );
        // cc:1095: while (IndexIsInBounds(current + max_offset, len)) — 4 字符窗口界
        while index_is_in_bounds(self.current + max_offset, self.subject.len()) {
          // cc:1096-1097: pos = current + cp_offset; current_char = Load4Characters(subject, pos)
          self.current_char = self.subject.load4_at(self.current + cp_offset);
          // cc:1098: both_chars == (current_char & both_mask) — 预筛后才分查两路
          if both_chars == (self.current_char & both_mask) {
            if chars1 == (self.current_char & mask1) {
              // cc:1098: SET_PC_FROM_OFFSET(on_match1); DISPATCH()
              return Some(Flow::Jump(on_match1));
            }
            if chars2 == (self.current_char & mask2) {
              // cc:1103: SET_PC_FROM_OFFSET(on_match2); DISPATCH()
              return Some(Flow::Jump(on_match2));
            }
          }
          // cc:1106: ADVANCE_CURRENT_POSITION(advance_by)
          self.advance_current(advance_by);
        }
        // cc:1108: SET_PC_FROM_OFFSET(on_failure)
        Some(Flow::Jump(on_failure))
      }
      // cc:1111-1179: bc0..bc7 复合体 (SkipUntilBitInTable+Load4+AndCheck4Chars 等
      // 融合, 正则 /<script|<style|<link/i 类); 外层 while(true), bc0 扫描命中后
      // break, 失败路径 advance bc3_by 后 continue (cc:1144-1148/1176-1177)
      Bytecode::SkipUntilOneOfMasked3 => {
        use self::bytecode_operands::skip_until_one_of_masked3::{
          BC0_ADVANCE_BY, BC0_CP_OFFSET, BC0_TABLE, BC1_BOUNDS_CHECK_OFFSET, BC1_CP_OFFSET,
          BC1_ON_FAILURE, BC2_CHARACTERS, BC2_MASK, BC3_BY, BC4_BOUNDS_CHECK_OFFSET, BC4_CP_OFFSET,
          BC5_CHARACTERS, BC5_MASK, BC5_ON_EQUAL, BC6_CHARACTERS, BC6_MASK, BC6_ON_EQUAL,
          BC7_CHARACTERS, BC7_MASK, FALLTHROUGH_JUMP_TARGET,
        };
        // cc:1114: DCHECK_EQ(1, sizeof(Char))
        debug_assert!(
          S::ONE_BYTE,
          "SkipUntilOneOfMasked3 requires one_byte subject"
        );
        let bc0_cp_offset = i32::from(operand_i16(code, self.pc, bc, BC0_CP_OFFSET));
        let bc0_advance_by = i32::from(operand_i16(code, self.pc, bc, BC0_ADVANCE_BY));
        let bc0_table = operand_bit_table(code, self.pc, bc, BC0_TABLE);
        let bc1_bounds_check_offset = operand_i32(code, self.pc, bc, BC1_BOUNDS_CHECK_OFFSET);
        let bc1_on_failure = operand_u32(code, self.pc, bc, BC1_ON_FAILURE);
        let bc1_cp_offset = i32::from(operand_i16(code, self.pc, bc, BC1_CP_OFFSET));
        let bc2_characters = operand_u32(code, self.pc, bc, BC2_CHARACTERS);
        let bc2_mask = operand_u32(code, self.pc, bc, BC2_MASK);
        let bc3_by = i32::from(operand_i16(code, self.pc, bc, BC3_BY));
        let bc4_bounds_check_offset = operand_i32(code, self.pc, bc, BC4_BOUNDS_CHECK_OFFSET);
        let bc4_cp_offset = i32::from(operand_i16(code, self.pc, bc, BC4_CP_OFFSET));
        let bc5_characters = operand_u32(code, self.pc, bc, BC5_CHARACTERS);
        let bc5_mask = operand_u32(code, self.pc, bc, BC5_MASK);
        let bc5_on_equal = operand_u32(code, self.pc, bc, BC5_ON_EQUAL);
        let bc6_characters = operand_u32(code, self.pc, bc, BC6_CHARACTERS);
        let bc6_mask = operand_u32(code, self.pc, bc, BC6_MASK);
        let bc6_on_equal = operand_u32(code, self.pc, bc, BC6_ON_EQUAL);
        let bc7_characters = operand_u32(code, self.pc, bc, BC7_CHARACTERS);
        let bc7_mask = operand_u32(code, self.pc, bc, BC7_MASK);
        let fallthrough_jump_target = operand_u32(code, self.pc, bc, FALLTHROUGH_JUMP_TARGET);
        // cc:1117: while (true)
        loop {
          // cc:1121-1128: bc0 = kSkipUntilBitInTable (on_match/on_no_match 均聚到 bc1)
          // 界式用 current + bc0_cp_offset (LOAD 位即界位, 1:1 保持)
          loop {
            if !index_is_in_bounds(self.current + bc0_cp_offset, self.subject.len()) {
              break;
            }
            self.current_char = self.subject.char_at(self.current + bc0_cp_offset);
            if check_bit_in_table(self.current_char, bc0_table) {
              // cc:1124: break — 命中, 出 bc0 扫描入 bc1
              break;
            }
            // cc:1127: ADVANCE_CURRENT_POSITION(bc0_advance_by)
            self.advance_current(bc0_advance_by);
          }
          // cc:1131-1134: bc1 = kLoad4CurrentChars 界检
          if !index_is_in_bounds(self.current + bc1_bounds_check_offset, self.subject.len()) {
            // cc:1132: SET_PC_FROM_OFFSET(bc1_on_failure); DISPATCH()
            return Some(Flow::Jump(bc1_on_failure));
          }
          // cc:1137-1138: pos = current + bc1_cp_offset; current_char = Load4Characters(subject, pos)
          self.current_char = self.subject.load4_at(self.current + bc1_cp_offset);
          // cc:1141: bc2 = AndCheck4Chars — on_equal 限定跳 bc4
          if bc2_characters == (self.current_char & bc2_mask) {
            // cc:1144-1146: bc4 = Load4CurrentChars 界检 (on_failure 限定回 bc3)
            if !index_is_in_bounds(self.current + bc4_bounds_check_offset, self.subject.len()) {
              // cc:1147: bc3 = AdvanceCpAndGoto — ADVANCE_CURRENT_POSITION(bc3_by) 后 continue
              self.advance_current(bc3_by);
              continue;
            }
            // cc:1154-1155: pos = current + bc4_cp_offset; 再 Load4
            self.current_char = self.subject.load4_at(self.current + bc4_cp_offset);
            // cc:1158: bc5 = AndCheck4Chars
            if bc5_characters == (self.current_char & bc5_mask) {
              // cc:1159: SET_PC_FROM_OFFSET(bc5_on_equal); DISPATCH()
              return Some(Flow::Jump(bc5_on_equal));
            }
            // cc:1162: bc6 = AndCheck4Chars
            if bc6_characters == (self.current_char & bc6_mask) {
              // cc:1163: SET_PC_FROM_OFFSET(bc6_on_equal); DISPATCH()
              return Some(Flow::Jump(bc6_on_equal));
            }
            // cc:1166: bc7 = AndCheckNot4Chars (on_not_equal 限定回 bc3)
            if bc7_characters == (self.current_char & bc7_mask) {
              // cc:1167-1168: SET_PC_FROM_OFFSET(fallthrough_jump_target); DISPATCH()
              return Some(Flow::Jump(fallthrough_jump_target));
            }
          }
          // cc:1175: bc3 = AdvanceCpAndGoto (fallthrough)
          self.advance_current(bc3_by);
          // cc:1176: 隐式 continue 回外层 while(true)
        }
        // cc:1181: UNREACHABLE() — loop 无正常出口 (所有回边为 continue/return),
        // 到达即控制流级错误; Rust break 不存在该落点, 结构性免除
      }
      _ => None,
    }
  }

  /// 原版 InterpreterRegisters::CopyToOutputRegisters (cc:202-205) 的调用点
  /// (Succeed cc:670) — 定长 output_register_count 槽拷贝
  fn copy_registers_to_output(&mut self) {
    let count = self.registers.output_register_count();
    self
      .registers
      .copy_to_output_registers(&mut self.output[..count]);
  }
}
