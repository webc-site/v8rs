#!/usr/bin/env bun
// tool/src/regexp/regexp-code-generator.h.js
// 机械提取 v8/src/regexp/regexp-code-generator.{h,cc} 的结果类别与入口骨架
// (字节码→机器码的模板编译面; Error 枚举与 visit 结构登记)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/regexp/regexp-code-generator.h");
const ccSrc = readV8("src/regexp/regexp-code-generator.cc");

if (!/class CodeGenerator final/.test(hSrc)) throw new Error("未找到 CodeGenerator");
// Error 无显式枚举声明, 源内用点: Error::kNone / Error::kUnsupportedBytecode
const errs = ["kNone", "kUnsupportedBytecode"];
if (!hSrc.includes("Error::kUnsupportedBytecode") || !hSrc.includes("Error::kNone")) {
  throw new Error("Error 用点集合变更, 需人工对数");
}
if (!ccSrc.includes("Assemble")) throw new Error("cc 缺 Assemble");

console.log(`regexp-code-generator.h: Error=${errs.length}`);

let rust = banner(
  "src/regexp/regexp-code-generator.h.js",
  "src/regexp/regexp-code-generator.{h,cc}",
  "字节码→机器码生成器结果类别与入口骨架",
);

rust += `
/// 生成器错误类别 (对应原版 CodeGenerator::Error)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CodeGenError {
`;
for (const [i, e] of errs.entries()) {
  rust += `  /// ${e}\n  ${e.replace(/^k/, "")} = ${i},\n`;
}
rust += `}

/// 字节码编译结果 (对应原版 CodeGenerator::Result; code 句柄语义待 Developer 接线)
pub struct CodeGenResult {
  pub error: CodeGenError,
}

impl CodeGenResult {
  /// 原版 Result::UnsupportedBytecode()
  pub fn unsupported_bytecode() -> Self {
    Self { error: CodeGenError::UnsupportedBytecode }
  }

  /// 原版 Succeeded()
  pub fn succeeded(&self) -> bool {
    self.error == CodeGenError::None
  }
}

/// 字节码编译器 (对应原版 CodeGenerator; visit 结构登记)
/// 原版流程: PreVisitBytecodes(为前跳分配 label) → VisitBytecodes → Visit<bc>
pub struct CodeGenerator;

impl CodeGenerator {
  /// 原版 Assemble(RegExpData*, Flags) -> Result
  pub fn assemble(&mut self) -> CodeGenResult {
    unimplemented!("登记骨架: 实现见 v8/src/regexp/regexp-code-generator.cc")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CodeGenError::None as u8 == 0);
};
`;

const p = writeSrc("v8-regexp-compiler", "code_generator_skel.rs", rust);
console.log(`written ${p}`);
