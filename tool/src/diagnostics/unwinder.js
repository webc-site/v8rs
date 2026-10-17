#!/usr/bin/env bun
// tool/src/diagnostics/unwinder.js
// 机械提取 v8/src/diagnostics/unwinder.{h,cc} → v8-diagnostics/src/unwinder.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/diagnostics/unwinder.h");
const ccSrc = readV8("src/diagnostics/unwinder.cc");

// 对数: 唯一自由函数 Load(Address)
if (!hSrc.includes("internal::Address Load(internal::Address address);")) throw new Error("缺 Load 声明");
if (!ccSrc.includes("i::Address Load(i::Address address)")) throw new Error("cc 缺 Load 实现");

let rust = banner(
  "src/diagnostics/unwinder.js",
  "src/diagnostics/unwinder.{h,cc}",
  "unwinder 栈展开基础函数骨架",
);

rust += `
/// 原版 v8::internal::Load(address): 栈展开器读取远端地址的辅助入口。
/// 沙箱/压缩指针语义由内存访问层接线; 签名骨架。
#[must_use]
pub fn load(_address: u64) -> u64 {
  todo!("待 Developer 修订: 对齐 unwinder.cc Load")
}
`;

const p = writeSrc("v8-diagnostics", "unwinder.rs", rust);
console.log(`unwinder: 自由函数 1 → ${p}`);
