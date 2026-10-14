// 由 tool/src/diagnostics/arm64/disasm-arm64.h.js 从 v8/src/diagnostics/arm64/disasm-arm64.{h,cc} 生成; 修订就地改, 复跑覆盖
// arm64 反汇编规则面: NEON 格式表/助记索引表/barrier 选项/骨架签名

// 原文件 #if V8_TARGET_ARCH_ARM64 门控 (arm64 专用), 本产物即 arm64 目标展开。
// 语义面登记跳过: DisassemblingDecoder::Visit*/Substitute* 解码替换实现
// (~4600 行) 不属机械规则代码, 由 Developer 修订补齐。
// NEONFormat (NF_*) 枚举本体在 instructions-arm64.h, 由
// tool/src/codegen/arm64/instructions-arm64.h.js 展开, 此处表值以名字登记。

/// NEON 格式映射 (原 struct NEONFormatMap): 按 bits 位取索引查 formats 表
#[derive(Debug, Clone, Copy)]
pub struct NeonFormatMap {
  /// 参与索引的指令位号 (原 forms_mask 初始化列表)
  pub bits: &'static [usize],
  /// NF_* 格式名 (原 NEONFormat 枚举值, 名字与
  /// crate 内 instructions_arm64_generated::NeonFormat 对应)
  pub formats: &'static [&'static str],
}

/// 原文件内 12 张 static const NEONFormatMap 实例 (含归属函数)
/// 原位 VisitNEON2RegMisc 内 static map_lp_ta
pub static MAP_LP_TA: NeonFormatMap = NeonFormatMap { bits: &[23, 22, 30], formats: &["NF_4H", "NF_8H", "NF_2S", "NF_4S", "NF_1D", "NF_2D"] };

/// 原位 VisitNEON2RegMisc 内 static map_cvt_ta
pub static MAP_CVT_TA: NeonFormatMap = NeonFormatMap { bits: &[22], formats: &["NF_4S", "NF_2D"] };

/// 原位 VisitNEON2RegMisc 内 static map_cvt_tb
pub static MAP_CVT_TB: NeonFormatMap = NeonFormatMap { bits: &[22, 30], formats: &["NF_4H", "NF_8H", "NF_2S", "NF_4S"] };

/// 原位 VisitNEONByIndexedElement 内 static map_ta
pub static MAP_TA: NeonFormatMap = NeonFormatMap { bits: &[23, 22], formats: &["NF_UNDEF", "NF_4S", "NF_2D"] };

/// 原位 VisitNEONModifiedImmediate 内 static map_b
pub static MAP_B: NeonFormatMap = NeonFormatMap { bits: &[30], formats: &["NF_8B", "NF_16B"] };

/// 原位 VisitNEONModifiedImmediate 内 static map_h
pub static MAP_H: NeonFormatMap = NeonFormatMap { bits: &[30], formats: &["NF_4H", "NF_8H"] };

/// 原位 VisitNEONModifiedImmediate 内 static map_s
pub static MAP_S: NeonFormatMap = NeonFormatMap { bits: &[30], formats: &["NF_2S", "NF_4S"] };

/// 原位 VisitNEONScalarShiftImmediate 内 static map_shift
pub static MAP_SHIFT: NeonFormatMap = NeonFormatMap { bits: &[22, 21, 20, 19], formats: &["NF_UNDEF", "NF_B", "NF_H", "NF_H", "NF_S", "NF_S", "NF_S", "NF_S", "NF_D", "NF_D", "NF_D", "NF_D", "NF_D", "NF_D", "NF_D", "NF_D"] };

/// 原位 VisitNEONScalarShiftImmediate 内 static map_shift_narrow
pub static MAP_SHIFT_NARROW: NeonFormatMap = NeonFormatMap { bits: &[21, 20, 19], formats: &["NF_UNDEF", "NF_H", "NF_S", "NF_S", "NF_D", "NF_D", "NF_D", "NF_D"] };

/// 原位 VisitNEONShiftImmediate 内 static map_shift_ta
pub static MAP_SHIFT_TA: NeonFormatMap = NeonFormatMap { bits: &[22, 21, 20, 19], formats: &["NF_UNDEF", "NF_8H", "NF_4S", "NF_4S", "NF_2D", "NF_2D", "NF_2D", "NF_2D"] };

/// 原位 VisitNEONShiftImmediate 内 static map_shift_tb
pub static MAP_SHIFT_TB: NeonFormatMap = NeonFormatMap { bits: &[22, 21, 20, 19, 30], formats: &["NF_UNDEF", "NF_UNDEF", "NF_8B", "NF_16B", "NF_4H", "NF_8H", "NF_4H", "NF_8H", "NF_2S", "NF_4S", "NF_2S", "NF_4S", "NF_2S", "NF_4S", "NF_2S", "NF_4S", "NF_UNDEF", "NF_2D", "NF_UNDEF", "NF_2D", "NF_UNDEF", "NF_2D", "NF_UNDEF", "NF_2D", "NF_UNDEF", "NF_2D", "NF_UNDEF", "NF_2D", "NF_UNDEF", "NF_2D", "NF_UNDEF", "NF_2D"] };

