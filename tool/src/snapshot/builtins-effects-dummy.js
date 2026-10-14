#!/usr/bin/env bun
// tool/src/snapshot/builtins-effects-dummy.js
// 机械提取 v8/src/snapshot/builtins-effects-dummy.cc 的 mksnapshot 副作用桩

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/snapshot/builtins-effects-dummy.cc");

// 1. 桩函数 (唯一): BuiltinCanAllocate → true
const fnM = src.match(/bool BuiltinCanAllocate\(Builtin builtin\) \{ return true; \}/);
if (!fnM) throw new Error("未找到 BuiltinCanAllocate 桩");
if (!src.includes("builtins-generated/builtins-effects.cc")) {
  throw new Error("未找到生成文件替代说明");
}
console.log("桩 1 条 (BuiltinCanAllocate → true)");

let rust = banner("src/snapshot/builtins-effects-dummy.js", "src/snapshot/builtins-effects-dummy.cc", "mksnapshot 专用内建副作用桩");
rust += `
/// mksnapshot 链接版 BuiltinCanAllocate (原版 builtins-effects-dummy.cc):
/// mksnapshot 期间保守假设所有内建均可分配内存, 恒返回 true。
/// 正式版由 mksnapshot 生成至 gen 产物 builtins-effects.cc,
/// V8/D8 链接生成版, 本桩仅 mksnapshot 链接。
#[must_use]
pub const fn builtin_can_allocate(_builtin: u32) -> bool {
    true
}
`;

const p = writeSrc("v8-snapshot", "builtins_effects_dummy.rs", rust);
console.log(`written ${p}`);
