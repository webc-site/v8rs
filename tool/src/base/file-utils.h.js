#!/usr/bin/env bun
// tool/src/base/file-utils.h.js — 族: src/base/file-utils.h + .cc
// 覆盖: RelativePath (exec_path 目录前缀 + name 拼接)
// → v8-base/src/file_utils.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/base/file-utils.h");
const ccSrc = readV8("src/base/file-utils.cc");

// —— 锚点断言 ——
if (!hSrc.includes("std::unique_ptr<char[]> RelativePath(const char* exec_path, const char* name);")) {
  throw new Error("缺 RelativePath 声明");
}
if (!ccSrc.includes("std::unique_ptr<char[]> RelativePath(const char* exec_path, const char* name) {")) {
  throw new Error("缺 RelativePath 定义");
}
if (!ccSrc.includes("OS::isDirectorySeparator(exec_path[basename_start - 1])")) throw new Error("缺分隔符扫描");
if (!ccSrc.includes("basename_start + name_length + 1")) throw new Error("缺缓冲尺寸");

// —— 真解析: 函数计数 ——
const fnCount = (ccSrc.match(/^std::unique_ptr<char\[\]> \w+\(/gm) || []).length;
if (fnCount !== 1) throw new Error(`函数数 ${fnCount} != 1`);
console.log("file-utils: 函数 1 (RelativePath)");

let rust = banner(
  "src/base/file-utils.h.js",
  "src/base/file-utils.h + src/base/file-utils.cc",
  "可执行路径目录前缀拼接",
);

rust += `
//! v8: src/base/file-utils.h/.cc

/// v8: src/base/file-utils.cc RelativePath(exec_path, name)
/// 语义: 从尾向头找首个目录分隔符 (OS::isDirectorySeparator), 取 [0, pos) 目录前缀,
///       拼 name 返回新串; 无分隔符则只返回 name 拷贝
/// Rust: Path::parent 风格, Developer 补平台分隔符 (win 兼容 '/' 与 '\\\\')
pub fn relative_path(exec_path: &str, name: &str) -> String;
`;

writeSrc("v8-base", "file_utils.rs.skel", rust);
console.log("written v8-base/src/file_utils.rs.skel");
