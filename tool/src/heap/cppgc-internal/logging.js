#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/logging.js — v8/src/heap/cppgc-internal/logging.cc → v8-heap-cppgc/src/logging.rs.skel
// cppgc 日志桥: DCheckImpl/FatalImpl 两出口, 委托 v8 base logging
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/logging.cc";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`logging.cc 缺锚 ${label}: ${needle}`);
  n++;
};

need("void DCheckImpl(const char* message, SourceLocation loc) {", "DCheckImpl 签名");
need("V8_Dcheck(V8_LOG_ARGS_LOC(loc, message));", "DCheckImpl 委托");
need("void FatalImpl(const char* message, SourceLocation loc) {", "FatalImpl 签名");
need("#if V8_LOGGING_LEVEL == 0", "日志级别条件");
need('FATAL("ignored");', "level0 分支");
need('FATAL_WITH_LOC(loc, "Check failed: %s.", message);', "FatalImpl 委托");

const body = `${banner("src/heap/cppgc-internal/logging.js", REL, "cppgc 断言/致命错误出口")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/logging.cc DCheckImpl
pub fn d_check_impl(message: &str, loc: SourceLocation) {
    v8_base_logging::v8_dcheck(loc, message);
}

/// v8: src/heap/cppgc-internal/logging.cc FatalImpl
/// C++ 侧 V8_LOGGING_LEVEL == 0 时打印 "ignored"; Rust 侧以 cfg(feature) 呈现, 语义留锚。
pub fn fatal_impl(message: &str, loc: SourceLocation) {
    v8_base_logging::fatal_with_loc(loc, format_args!("Check failed: {message}."), &format_args!("ignored"));
}
`;

writeSrc("v8-heap-cppgc", "logging.rs.skel", body);
console.log(`logging.js: 对数 ${n} 项 (签名 2 + 委托 3 + 条件 1) → v8-heap-cppgc/src/logging.rs.skel`);
