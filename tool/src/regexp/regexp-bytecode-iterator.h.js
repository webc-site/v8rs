#!/usr/bin/env bun
// tool/src/regexp/regexp-bytecode-iterator.h.js
// 机械提取 v8/src/regexp/regexp-bytecode-iterator.{h,cc,-inl.h} 的迭代器骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/regexp/regexp-bytecode-iterator.h");
const inlSrc = readV8("src/regexp/regexp-bytecode-iterator-inl.h");

for (const anchor of ["done()", "advance()", "current_bytecode()", "current_size()", "current_offset()", "current_address()", "reset()"]) {
  if (!hSrc.includes(anchor)) throw new Error(`迭代器接口缺 ${anchor}`);
}
if (!inlSrc.includes("ForEachBytecode")) throw new Error("inl.h 缺 ForEachBytecode");

console.log("regexp-bytecode-iterator.h: skeleton");

let rust = banner(
  "src/regexp/regexp-bytecode-iterator.h.js",
  "src/regexp/regexp-bytecode-iterator.{h,cc,-inl.h}",
  "字节码迭代器骨架 (语义面待 Developer 修订)",
);

rust += `
/// 字节码序列迭代器 (对应原版 BytecodeIterator)
/// 布局: [start_, end_) 区间内按 current_size() 逐条推进
pub struct BytecodeIterator<'a> {
  bytecode: &'a [u8],
  start: *const u8,
  end: *const u8,
  cursor: *const u8,
}

impl<'a> BytecodeIterator<'a> {
  /// 原版 BytecodeIterator(TrustedByteArray)
  pub fn new(bytecode: &'a [u8]) -> Self {
    unimplemented!("登记骨架")
  }

  /// 原版 BytecodeIterator(TrustedByteArray, offset)
  pub fn with_offset(bytecode: &'a [u8], _offset: u32) -> Self {
    unimplemented!("登记骨架")
  }

  /// 原版 done()
  pub fn done(&self) -> bool {
    unimplemented!("登记骨架: 见 regexp-bytecode-iterator-inl.h")
  }

  /// 原版 advance()
  pub fn advance(&mut self) {
    unimplemented!("登记骨架: 见 regexp-bytecode-iterator-inl.h")
  }

  /// 原版 current_bytecode()
  pub fn current_bytecode(&self) -> crate::bytecodes_generated::Bytecode {
    unimplemented!("登记骨架")
  }

  /// 原版 current_size()
  pub fn current_size(&self) -> u8 {
    unimplemented!("登记骨架")
  }

  /// 原版 current_offset()
  pub fn current_offset(&self) -> u32 {
    unimplemented!("登记骨架")
  }

  /// 原版 reset()
  pub fn reset(&mut self) {
    unimplemented!("登记骨架")
  }
}
`;

const p = writeSrc("v8-regexp-compiler", "bytecode_iterator_skel.rs", rust);
console.log(`written ${p}`);
