#!/usr/bin/env bun
// tool/src/regexp/experimental/experimental.cc.js
//   — v8/src/regexp/experimental/experimental.{h,cc} (顶层入口族)
//   → v8-regexp-exec/src/experimental/experimental.cc.rs.skel
// 机械提取: ExperimentalRegExp 9 静态方法签名 (h) + cc 定义全集 + 文件级辅助 5 件
//   (VectorToByteArray/CompilationResult/CompileImpl/AsInstructionSequence/ExecRawImpl);
// 对数: h 声明数 == cc 定义数; trace 输出 5 处/VERIFY_HEAP 3 处/沙箱硬件 1 处/
//   kIsLatin1 ×2/kFromRuntime ×2/kInternalRegExpRetry ×2 断言; 不臆测, 语义面留 /// v8: 锚
import { banner, readV8, writeSrc } from "../../../lib.js";

const h = readV8("src/regexp/experimental/experimental.h");
const cc = readV8("src/regexp/experimental/experimental.cc");

let n = 0;
const must = (c, m) => { n++; if (!c) throw new Error(m); };
const cnt = (s, re) => (s.match(re) || []).length;

// ── h 侧: 9 静态方法 + 哨兵 ────────────────────────────────────────────────
const hSigs = [...h.matchAll(/^ {2}(?:V8_WARN_UNUSED_RESULT\n {2})?(static (?:bool|void|int32_t|std::optional<int>)) (\w+)\(/gm)]
  .map((m) => ({ ret: m[1].replace(/^static /, ""), name: m[2] }));
const WANT = ["CanBeHandled", "Initialize", "IsCompiled", "Compile", "MatchForCallFromJs", "Exec", "ExecRaw", "OneshotExec", "OneshotExecRaw"];
if (hSigs.length !== 9 || hSigs.map((s) => s.name).join(",") !== WANT.join(",")) {
  throw new Error(`experimental.h 9 方法签名漂移: ${hSigs.map((s) => s.name).join(",")}`);
}
must(h.includes("static constexpr bool kSupportsUnicode = false;"), "h: kSupportsUnicode=false 缺失");
must(h.includes("class ExperimentalRegExp final : public AllStatic"), "h: AllStatic 类锚缺失");

// ── cc 侧: 9 定义逐名对数 + 5 文件级辅助 ──────────────────────────────────
const ccDefs = [...cc.matchAll(/\bExperimentalRegExp::(\w+)\(/g)].map((m) => m[1]);
const ccWant = ["CanBeHandled", "Initialize", "IsCompiled", "Compile", "ExecRaw", "MatchForCallFromJs", "Exec", "OneshotExecRaw", "OneshotExec"];
if (ccDefs.length !== 9 || ccDefs.join(",") !== ccWant.join(",")) {
  throw new Error(`experimental.cc 定义漂移 (${ccDefs.length}): ${ccDefs.join(",")}`);
}
for (const anchor of [
  ["template <class T>\nDirectHandle<TrustedByteArray> VectorToByteArray", "VectorToByteArray 模板"],
  ["struct CompilationResult {", "CompilationResult 结构"],
  ["std::optional<CompilationResult> CompileImpl(", "CompileImpl"],
  ["base::Vector<Instruction> AsInstructionSequence(", "AsInstructionSequence"],
  ["int32_t ExecRawImpl(", "ExecRawImpl"],
]) must(cc.includes(anchor[0]), `cc: ${anchor[1]} 缺失`);

// ── 条件编译与语义锚计数 ──────────────────────────────────────────────────
const checks = [
  [/#ifdef VERIFY_HEAP/g, 3, "VERIFY_HEAP ×3 (IsCompiled/Compile/Exec)"],
  [/#ifdef V8_ENABLE_SANDBOX_HARDWARE_SUPPORT/g, 1, "沙箱硬件 ×1 (MatchForCallFromJs 前 DISABLE_ASAN)"],
  [/DISABLE_ASAN/g, 1, "DISABLE_ASAN ×1"],
  [/v8_flags\.trace_experimental_regexp_engine\b/g, 5, "trace 输出 5 (CanBeHandled/Initialize/Compile/ExecRaw/OneshotExecRaw)"],
  [/static constexpr bool kIsLatin1 = true;/g, 2, "kIsLatin1 ×2 (IsCompiled/ExecRaw)"],
  [/RegExp::kFromRuntime/g, 2, "kFromRuntime ×2 (Exec/OneshotExecRaw)"],
  [/RegExp::kFromJs/g, 1, "kFromJs ×1 (MatchForCallFromJs)"],
  [/RegExp::kInternalRegExpRetry/g, 2, "kInternalRegExpRetry ×2 (Exec/OneshotExec 重试环)"],
  [/RegExp::kInternalRegExpException/g, 1, "kInternalRegExpException ×1 (OneshotExecRaw)"],
  [/UNREACHABLE\(\);/g, 2, "UNREACHABLE ×2 (Exec/OneshotExec do-while 尾)"],
  [/ExperimentalCompiler::CanBeHandled\(/g, 1, "委托 ExperimentalCompiler::CanBeHandled"],
  [/ExperimentalCompiler::Compile\(/g, 1, "委托 ExperimentalCompiler::Compile"],
  [/ExperimentalRegExpInterpreter::FindMatches\(/g, 1, "委托 FindMatches"],
  [/RegExp::ThrowRegExpException\(/g, 1, "解析栈溢出抛错"],
  [/RegExp::CreateCaptureNameMap\(/g, 1, "捕获名表"],
  [/SetBytecodeForExperimental\(/g, 1, "回写 IrRegExpData"],
  [/SBXCHECK_EQ\(/g, 1, "SBXCHECK_EQ capture_count"],
  [/JSRegExp::RegistersForCaptureCount\(/g, 4, "寄存器数换算 ×4"],
];
for (const [re, want, label] of checks) {
  must(cnt(cc, re) === want, `${label}: 实得 ${cnt(cc, re)} want ${want}`);
}
// IsCompiled 判定: EXPERIMENTAL 型标 + latin1 字节码在场
must(cc.includes("DCHECK_EQ(re_data->type_tag(), RegExpData::Type::EXPERIMENTAL);"), "EXPERIMENTAL 型标断言缺失");

console.log(`experimental: h 方法 ${hSigs.length}, cc 定义 ${ccDefs.length}, 辅助 5, 断言 ${n} 条全过 ✓`);

// ── 生成 Rust 骨架 ────────────────────────────────────────────────────────
const rust = [];
rust.push(banner(
  "src/regexp/experimental/experimental.cc.js",
  "src/regexp/experimental/experimental.{h,cc}",
  "实验引擎顶层入口: 9 静态方法签名骨架 + 编译/执行通道处置表 (审计基线)",
));
rust.push(`
//! ExperimentalRegExp (AllStatic) — 实验引擎对外入口面。
//! 通道: CompileImpl(解析+ExperimentalCompiler::Compile+VectorToByteArray)
//!   → SetBytecodeForExperimental; ExecRawImpl → ExperimentalRegExpInterpreter::FindMatches;
//!   Exec/OneshotExec 的 do-while 重试环消化 kInternalRegExpRetry。

/// v8: src/regexp/experimental/experimental.h ExperimentalRegExp::kSupportsUnicode
pub const SUPPORTS_UNICODE: bool = false;

/// 9 方法处置表 (h 声明 == cc 定义, 逐名 1:1):
/// - CanBeHandled → 委托 ExperimentalCompiler::can_be_handled (trace 拒绝打印 1)
/// - Initialize → factory SetRegExpExperimentalData (JSRegExp 层)
/// - IsCompiled → type_tag==EXPERIMENTAL && has_bytecode(kIsLatin1=true)
/// - Compile → CompileImpl + SetBytecodeForExperimental + set_capture_name_map
/// - MatchForCallFromJs → 沙箱出口 (ExitSandboxScope) + SbxCast + ExecRaw(kFromJs)
/// - Exec → IsCompiled?→Compile→Flatten→ExecRaw 环 (kFromRuntime)
/// - ExecRaw → bytecode(kIsLatin1) + ExecRawImpl
/// - OneshotExecRaw → CHECK(on_excessive_backtracks) + CompileImpl 现算现执行
/// - OneshotExec → 同 Exec 环但 OneshotExecRaw
pub struct ExperimentalRegExp;`);
for (const s of hSigs) {
  rust.push(`
impl ExperimentalRegExp {
  /// v8: src/regexp/experimental/experimental.cc ExperimentalRegExp::${s.name}
  pub fn ${s.name.toLowerCase()}(/* ${s.ret} — 签名见 cc; 参数面 8 通道异构, 落地时按调用方收窄 */) {
    unimplemented!("登记骨架")
  }
}`);
}
rust.push(`
// 文件级辅助 5 件 (cc 匿名/具名 namespace):
// /// v8: cc VectorToByteArray<T> — trivial T 数组 → TrustedByteArray (MemCopy)
// /// v8: cc struct CompilationResult { bytecode, capture_name_map }
// /// v8: cc CompileImpl — Parser::ParseRegExpFromHeapString + 失败仅 kStackOverflow
// /// v8: cc AsInstructionSequence — u8 视图按 sizeof(Instruction) 重解释 (对齐前提)
// /// v8: cc ExecRawImpl — RegistersForCaptureCount + FindMatches
// 条件编译: #ifdef VERIFY_HEAP ×3 (排除), 沙箱硬件 DISABLE_ASAN ×1 (排除);
//   kIsLatin1=true 恒真 (实验引擎无 uc16 字节码两态)。
`);

writeSrc("v8-regexp-exec", "experimental/experimental.cc.rs.skel", rust.join("\n") + "\n");
console.log("written: v8-regexp-exec/src/experimental/experimental.cc.rs.skel");
