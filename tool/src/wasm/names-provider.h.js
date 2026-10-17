#!/usr/bin/env bun
// tool/src/wasm/names-provider.h.js
// 机械提取 v8/src/wasm/names-provider.h 的名称行为枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/names-provider.h");

const fnbMatch = hSrc.match(/enum FunctionNamesBehavior : bool \{ kWasmInternal = false, kDevTools = true \};/);
if (!fnbMatch) throw new Error("未找到 FunctionNamesBehavior");

const iacMatch = hSrc.match(/enum IndexAsComment : bool \{\s*kDontPrintIndex = false,\s*kIndexAsComment = true\s*\};/s);
if (!iacMatch) throw new Error("未找到 IndexAsComment");

console.log("names-provider.h: 2 enums");

let rust = banner(
  "src/wasm/names-provider.h.js",
  "src/wasm/names-provider.h",
  "名称提供器的显示行为枚举",
);

rust += `
/// 函数名展示行为 (对应原版 NamesProvider::FunctionNamesBehavior)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(bool)]
pub enum FunctionNamesBehavior {
  /// kWasmInternal — 仅 name section 原始名
  WasmInternal = false,
  /// kDevTools — 前缀 '$', 以导入/导出名回退, 缺省 "$funcN"
  DevTools = true,
}

/// 是否将索引作为注释附带 (对应原版 NamesProvider::IndexAsComment)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(bool)]
pub enum IndexAsComment {
  /// kDontPrintIndex
  DontPrintIndex = false,
  /// kIndexAsComment
  IndexAsComment = true,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FunctionNamesBehavior::DevTools as bool);
  assert!(!IndexAsComment::DontPrintIndex as bool);
};
`;

const p = writeSrc("v8-wasm-core", "names_provider_consts.rs", rust);
console.log(`written ${p}`);