/// 原位 VisitNEONTable 内 static map_b
pub static MAP_B: NeonFormatMap = NeonFormatMap { bits: &[30], formats: &["NF_8B", "NF_16B"] };

/// 屏障选项表 (原 SubstituteBarrierField 内 options[4][4], 索引 [domain][type])
#[rustfmt::skip]
pub static BARRIER_OPTIONS: [[&str; 4]; 4] = [
  ["sy (0b0000)", "oshld", "oshst", "osh"],
  ["sy (0b0100)", "nshld", "nshst", "nsh"],
  ["sy (0b1000)", "ishld", "ishst", "ish"],
  ["sy (0b1100)", "ld", "st", "sy"],
];

/// Visit 函数内助记索引表 (原 static const char* mnemonics[32], nullptr 以 None 登记)
pub struct MnemonicTable {
  /// 归属的 DisassemblingDecoder::Visit 函数
  pub owner: &'static str,
  /// 取索引公式 (原文转录, instr-> 改 instr.)
  pub index_expr: &'static str,
  /// 助记符, None 即原 nullptr (外层 switch 特判)
  pub mnemonics: &'static [Option<&'static str>],
}

/// 原 VisitNEON3Same 内助记表, 索引: `unsigned index = (instr.Bits(15, 11) << 2) | (instr.Bit(23) << 1) | instr.Bit(29)`
pub static VISITNEON3SAME_MNEMONICS: MnemonicTable = MnemonicTable {
  owner: "VisitNEON3Same",
  index_expr: "unsigned index = (instr.Bits(15, 11) << 2) | (instr.Bit(23) << 1) | instr.Bit(29)",
  mnemonics: [
    Some("shadd"),
    Some("uhadd"),
    Some("shadd"),
    Some("uhadd"),
    Some("sqadd"),
    Some("uqadd"),
    Some("sqadd"),
    Some("uqadd"),
    Some("srhadd"),
    Some("urhadd"),
    Some("srhadd"),
    Some("urhadd"),
    None,
    None,
    None,
    None,
    Some("shsub"),
    Some("uhsub"),
    Some("shsub"),
    Some("uhsub"),
    Some("sqsub"),
    Some("uqsub"),
    Some("sqsub"),
    Some("uqsub"),
    Some("cmgt"),
    Some("cmhi"),
    Some("cmgt"),
    Some("cmhi"),
    Some("cmge"),
    Some("cmhs"),
    Some("cmge"),
    Some("cmhs"),
    Some("sshl"),
    Some("ushl"),
    Some("sshl"),
    Some("ushl"),
    Some("sqshl"),
    Some("uqshl"),
    Some("sqshl"),
    Some("uqshl"),
    Some("srshl"),
    Some("urshl"),
    Some("srshl"),
    Some("urshl"),
    Some("sqrshl"),
    Some("uqrshl"),
    Some("sqrshl"),
    Some("uqrshl"),
    Some("smax"),
    Some("umax"),
    Some("smax"),
    Some("umax"),
    Some("smin"),
    Some("umin"),
    Some("smin"),
    Some("umin"),
    Some("sabd"),
    Some("uabd"),
    Some("sabd"),
    Some("uabd"),
    Some("saba"),
    Some("uaba"),
    Some("saba"),
    Some("uaba"),
    Some("add"),
    Some("sub"),
    Some("add"),
    Some("sub"),
    Some("cmtst"),
    Some("cmeq"),
    Some("cmtst"),
    Some("cmeq"),
    Some("mla"),
    Some("mls"),
    Some("mla"),
    Some("mls"),
    Some("mul"),
    Some("pmul"),
    Some("mul"),
    Some("pmul"),
    Some("smaxp"),
    Some("umaxp"),
    Some("smaxp"),
    Some("umaxp"),
    Some("sminp"),
    Some("uminp"),
    Some("sminp"),
    Some("uminp"),
    Some("sqdmulh"),
    Some("sqrdmulh"),
    Some("sqdmulh"),
    Some("sqrdmulh"),
    Some("addp"),
    Some("unallocated"),
    Some("addp"),
    Some("unallocated"),
    Some("fmaxnm"),
    Some("fmaxnmp"),
    Some("fminnm"),
    Some("fminnmp"),
    Some("fmla"),
    Some("unallocated"),
    Some("fmls"),
    Some("unallocated"),
    Some("fadd"),
    Some("faddp"),
    Some("fsub"),
    Some("fabd"),
    Some("fmulx"),
    Some("fmul"),
    Some("unallocated"),
    Some("unallocated"),
    Some("fcmeq"),
    Some("fcmge"),
    Some("unallocated"),
    Some("fcmgt"),
    Some("unallocated"),
    Some("facge"),
    Some("unallocated"),
    Some("facgt"),
    Some("fmax"),
    Some("fmaxp"),
    Some("fmin"),
    Some("fminp"),
    Some("frecps"),
    Some("fdiv"),
    Some("frsqrts"),
    Some("unallocated"),
  ],
};

