#!/usr/bin/env bun
// tool/src/wasm/wasm-result.cc.js
// 机械提取 v8/src/wasm/wasm-result.cc 的 ErrorThrower 错误类型映射与首错覆盖语义

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/wasm/wasm-result.cc");
const hSrc = readV8("src/wasm/wasm-result.h");

// 1. ErrorType 枚举 (h, 按声明序)
const enumBlock = hSrc.match(
  /enum ErrorType \{([\s\S]*?)\};/,
);
if (!enumBlock) throw new Error("未找到 ErrorType 枚举");
const body = enumBlock[1].replace(/\/\/[^\n]*/g, "");
const entries = [...body.matchAll(/\b(k\w+)\s*(?:=\s*([\w]+))?,/g)].map((m) => ({
  name: m[1],
  value: m[2] ?? null,
}));
if (entries.length !== 6) throw new Error(`ErrorType 成员 ${entries.length} != 6`);
const plain = entries.filter((e) => !e.value).map((e) => e.name);
if (plain.join(",") !== "kNone,kTypeError,kRangeError,kCompileError,kLinkError,kRuntimeError") {
  throw new Error(`ErrorType 声明序异常: ${plain.join(",")}`);
}
if (!body.includes("kFirstWasmError = kCompileError")) {
  throw new Error("未找到 kFirstWasmError 别名");
}

// 2. cc: 五个便捷方法 -> 枚举映射
const methods = [...ccSrc.matchAll(/void ErrorThrower::(\w+)\(const char\* format/g)].map((m) => m[1]);
if (methods.join(",") !== "TypeError,RangeError,CompileError,LinkError,RuntimeError") {
  throw new Error(`便捷方法序异常: ${methods.join(",")}`);
}
for (const m of methods) {
  if (!ccSrc.includes(`Format(k${m}, format, arguments);`)) {
    throw new Error(`${m} 未调用 Format(k${m}, ...)`);
  }
}

// 3. cc: Reify 的 constructor 映射
const reifyMap = [
  ["kTypeError", "type_error_function"],
  ["kRangeError", "range_error_function"],
  ["kCompileError", "wasm_compile_error_function"],
  ["kLinkError", "wasm_link_error_function"],
  ["kRuntimeError", "wasm_runtime_error_function"],
];
for (const [variant, ctor] of reifyMap) {
  if (!ccSrc.includes(`case ${variant}:`)) throw new Error(`Reify 缺 case ${variant}`);
  if (!ccSrc.includes(`constructor = isolate_->${ctor}();`)) throw new Error(`Reify 缺 ${ctor}`);
}

// 4. cc: 首错覆盖语义
if (!ccSrc.includes("if (error()) return;")) throw new Error("未找到首错覆盖短路");

// 5. cc: context 前缀格式
if (!ccSrc.includes('PrintFToString(&error_msg_, 0, "%s: ", context_);')) {
  throw new Error("未找到 context 前缀格式");
}

// 6. cc: VPrintFToString 缓冲增长策略 (bit_ceil)
if (!ccSrc.includes("for (;; len = std::bit_ceil(len + 1))")) {
  throw new Error("未找到 bit_ceil 缓冲增长");
}
if (!ccSrc.includes("if (written < 0) continue;")) throw new Error("未找到空间不足重试");

// 7. h: CompileFailed 消息格式
if (!hSrc.includes('CompileError("%s @+%u", error.message().c_str(), error.offset());')) {
  throw new Error("未找到 CompileFailed 格式");
}

console.log(`wasm-result: 枚举=${entries.length} 便捷方法=${methods.length} Reify映射=${reifyMap.length}`);

// 8. 生成 Rust
let rust = banner(
  "src/wasm/wasm-result.cc.js",
  "src/wasm/wasm-result.{h,cc}",
  "ErrorThrower 错误类型枚举与 JS 构造器映射",
);

rust += `
//! wasm 错误抛出器: 首错保留, 析构时 Reify 为 JS 异常。
/// v8: src/wasm/wasm-result.h ErrorThrower

/// 错误类型 (声明序即判别值)
/// v8: src/wasm/wasm-result.h enum ErrorType
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ErrorType {
    /// v8: kNone
    None = 0,
    /// v8: kTypeError (通用错误)
    TypeError = 1,
    /// v8: kRangeError (通用错误)
    RangeError = 2,
    /// v8: kCompileError (wasm 错误)
    CompileError = 3,
    /// v8: kLinkError (wasm 错误)
    LinkError = 4,
    /// v8: kRuntimeError (wasm 错误)
    RuntimeError = 5,
}

/// v8: kFirstWasmError = kCompileError (wasm_error() 判定下界)
pub const FIRST_WASM_ERROR: u8 = ErrorType::CompileError as u8;

impl ErrorType {
    /// v8: src/wasm/wasm-result.h bool wasm_error()
    pub fn is_wasm_error(self) -> bool {
        self as u8 >= FIRST_WASM_ERROR
    }

    /// Reify 时取的 isolate 构造器名 (语义锚, 对接 objects 层)
    /// v8: src/wasm/wasm-result.cc ErrorThrower::Reify
    pub fn js_constructor(self) -> &'static str {
        match self {
            ErrorType::None => unreachable!(),
            ErrorType::TypeError => "type_error_function",
            ErrorType::RangeError => "range_error_function",
            ErrorType::CompileError => "wasm_compile_error_function",
            ErrorType::LinkError => "wasm_link_error_function",
            ErrorType::RuntimeError => "wasm_runtime_error_function",
        }
    }
}

/// 编译失败消息格式: "<message> @+<offset>"
/// v8: src/wasm/wasm-result.h ErrorThrower::CompileFailed
pub const COMPILE_FAILED_FORMAT: &str = "%s @+%u";

/// context 前缀格式: "<context>: <message>"
/// v8: src/wasm/wasm-result.cc ErrorThrower::Format
pub const CONTEXT_PREFIX_FORMAT: &str = "%s: ";

/// 格式化缓冲增长: bit_ceil(len + 1) 直到写下
/// v8: src/wasm/wasm-result.cc VPrintFToString
pub fn next_buffer_len(len: usize) -> usize {
    (len + 1).next_power_of_two()
}

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(FIRST_WASM_ERROR, 3);
    assert!(!ErrorType::RangeError.is_wasm_error());
    assert!(ErrorType::CompileError.is_wasm_error());
    assert_eq!(next_buffer_len(5), 8);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_result.rs.skel", rust);
console.log(`written ${p}`);
