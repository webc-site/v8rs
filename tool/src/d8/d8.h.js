#!/usr/bin/env bun
// tool/src/d8/d8.h.js
// 机械提取 v8/src/d8/d8.{h,cc} (d8.cc 8709 行 → 只取常量/枚举/旗标表; d8.h 全量结构)
// 产物: rs8/src/d8.rs — Shell 体系枚举 + ShellOptions 旗标表 + 方法骨架台账

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/d8/d8.h");
const ccSrc = readV8("src/d8/d8.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// ── d8.h ────────────────────────────────────────────────────────────────────
// 1. ModuleType 枚举 6
const moduleType = hSrc.match(/enum class ModuleType \{([^}]+)\};/);
must(moduleType, "ModuleType 枚举");
const moduleTypes = [...moduleType[1].matchAll(/(k\w+)/g)].map((m) => m[1]);
if (moduleTypes.length !== 6) fail("ModuleType 条目", moduleTypes.length, 6);

// 2. Counter / CounterCollection 常量
const maxName = hSrc.match(/static const int kMaxNameSize = (\d+);/);
must(maxName, "Counter::kMaxNameSize");
const maxCounters = hSrc.match(/static const unsigned kMaxCounters = (\d+);/);
must(maxCounters, "CounterCollection::kMaxCounters");

// 3. Worker::State 5 态
const workerState = hSrc.match(/enum class State \{\n    kReady,([\s\S]*?)\n  \};/);
must(workerState, "Worker::State 枚举");
const workerStates = ["kReady", ...[...workerState[1].matchAll(/^\s+(k\w+),/gm)].map((m) => m[1])];
if (workerStates.length !== 5) fail("Worker::State 条目", workerStates.length, 5);

// 4. Shell 四组枚举
const enumOf = (label, body, want) => {
  const names = [...body.matchAll(/(k\w+)/g)].map((m) => m[1]);
  if (names.length !== want) fail(label, names.length, want);
  return names;
};
const printResult = hSrc.match(/enum PrintResult : bool \{([^}]+)\};/);
must(printResult, "PrintResult");
const reportEx = hSrc.match(/enum ReportExceptions : bool \{([\s\S]*?)\n  \};/);
must(reportEx, "ReportExceptions");
const processMq = hSrc.match(/enum ProcessMessageQueue : bool \{([\s\S]*?)\n  \};/);
must(processMq, "ProcessMessageQueue");
const codeType = hSrc.match(/enum class CodeType \{([^}]+)\};/);
must(codeType, "CodeType");
const codeTypes = enumOf("CodeType", codeType[1], 5);

// 5. ShellOptions::CodeCacheOptions 3
const cco = hSrc.match(/enum CodeCacheOptions \{([^}]+)\};/);
must(cco, "CodeCacheOptions");
const codeCacheOptions = [...cco[1].matchAll(/(k\w+)/g)].map((m) => m[1]);
if (codeCacheOptions.length !== 3) fail("CodeCacheOptions 条目", codeCacheOptions.length, 3);

// 6. PerIsolateData::kMainRealmIndex = 0
must(hSrc.includes("static constexpr int kMainRealmIndex = 0;"), "kMainRealmIndex = 0");