/// 原 VisitNEON3SameHP 内助记表, 索引: `unsigned index = // size (23) and the U bit (29). // Operation is determined by the opcode bits (15-11), the top bit of  "frecps",      "fdiv",        "frsqrts",     "unallocated"}`
pub static VISITNEON3SAMEHP_MNEMONICS: MnemonicTable = MnemonicTable {
  owner: "VisitNEON3SameHP",
  index_expr: "unsigned index = // size (23) and the U bit (29). // Operation is determined by the opcode bits (15-11), the top bit of  \"frecps\",      \"fdiv\",        \"frsqrts\",     \"unallocated\"}",
  mnemonics: [
    Some("fmaxnm"),
    Some("fmaxnmp"),
    Some("fminnm"),
    Some("fminnmp"),
    Some("fmla"),
    Some("uqadd"),
    Some("fmls"),
    Some("uqadd"),
    Some("fadd"),
    Some("faddp"),
    Some("fsub"),
    Some("fabd"),
    Some("fmulx"),
    Some("fmul"),
    Some("fmul"),
    Some("fmul"),
    Some("fcmeq"),
    Some("fcmge"),
    Some("shsub"),
    Some("fcmgt"),
    Some("sqsub"),
    Some("facge"),
    Some("sqsub"),
    Some("facgt"),
    Some("fmax"),
    Some("fmaxp"),
    Some("fmin"),
    Some("fminp"),
    Some("frecps"),
    Some("fdiv"),
    Some("frsqrts"),
    Some("fdiv"),
  ],
};

// —— disasm-arm64.h 类面骨架 (DisassemblingDecoder/PrintDisassembler 签名) ——
// 原版经 DECLARE(A) × VISITOR_LIST 展开为全部 Visit<A>(); 此处登记声明锚。
// TODO(dev): 按 DecoderVisitor 框架补 trait 与实现。
// 原签名: void ProcessOutput(Instruction* instr)
// 原签名: void AppendRegisterNameToOutput(const CPURegister& reg)
// 原签名: void Format(Instruction* instr, const char* mnemonic, const char* format)
// 原签名: void Substitute(Instruction* instr, const char* string)
// 原签名: int SubstituteField(Instruction* instr, const char* format)
// 原签名: int SubstituteRegisterField(Instruction* instr, const char* format)
// 原签名: int SubstituteImmediateField(Instruction* instr, const char* format)
// 原签名: int SubstituteLiteralField(Instruction* instr, const char* format)
// 原签名: int SubstituteBitfieldImmediateField(Instruction* instr, const char* format)
// 原签名: int SubstituteShiftField(Instruction* instr, const char* format)
// 原签名: int SubstituteExtendField(Instruction* instr, const char* format)
// 原签名: int SubstituteConditionField(Instruction* instr, const char* format)
// 原签名: int SubstitutePCRelAddressField(Instruction* instr, const char* format)
// 原签名: int SubstituteBranchTargetField(Instruction* instr, const char* format)
// 原签名: int SubstituteLSRegOffsetField(Instruction* instr, const char* format)
// 原签名: int SubstitutePrefetchField(Instruction* instr, const char* format)
// 原签名: int SubstituteBarrierField(Instruction* instr, const char* format)
// 原签名: int SubstituteIntField(const Instruction* instr, const char* format)
// 原签名: void ResetOutput()
// 原签名: void AppendToOutput(const char* string, ...)
// 原签名: void DisassembleNEONPolynomialMul(Instruction* instr)
// 原签名: void ProcessOutput(Instruction* instr)

// —— 提取不变式断言 ——
const _: () = {
  assert!(BARRIER_OPTIONS.len() == 4);
  assert!(BARRIER_OPTIONS[0][0] == "sy (0b0000)");
  assert!(BARRIER_OPTIONS[3][3] == "sy");
  assert!(!MAP_LP_TA.formats.is_empty());
  assert!(!MAP_CVT_TA.formats.is_empty());
  assert!(!MAP_CVT_TB.formats.is_empty());
  assert!(VISITNEON3SAME_MNEMONICS.mnemonics.iter().filter(|m| m.is_some()).count() >= 24);
  assert!(VISITNEON3SAMEHP_MNEMONICS.mnemonics.iter().filter(|m| m.is_some()).count() >= 24);
};
