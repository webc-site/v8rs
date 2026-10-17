#!/usr/bin/env bun
// tool/src/base/logging.cc.js — 族: src/base/logging.cc (logging.h 归别席/既有)
// 覆盖: 3 个回调全局 + Set* 4 + FatalOOM/FatalNoSecurityImpact + V8_Fatal/V8_Dcheck
//        + FailureMessage (栈上标记) + PrettyPrintChar + 2 组 X-macro 展开
// → v8-base/src/logging_cc.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/logging.cc");

// —— 锚点断言 ——
if (!src.includes("void (*g_print_stack_trace)() = nullptr;")) throw new Error("缺 g_print_stack_trace");
if (!src.includes("void (*g_dcheck_function)(const char*, int, const char*) = DefaultDcheckHandler;")) throw new Error("缺 g_dcheck_function");
if (!src.includes("void (*g_fatal_function)(const char*, int, const char*) = nullptr;")) throw new Error("缺 g_fatal_function");
if (!src.includes("static const uintptr_t kStartMarker = 0xdecade10;")) throw new Error("缺起始标记");
if (!src.includes("static const uintptr_t kEndMarker = 0xdecade11;")) throw new Error("缺结束标记");
if (!src.includes("static const int kMessageBufferSize = 512;")) throw new Error("缺缓冲尺寸");

// —— 真解析: X-macro 展开对数 ——
const charCases = [...src.matchAll(/^    CHAR_PRINT_CASE\('(.*)'\)/gm)].map((m) => m[1]);
if (charCases.length !== 10) throw new Error(`CHAR_PRINT_CASE ${charCases.length} != 10`);
const charTypes = [...src.matchAll(/^DEFINE_PRINT_CHECK_OPERAND_CHAR\(([\w ]+)\)/gm)].map((m) => m[1].trim());
if (charTypes.join(",") !== "char,signed char,unsigned char") throw new Error(`char 类型 ${charTypes}`);
const checkOpTypes = [...src.matchAll(/^DEFINE_MAKE_CHECK_OP_STRING\(([\w *]+)\)/gm)].map((m) => m[1].trim());
if (checkOpTypes.length !== 7) throw new Error(`CHECK_OP 实例化 ${checkOpTypes.length} != 7`);
const setters = ["SetPrintStackTrace", "SetDcheckFunction", "SetFatalFunction"];
for (const s of setters) if (!src.includes(`void ${s}(`)) throw new Error(`缺 ${s}`);
if (!src.includes("void FatalOOM(OOMType type, const char* msg)")) throw new Error("缺 FatalOOM");
if (!src.includes("void FatalNoSecurityImpact(const char* format, ...)")) throw new Error("缺 FatalNoSecurityImpact");
const fatalSigs = (src.match(/^void V8_Fatal\(/gm) || []).length;
const dcheckSigs = (src.match(/^void V8_Dcheck\(/gm) || []).length;
if (fatalSigs !== 2 || dcheckSigs !== 2) throw new Error(`V8_Fatal ${fatalSigs}/V8_Dcheck ${dcheckSigs} 编译级双签名异常`);
console.log(`logging.cc: 回调全局 3, Setter 3, Fatal 类 4 (FatalOOM/FatalNoSecurityImpact/V8_Fatal/V8_Dcheck), X-macro char 10 案 + 类型 3 + 实例化 7`);

let rust = banner(
  "src/base/logging.cc.js",
  "src/base/logging.cc",
  "日志后端 (回调注入 + 栈上 FailureMessage + OOM/Fatal 处理)",
);

rust += `
//! v8: src/base/logging.cc — 对应 logging.h 声明 (归 base-core 顶层)

// v8: 匿名命名空间回调全局 (Rust: static mut / OnceLock<fn>)
// g_print_stack_trace = null; g_dcheck_function = DefaultDcheckHandler; g_fatal_function = null

/// v8: logging.cc SetPrintStackTrace / SetDcheckFunction (null 回落默认) / SetFatalFunction
pub fn set_print_stack_trace(f: Option<fn()>);
pub fn set_dcheck_function(f: Option<fn(&'static str, i32, &'static str)>);
pub fn set_fatal_function(f: Option<fn(&'static str, i32, &'static str)>);

/// v8: logging.cc PrettyPrintChar — 10 个转义字符 X-macro 案 (${charCases.map((c) => `'${c}'`).join(" ")});
/// 其余 isprint 则 'c', 否则 \\xNN
pub fn pretty_print_char(ch: u8) -> String;

/// v8: logging.cc FatalOOM(type, msg)
/// kProcess → "process", 否则 "JavaScript"; 打印后按 abort-mode 选 ExitProcess(-1) 或 Abort()
pub fn fatal_oom(type_: OomType, msg: &str) -> !;

/// v8: logging.cc FatalNoSecurityImpact(format, ...)
/// 同 FatalOOM 的 abort-mode 分叉, 面向无安全影响的致命错
pub fn fatal_no_security_impact(args: std::fmt::Arguments) -> !;

/// v8: logging.cc V8_Fatal (V8_LOGGING_LEVEL==2 双签名: 带/不带 file,line)
/// 流程: FailureMessage 入栈 → g_fatal_function 可选接管 → ControlledCrashesAreHarmless
///       分支打 "Safely terminating" 前缀 → 打消息与对象地址 → 栈回溯 → Abort
pub fn v8_fatal(file: &str, line: i32, args: std::fmt::Arguments) -> !;

/// v8: logging.cc V8_Dcheck — DcheckFailuresAreIgnored 则仅打印并返回, 否则走 g_dcheck_function
pub fn v8_dcheck(file: &str, line: i32, message: &str);

/// v8: logging.cc FailureMessage — 栈上定长消息, 首尾哨兵供崩溃处理器从栈中取回
pub struct FailureMessage {
    /// v8: kStartMarker = 0xdecade10
    pub start_marker: usize,
    /// v8: kMessageBufferSize = 512
    pub message: [u8; 512],
    /// v8: kEndMarker = 0xdecade11
    pub end_marker: usize,
}

pub const START_MARKER: usize = 0xdecade10; // v8: FailureMessage::kStartMarker
pub const END_MARKER: usize = 0xdecade11;   // v8: FailureMessage::kEndMarker
pub const MESSAGE_BUFFER_SIZE: usize = 512; // v8: FailureMessage::kMessageBufferSize

// v8: PrintCheckOperand 特化 (char/signed char/unsigned char → PrettyPrintChar;
//     char*/const char* → 按 void* 指针打印而非字符串)
// v8: MakeCheckOpString 显式实例化 7 类型: ${checkOpTypes.join(", ")}
`;

// 产物名 logging_cc: logging.h (别席) 产出 logging.rs.skel, 本 .cc 族错开防互覆
writeSrc("v8-base", "logging_cc.rs.skel", rust);
console.log("written v8-base/src/logging_cc.rs.skel");
