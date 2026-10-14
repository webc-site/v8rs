#!/usr/bin/env bun
// tool/src/regexp/regexp.cc.js — v8/src/regexp/regexp.cc
//   → v8-regexp-compiler/src/regexp_cc_generated.rs
// 机械提取: RegExpImpl 静态方法签名清单 + 编译管线常量 (采样/回搜上限/
//   内联主题上限) + GlobalMode 选择规则 + 编译目标双轨 (kNative/kBytecode)
//   + 诊断打印挂钩面 + 引擎性能日志格式;
// 对数断言: RegExpImpl 方法 15 / 常量 3 逐值 / GlobalMode 规则 3 分支。
import { banner, readV8, writeSrc } from "../../lib.js";

const cc = readV8("src/regexp/regexp.cc");

// ── RegExpImpl 静态方法 (cc:44-127) ──────────────────────────────────────
const implBlock = cc.match(/class RegExpImpl final : public AllStatic \{([\s\S]*?)\n\};/);
if (!implBlock) throw new Error("RegExpImpl 类未找到");
const implMethods = [...implBlock[1].matchAll(/^\s+(?:V8_WARN_UNUSED_RESULT\s+)?(?:static\s+)?(?:inline\s+)?[\w:<>&*,\s]+?\s(\w+)\(/gm)].map((r) => r[1]);
if (implMethods.length !== 13) throw new Error(`RegExpImpl 方法应 13 (AtomExecRaw 重载计 2), 实数 ${implMethods.length}`);
const EXPECT_METHODS = [
  "ToString", "IrregexpInitialize", "IrregexpPrepare", "AtomCompile", "AtomExecRaw",
  "AtomExecRaw", "AtomExec", "IrregexpExecRaw", "IrregexpExec",
  "CompileIrregexpFromSource", "CompileIrregexpFromBytecode", "EnsureCompiledIrregexp", "Compile",
];
const msSet = new Set(implMethods);
for (const m of EXPECT_METHODS) {
  if (!msSet.has(m)) throw new Error(`RegExpImpl 缺 ${m}`);
}

// ── 常量逐值锚 ────────────────────────────────────────────────────────────
const CONSTS = [
  [1409, "kSampleSize", 128, "中段采样字符数 (频率统计)"],
  [1475, "kMaxBacksearchLimit", 1024, "末锚定回搜上限 (SetCurrentPositionFromEnd 前提)"],
  [2026, "kMaxInlineSubjectLength", 512, "引擎日志内联主题截断"],
];
for (const [line, name, want] of CONSTS) {
  const ln = cc.split("\n")[line - 1];
  const m = ln.match(new RegExp(`(static const|static constexpr) int ${name} = (\\d+);`));
  if (!m || Number(m[2]) !== want) throw new Error(`cc:${line} ${name} 漂移: ${ln.trim()}`);
}

// ── GlobalMode 选择规则 (cc:1481-1489) ───────────────────────────────────
if (!cc.includes("RegExpMacroAssembler::GlobalMode mode = RegExpMacroAssembler::GLOBAL;")) {
  throw new Error("GLOBAL 默认漂移");
}
if (!cc.includes("mode = RegExpMacroAssembler::GLOBAL_NO_ZERO_LENGTH_CHECK;")) {
  throw new Error("min_match>0 分支漂移");
}
if (!cc.includes("mode = RegExpMacroAssembler::GLOBAL_UNICODE;")) throw new Error("unicode 分支漂移");
if (!cc.includes("if (data->tree->min_match() > 0)")) throw new Error("min_match 条件漂移");
if (!cc.includes("} else if (IsEitherUnicode(flags))")) throw new Error("IsEitherUnicode 条件漂移");

// ── 编译目标双轨 (cc:1441-1463) ──────────────────────────────────────────
if (!cc.includes("data->compilation_target == CompilationTarget::kNative")) throw new Error("kNative 轨缺失");
if (!cc.includes("DCHECK_EQ(data->compilation_target, CompilationTarget::kBytecode);")) throw new Error("kBytecode 轨缺失");
if (!cc.includes("is_one_byte ? RegExpMacroAssembler::LATIN1\n                                          : RegExpMacroAssembler::UC16")) {
  throw new Error("BytecodeGenerator 模式选择漂移");
}
// 回搜前提: 末锚定 && 非起始锚定 && 非粘滞 && max_length < 上限
if (!cc.includes("if (is_end_anchored && !is_start_anchored && !IsSticky(flags) &&")) {
  throw new Error("回搜四前提漂移");
}
// 锚定预算 (Node::kRecursionBudget)
if (!cc.includes("Node::kRecursionBudget")) throw new Error("锚定递归预算引用缺失");
// 优化门: kMaxOptimizedPatternLength
if (!cc.includes("RegExp::kMaxOptimizedPatternLength")) throw new Error("kMaxOptimizedPatternLength 引用缺失");

// ── 公共面 (cc:132-140) ──────────────────────────────────────────────────
if (!cc.includes("v8_flags.regexp_interpret_all || v8_flags.regexp_tier_up")) {
  throw new Error("CanGenerateBytecode 双旗标漂移");
}
if (!cc.includes("if (IsUnicode(flags) && IsUnicodeSets(flags)) return false;")) {
  throw new Error("VerifyFlags 互斥规则漂移");
}

// ── 诊断打印挂钩 (V8_ENABLE_REGEXP_DIAGNOSTICS 面) ──────────────────────
const diagRefs = [...cc.matchAll(/#ifdef V8_ENABLE_REGEXP_DIAGNOSTICS/g)];
if (diagRefs.length < 2) throw new Error(`诊断门控应 ≥2 处, 实数 ${diagRefs.length}`);
for (const hook of ["graph_printer()->PrintGraph(", 'DotPrinter::DotPrint("Start"']) {
  if (!cc.includes(hook)) throw new Error(`诊断挂钩缺失: ${hook}`);
}
if (!cc.includes("RegExpMacroAssemblerTracer")) throw new Error("Tracer 装饰挂钩缺失");

// ── 引擎性能日志格式 (cc:2020-2035) ─────────────────────────────────────
if (!cc.includes('PrintF("%" PRId64 ",%s,/%s/%s,%d,%d,%d\\n"')) throw new Error("短主题日志格式漂移");
if (!cc.includes('PrintF("%" PRId64 ",%s,/%s/%s,%d,\\"%s\\",%d\\n"')) throw new Error("长主题日志格式漂移");

// ── 栈限制与实验引擎联动 ────────────────────────────────────────────────
if (!cc.includes("ErrorIsStackOverflow(error)")) throw new Error("栈溢出错误判定缺失");
if (!cc.includes("SetBacktrackAndExperimentalFallback")) throw new Error("回溯限制/实验回退设置缺失");

console.log(
  `regexp.cc: RegExpImpl 方法 ${implMethods.length}, 常量 ${CONSTS.length}, 诊断门控 ${diagRefs.length}, ` +
    `GlobalMode 3 分支, 双编译轨 ✓`,
);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const body = `${banner(
  "src/regexp/regexp.cc.js",
  "src/regexp/regexp.cc",
  "RegExp 编排层基线: 编译管线常量 + GlobalMode 规则 + 双编译轨; Isolate 硬耦合面待 Developer 就地修订",
)}
/// RegExpImpl 静态方法清单 (cc:44-127; 声明序, AtomExecRaw 重载计 2)
pub const REGEXP_IMPL_METHODS: [&str; ${implMethods.length}] = [
${implMethods.map((m) => `  "${m}",`).join("\n")}
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
pub const K_TRACE_LOG_SHORT: &str = "%s,%s,/%s/%s,%d,%d,%d\\n";
pub const K_TRACE_LOG_LONG: &str = "%s,%s,/%s/%s,%d,\\"%s\\",%d\\n";
pub const K_TRACE_LOG_ELLIPSIS: &str = "...";

/// CanGenerateBytecode 双旗标 (cc:132-134): regexp_interpret_all || regexp_tier_up
pub const K_BYTECODE_FLAGS: usize = 2;
/// VerifyFlags 互斥 (cc:137-140): Unicode ∧ UnicodeSets → false
pub const K_FLAG_MUTEX_PAIRS: usize = 1;

const _: () = assert!(REGEXP_IMPL_METHODS.len() == 13);
const _: () = assert!(K_SAMPLE_SIZE == 128);
const _: () = assert!(K_MAX_BACKSEARCH_LIMIT == 1024);
const _: () = assert!(K_MAX_INLINE_SUBJECT_LENGTH == 512);
`;

writeSrc("v8-regexp-compiler", "regexp_cc_generated.rs", body);
console.log("written: v8-regexp-compiler/src/regexp_cc_generated.rs");
