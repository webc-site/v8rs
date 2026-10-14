#!/usr/bin/env bun
// tool/src/wasm/wasm-arguments.h.js
// 机械提取 v8/src/wasm/wasm-arguments.h 的 CWasmArgumentsPacker 常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-arguments.h");

// kMaxOnStackBuffer = 10 * kSystemPointerSize (x64/arm64 均为 8 字节指针)
const maxOnStackMatch = hSrc.match(/static const size_t kMaxOnStackBuffer = (\d+) \* i::kSystemPointerSize;/);
if (!maxOnStackMatch) throw new Error("未找到 kMaxOnStackBuffer");
const factor = parseInt(maxOnStackMatch[1], 10);
if (factor !== 10) throw new Error(`kMaxOnStackBuffer 系数不符: ${factor}`);
const kSystemPointerSize = 8; // x64+arm64 目标
const maxOnStackBuffer = factor * kSystemPointerSize;

// on_stack_buffer_ 数组宽应与常量一致
const bufferFieldMatch = hSrc.match(/uint8_t on_stack_buffer_\[kMaxOnStackBuffer\];/);
if (!bufferFieldMatch) throw new Error("未找到 on_stack_buffer_ 字段声明");

console.log(`wasm-arguments: kMaxOnStackBuffer=${maxOnStackBuffer}`);

let rust = banner(
  "src/wasm/wasm-arguments.h.js",
  "src/wasm/wasm-arguments.h",
  "CWasmEntry 栈参数打包器常量",
);

rust += `
/// 指针宽 (x64/arm64 目标, 原版 kSystemPointerSize)
pub const SYSTEM_POINTER_SIZE: usize = ${kSystemPointerSize};

/// 栈上缓冲最大字节数 (原版 CWasmArgumentsPacker::kMaxOnStackBuffer =
/// 10 * kSystemPointerSize); 超过则转堆缓冲
pub const MAX_ON_STACK_BUFFER: usize = ${maxOnStackBuffer};

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_ON_STACK_BUFFER == 80);
  assert!(SYSTEM_POINTER_SIZE == 8);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_arguments.rs", rust);
console.log(`written ${p}`);
