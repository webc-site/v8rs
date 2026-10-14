#!/usr/bin/env bun
// tool/src/wasm/module-compiler.h.js
// 机械提取 v8/src/wasm/module-compiler.h 的编译任务调度枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/module-compiler.h");

const ueftMatch = hSrc.match(/enum UseExistingForegroundTask : bool \{\s*kUseExistingForegroundTask = true,\s*kAssertNoExistingForegroundTask = false\s*\};/s);
if (!ueftMatch) throw new Error("未找到 UseExistingForegroundTask");

console.log("module-compiler.h: UseExistingForegroundTask=2");

let rust = banner(
  "src/wasm/module-compiler.h.js",
  "src/wasm/module-compiler.h",
  "编译任务前台调度策略枚举",
);

rust += `
/// 是否复用已存在的前台任务 (对应原版 CompilationStateImpl::UseExistingForegroundTask)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(bool)]
pub enum UseExistingForegroundTask {
  /// kUseExistingForegroundTask — 复用, 避免派生重复任务
  UseExistingForegroundTask = true,
  /// kAssertNoExistingForegroundTask — 断言不存在并新建
  AssertNoExistingForegroundTask = false,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(UseExistingForegroundTask::UseExistingForegroundTask as bool);
};
`;

const p = writeSrc("v8-wasm-core", "module_compiler_consts.rs", rust);
console.log(`written ${p}`);
