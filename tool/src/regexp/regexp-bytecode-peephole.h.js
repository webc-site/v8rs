#!/usr/bin/env bun
// tool/src/regexp/regexp-bytecode-peephole.h.js
// 机械提取 v8/src/regexp/regexp-bytecode-peephole.{h,cc} 的窥孔优化入口骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/regexp/regexp-bytecode-peephole.h");
const ccSrc = readV8("src/regexp/regexp-bytecode-peephole.cc");

if (!/class BytecodePeepholeOptimization : public AllStatic/.test(hSrc)) {
  throw new Error("未找到 BytecodePeepholeOptimization : AllStatic");
}
if (!ccSrc.includes("OptimizeBytecode")) throw new Error("cc 缺 OptimizeBytecode");

console.log("regexp-bytecode-peephole.h: skeleton");

let rust = banner(
  "src/regexp/regexp-bytecode-peephole.h.js",
  "src/regexp/regexp-bytecode-peephole.{h,cc}",
  "窥孔优化入口骨架 (预定义字节码序列合并; 语义面待 Developer 修订)",
);

rust += `
/// 字节码窥孔优化 (对应原版 BytecodePeepholeOptimization, AllStatic)
/// 将 BytecodeGenerator 产出的预定义序列合并为单条字节码
pub struct BytecodePeepholeOptimization;

impl BytecodePeepholeOptimization {
  /// 原版 OptimizeBytecode(Isolate*, Zone*, RegExpData*, BytecodeWriter* src_writer)
  /// 返回优化后的字节码
  pub fn optimize_bytecode(_src: &[u8]) -> Vec<u8> {
    unimplemented!("登记骨架: 实现见 v8/src/regexp/regexp-bytecode-peephole.cc")
  }
}
`;

const p = writeSrc("v8-regexp-compiler", "bytecode_peephole_skel.rs", rust);
console.log(`written ${p}`);
