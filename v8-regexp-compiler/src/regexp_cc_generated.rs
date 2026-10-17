// 由 tool/src/regexp/regexp.cc.js 从 v8/src/regexp/regexp.cc 生成; 修订就地改, 复跑覆盖
// RegExp 编排层基线: 编译管线常量 + GlobalMode 规则 + 双编译轨; Isolate 硬耦合面待 Developer 就地修订

/// RegExpImpl 静态方法清单 (cc:44-127; 声明序, AtomExecRaw 重载计 2)
pub const REGEXP_IMPL_METHODS: [&str; 13] = [
  "ToString",
  "IrregexpInitialize",
  "IrregexpPrepare",
  "AtomCompile",
  "AtomExecRaw",
  "AtomExecRaw",
  "AtomExec",
  "IrregexpExecRaw",
  "IrregexpExec",
  "CompileIrregexpFromSource",
  "CompileIrregexpFromBytecode",
  "EnsureCompiledIrregexp",
  "Compile",
];

// ── 编译管线常量 ──
/// 中段采样字符数 (cc:1409, 频率统计窗口: 中点起 128 字符)
pub const K_SAMPLE_SIZE: i32 = 128;
/// 末锚定回搜上限 (cc:1475: max_length 超此不走 SetCurrentPositionFromEnd)
pub const K_MAX_BACKSEARCH_LIMIT: i32 = 1024;
/// 引擎日志内联主题截断上限 (cc:2026)
pub const K_MAX_INLINE_SUBJECT_LENGTH: usize = 512;

// ── GlobalMode 选择规则 (cc:1481-1489) ──
/// 规则: IsGlobal 且 min_match > 0 → GLOBAL_NO_ZERO_LENGTH_CHECK;
///       IsGlobal 且 EitherUnicode → GLOBAL_UNICODE; IsGlobal 其余 → GLOBAL
pub const K_GLOBAL_MODE_RULES: usize = 3;

// ── 编译目标双轨 (cc:1441-1463) ──
/// kNative: CreateNativeMacroAssembler (输出寄存器数 = RegistersForCaptureCount)
/// kBytecode: BytecodeGenerator (LATIN1/UC16 按 is_one_byte)
pub const K_COMPILATION_TARGETS: usize = 2;

// ── 回搜四前提 (cc:1476) ──
/// is_end_anchored && !is_start_anchored && !IsSticky && max_length < K_MAX_BACKSEARCH_LIMIT
pub const K_BACKSEARCH_PREMISES: usize = 4;

/// 引擎性能日志双格式 (cc:2024/2032):
/// 短: {ns},{engine},/{pattern}/{flags},{last_index},{backref_id},{result}
/// 长: {ns},{engine},/{pattern}/{flags},{last_index},"{subject 截 512}",{result}
pub const K_TRACE_LOG_SHORT: &str = "%s,%s,/%s/%s,%d,%d,%d\n";
pub const K_TRACE_LOG_LONG: &str = "%s,%s,/%s/%s,%d,\"%s\",%d\n";
pub const K_TRACE_LOG_ELLIPSIS: &str = "...";

/// CanGenerateBytecode 双旗标 (cc:132-134): regexp_interpret_all || regexp_tier_up
pub const K_BYTECODE_FLAGS: usize = 2;
/// VerifyFlags 互斥 (cc:137-140): Unicode ∧ UnicodeSets → false
pub const K_FLAG_MUTEX_PAIRS: usize = 1;

const _: () = assert!(REGEXP_IMPL_METHODS.len() == 13);
const _: () = assert!(K_SAMPLE_SIZE == 128);
const _: () = assert!(K_MAX_BACKSEARCH_LIMIT == 1024);
const _: () = assert!(K_MAX_INLINE_SUBJECT_LENGTH == 512);
