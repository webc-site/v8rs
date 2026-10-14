#!/usr/bin/env bun
// tool/src/debug/core.js — Debug 核心组 (断点/单步/异常中断面, 调试接口优先)
// 覆盖: src/debug/debug.h/.cc + debug-interface.h/.cc + interface-types.h (5 文件)
// 产出: v8-debug/src/core.rs.skel (规则代码: StepAction/ExceptionBreakType/DebugBreakType/
//       BreakReason/CoverageMode 等枚举全量 + Debug/BreakLocation 类面骨架)
// 复跑幂等: 纯文本扫描 + 源码序输出, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/debug/core.js";
const FILES = [
  "src/debug/debug.h",
  "src/debug/debug.cc",
  "src/debug/debug-interface.h",
  "src/debug/debug-interface.cc",
  "src/debug/interface-types.h",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[core.js] ${m}`); };
const need = (f, s) => { if (!txt[f].includes(s)) die(`${f} 缺针 ${s}`); };

// ① StepAction / ExceptionBreakType / DebugBreakType / IgnoreBreakMode (debug.h)
need("src/debug/debug.h", "enum StepAction : int8_t {");
for (const v of ["StepNone = -1,", "StepOut = 0,", "StepOver = 1,", "StepInto = 2,"]) need("src/debug/debug.h", v);
need("src/debug/debug.h", "enum ExceptionBreakType {");
for (const v of ["BreakCaughtException = 0,", "BreakUncaughtException = 1,"]) need("src/debug/debug.h", v);
need("src/debug/debug.h", "enum DebugBreakType {");
for (const v of ["NOT_DEBUG_BREAK,", "DEBUGGER_STATEMENT,", "DEBUG_BREAK_SLOT_AT_SUSPEND,"]) need("src/debug/debug.h", v);
need("src/debug/debug.h", "enum IgnoreBreakMode {");
need("src/debug/debug.h", "class BreakLocation {");
need("src/debug/debug.h", "class V8_EXPORT_PRIVATE Debug {");
need("src/debug/debug.h", "void Break(JavaScriptFrame* frame, DirectHandle<JSFunction> break_target);");
need("src/debug/debug.h", "class DisableBreak {");
need("src/debug/debug.h", "class V8_NODISCARD DebugScope {");
// ② BreakReason / ExceptionBreakState / 过滤器枚举 (debug-interface.h)
need("src/debug/debug-interface.h", "enum class BreakReason : uint8_t {");
for (const v of ["kAlreadyPaused,", "kStep,", "kException,", "kDebuggerStatement,", "kOOM,", "kScheduled,", "kAgent"]) {
  need("src/debug/debug-interface.h", v);
}
need("src/debug/debug-interface.h", "enum ExceptionBreakState {");
need("src/debug/debug-interface.h", "BreakOnAnyException = 3,");
need("src/debug/debug-interface.h", "enum class PrivateMemberFilter {");
need("src/debug/debug-interface.h", "enum class GetSourceOffsetMode { kStrict, kClamp };");
need("src/debug/debug-interface.h", "enum class ActionAfterInstrumentation {");
need("src/debug/debug-interface.h", "enum class NativeAccessorType {");
need("src/debug/debug-interface.h", "enum class EvaluateGlobalMode {");
need("src/debug/debug-interface.h", "kDisableBreaksAndThrowOnSideEffect");
// ③ CoverageMode / BreakLocationKind (interface-types.h)
need("src/debug/interface-types.h", "enum class CoverageMode {");
for (const v of ["kBestEffort,", "kPreciseCount,", "kPreciseBinary,"]) need("src/debug/interface-types.h", v);
need("src/debug/interface-types.h", "kCommonBreakLocation");

// ④ 机械提取: BreakReason 变体 (源码序, 9 变体)
const brBlock = txt["src/debug/debug-interface.h"].match(/enum class BreakReason[^{]*\{([^}]*)\}/)[1];
const brVars = [...brBlock.matchAll(/k(\w+)/g)].map((m) => m[1]);
if (brVars.length !== 9) die(`BreakReason 变体数=${brVars.length}, 期望 9`);
// ⑤ 机械提取: DebugBreakType 变体 (源码序, 7 变体)
const dbtBlock = txt["src/debug/debug.h"].match(/enum DebugBreakType \{([^}]*)\}/)[1];
const dbtVars = [...dbtBlock.matchAll(/\b([A-Z][A-Z_]+)\b/g)].map((m) => m[1]);
if (dbtVars.length !== 7) die(`DebugBreakType 变体数=${dbtVars.length}, 期望 7`);

console.log(`OK core.js 针核验通过: BreakReason ${brVars.length} + DebugBreakType ${dbtVars.length} 校验全过`);