// 7. ShellOptions 旗标表 (DisallowReassignment 条目 + 平台门控 + 朴素字段)
const soBody = hSrc.match(/class ShellOptions \{([\s\S]*?)\n\};/);
must(soBody, "ShellOptions 类体");
const disallowAll = [...soBody[1].matchAll(/DisallowReassignment<[^>]+>\s+(\w+)/g)].map((m) => m[1]);
if (disallowAll.length !== 56) fail("DisallowReassignment 条目", disallowAll.length, 56);
// 平台门控: V8_OS_LINUX ×3 / V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT ×2 / WASM ×1
const linuxGate = soBody[1].match(/#ifdef V8_OS_LINUX([\s\S]*?)#endif/);
must(linuxGate, "V8_OS_LINUX 门控块");
const linuxFlags = [...linuxGate[1].matchAll(/DisallowReassignment<[\w:, <>()]*>\s+(\w+)/g)].map((m) => m[1]);
if (linuxFlags.length !== 3) fail("linux 门控旗标", linuxFlags.length, 3);
const wpFlags = [...soBody[1].matchAll(/#ifdef V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT[\s\S]*?#endif/g)].flatMap((m) =>
  [...m[0].matchAll(/DisallowReassignment<[\w:, <>()]*>\s+(\w+)/g)].map((x) => x[1]),
);
if (wpFlags.length !== 2) fail("watchpoint 门控旗标", wpFlags.length, 2);
const wasmFlags = [...soBody[1].matchAll(/#if V8_ENABLE_WEBASSEMBLY[\s\S]*?#endif/g)].flatMap((m) =>
  [...m[0].matchAll(/DisallowReassignment<[\w:, <>()]*>\s+(\w+)/g)].map((x) => x[1]),
);
if (wasmFlags.length !== 1) fail("wasm 门控旗标", wasmFlags.length, 1);
// 旗标名字符串 (= {"flag-name", …}) — 按条目切分, 字段名/旗标名各取其一
const flagNames = soBody[1]
  .split(/(?=DisallowReassignment<)/)
  .slice(1)
  .map((entry) => {
    const field = entry.match(/^DisallowReassignment<[^>]+>\s+(\w+)/)[1];
    const flag = entry.match(/=\s*\{?\s*\n?\s*"([^"]+)"/)[1];
    return [field, flag];
  });
if (flagNames.length !== 56) fail("旗标字符串数", flagNames.length, 56);
// 朴素字段
for (const f of ["post_filtering_cwd_index", "test_shell", "num_isolates", "arguments"]) {
  if (!soBody[1].includes(f)) fail(`朴素字段 ${f}`, false, true);
}

// 8. Shell 类静态面: 公开 137 / 私有 (class Shell 区间) — 机械行提取
const shellStart = hSrc.indexOf("class Shell : public i::AllStatic {");
must(shellStart > 0, "class Shell 起点");
const shellEnd = hSrc.indexOf("\n};", shellStart);
must(shellEnd > shellStart, "class Shell 终点");
const shellBody = hSrc.slice(shellStart, shellEnd);
const privIdx = shellBody.indexOf("\n private:");
must(privIdx > 0, "Shell private 分界");
const pubStatics = [...shellBody.slice(0, privIdx).matchAll(/^\s+static (.+)$/gm)].map((m) => m[1].trim());
// 137 含 Source::FromString/FromFile (4 空格缩进, 嵌套类)
if (pubStatics.length !== 137) fail("Shell 公开静态数", pubStatics.length, 137);
const privStatics = [...shellBody.slice(privIdx).matchAll(/^\s+static (.+)$/gm)].map((m) => m[1].trim());
if (privStatics.length !== 65) fail("Shell 私有静态数", privStatics.length, 65);

// ── d8.cc 常量/枚举 ────────────────────────────────────────────────────────
must(ccSrc.includes("bool check_d8_flag_contradictions = true;"), "check_d8_flag_contradictions = true");
must(ccSrc.includes("bool exit_on_flag_contradictions = false;"), "exit_on_flag_contradictions = false");
must(ccSrc.includes("bool Shell::fuzzilli_reprl_failed_ = false;"), "fuzzilli_reprl_failed_ = false");

// JSApiInstanceType 6
const jsApi = ccSrc.match(/enum class JSApiInstanceType : uint16_t \{([^}]+)\};/);
must(jsApi, "JSApiInstanceType 枚举");
const jsApiTypes = [...jsApi[1].matchAll(/^\s*(k\w+)/gm)].map((m) => m[1]);
if (jsApiTypes.length !== 6) fail("JSApiInstanceType 条目", jsApiTypes.length, 6);

// 模块嵌入器数据下标 / 模块解析数据下标
must(ccSrc.includes("enum { kModuleEmbedderDataIndex, kInspectorClientIndex };"), "kModuleEmbedderDataIndex 对");
const modRes = ccSrc.match(/enum ModuleResolutionDataIndex : uint32_t \{([^}]+)\};/);
must(modRes, "ModuleResolutionDataIndex 枚举");

// FuzzerMonitor 7 私有 + 1 公开
const fmBody = hSrc.match(/class FuzzerMonitor : public i::AllStatic \{([\s\S]*?)\n\};/);
must(fmBody, "FuzzerMonitor 类体");
const fmMethods = [...fmBody[1].matchAll(/^\s+(?:static )?(?:void|\w+) (\w+)\(\)/gm)].map((m) => m[1]);
if (fmMethods.length !== 8) fail("FuzzerMonitor 方法", fmMethods.length, 8);

let n = 25;
const rust = `${banner("src/d8/d8.h.js", "src/d8/d8.{h,cc}", "d8 Shell 体系: 枚举全集 + ShellOptions 旗标表 + 静态方法台账")}
//! d8 shell 主体系 — 原版 namespace v8::{Shell, ShellOptions, Worker, …} (d8.h + d8.cc 常量面)

/// 模块类型 (原版 enum class ModuleType, 6 值)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
${moduleTypes.map((t) => `  /// 原版 ${t}\n  ${t.replace(/^k/, "")},`).join("\n")}
}

/// 计数器名上限 (原版 Counter::kMaxNameSize)
pub const COUNTER_MAX_NAME_SIZE: usize = ${maxName[1]};
/// 计数器集合容量 (原版 CounterCollection::kMaxCounters)
pub const COUNTER_COLLECTION_MAX: usize = ${maxCounters[1]};

/// Worker 状态机 (原版 Worker::State, 5 态)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
${workerStates.map((t, i) => `  /// 原版 ${t}\n  ${t.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// Shell::CodeType (原版 enum class CodeType, 5 值)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeType {
${codeTypes.map((t, i) => `  /// 原版 ${t}\n  ${t.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 代码缓存选项 (原版 ShellOptions::CodeCacheOptions, 3 值)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeCacheOptions {
${codeCacheOptions.map((t, i) => `  /// 原版 ${t}\n  ${t.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 主 Realm 下标 (原版 PerIsolateData::kMainRealmIndex)
pub const MAIN_REALM_INDEX: i32 = 0;

/// d8 旗标矛盾检查开关 (原版 d8.cc 全局 bool, 默认 true)
pub const CHECK_D8_FLAG_CONTRADICTIONS: bool = true;
/// 矛盾即退出 (原版 d8.cc 全局 bool, 默认 false)
pub const EXIT_ON_FLAG_CONTRADICTIONS: bool = false;

/// JSApiInstanceType (原版 d8.cc enum class : uint16_t, 6 值; DOM 模板实例类型)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum JSApiInstanceType {
${jsApiTypes.map((t, i) => `  /// 原版 ${t}${i === 0 ? " (= FunctionTemplateInfo::kNoJSApiObjectType)" : ""}\n  ${t.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// ShellOptions 旗标表 (原版 DisallowReassignment 条目 56; flag 名为 CLI 拼写)
/// 门控: scope_linux_perf*/perf_*_fd ×3 (V8_OS_LINUX); memory_corruption* ×2
/// (V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT); wasm_trap_handler ×1 (V8_ENABLE_WEBASSEMBLY)
pub const SHELL_OPTIONS: [(&str, &str); ${flagNames.length}] = [
  // (字段名, CLI 旗标名)
${flagNames.map(([f, n2]) => `  (${JSON.stringify(f)}, ${JSON.stringify(n2)}),`).join("\n")}
];

/// 朴素字段 (非 DisallowReassignment): post_filtering_cwd_index(i32=-1) /
///   test_shell(bool) / num_isolates(i32=1) / arguments(Vec<&str>)

/// Shell 公开静态方法面 (原版 137 条声明, 定义序 1:1)
pub const SHELL_PUBLIC_STATICS: [&str; ${pubStatics.length}] = [
${pubStatics.map((s) => `  ${JSON.stringify(s)},`).join("\n")}
];

/// Shell 私有静态面 (原版 65 条声明, 定义序 1:1)
pub const SHELL_PRIVATE_STATICS: [&str; ${privStatics.length}] = [
${privStatics.map((s) => `  ${JSON.stringify(s)},`).join("\n")}
];

// —— 枚举骨架补遗 (bool 语义枚举 3 组, 各 2 值) ——
// PrintResult: kPrintResult(true) / kNoPrintResult(false)
// ReportExceptions: kReportExceptions(true) / kNoReportExceptions(false)
// ProcessMessageQueue: kProcessMessageQueue(true) / kNoProcessMessageQueue(false)

// —— FuzzerMonitor (原版 AllStatic, 8 方法 1:1) ——
// simulate_errors() 公开; 私有: control_flow_violation / d_check / fatal /
//   observable_difference / undefined_behavior / use_after_free / use_of_uninitialized_value

// d8.cc 侧台账:
//   enum { kModuleEmbedderDataIndex, kInspectorClientIndex } — 上下文嵌入器数据槽
//   enum ModuleResolutionDataIndex : uint32_t { kResolver = 0, kNamespaceOrSource = 1 }
//   Shell::options / Shell::stringify_source_ (见 d8_js.rs) / fuzzilli_reprl_failed_ = false

// 平台分支台账:
//   V8_OS_LINUX (perf 3 旗标) — x64/arm64 Linux 目标展开
//   V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT (2 旗标) — 仅 x64+Linux 展开, arm64 跳过
//   V8_ENABLE_WEBASSEMBLY (wasm_trap_handler) — 按特性开关
//   V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA (GetContinuationPreservedEmbedderData)
//     — 按特性开关登记
`;

const p = writeSrc("rs8", "d8.rs", rust);
console.log(`OK d8.h.js: 对数断言 ${n}+ 全过; 枚举 7 组 / 旗标表 ${flagNames.length} (门控 6) / Shell 静态 137+65 / FuzzerMonitor 8; 产物 ${p}`);
