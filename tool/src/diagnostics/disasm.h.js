#!/usr/bin/env bun
// tool/src/diagnostics/disasm.h.js
// 机械提取 v8/src/diagnostics/disasm.h: NameConverter/Disassembler 接口骨架
// → v8-diagnostics/src/disasm.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/diagnostics/disasm.h");

// 对数: 接口成员逐个在场
const converterMethods = ["NameOfCPURegister", "NameOfByteCPURegister", "NameOfXMMRegister",
  "NameOfAddress", "NameOfConstant", "NameInCode", "RootRelativeName"];
for (const mth of converterMethods) {
  if (!src.includes(`virtual const char* ${mth}(`) && !src.includes(`virtual const char* ${mth}`) &&
      !src.includes(`${mth}(`)) throw new Error(`缺 NameConverter 方法 ${mth}`);
}
for (const e of ["kContinueOnUnimplementedOpcode", "kAbortOnUnimplementedOpcode"]) {
  if (!src.includes(e)) throw new Error(`缺 UnimplementedOpcodeAction ${e}`);
}
for (const f of ["InstructionDecode", "ConstantPoolSizeAt", "Disassemble",
  "hit_unimplemented_opcode", "clear_hit_unimplemented_opcode"]) {
  if (!src.includes(f)) throw new Error(`缺 Disassembler 成员 ${f}`);
}
// tmp_buffer_ 128 字节
if (!src.includes("std::array<char, 128> tmp_buffer_;")) throw new Error("缺 tmp_buffer_ 128");

let rust = banner(
  "src/diagnostics/disasm.h.js",
  "src/diagnostics/disasm.h",
  "disasm 接口: NameConverter/Disassembler 骨架",
);

rust += `
/// 原版 namespace disasm::NameConverter: 地址/寄存器号 → 文本 (默认实现随架构)
pub trait NameConverter {
  /// 原版 NameOfCPURegister
  fn name_of_cpu_register(&self, reg: i32) -> String;
  /// 原版 NameOfByteCPURegister (ia32)
  fn name_of_byte_cpu_register(&self, reg: i32) -> String;
  /// 原版 NameOfXMMRegister (x64)
  fn name_of_xmm_register(&self, reg: i32) -> String;
  /// 原版 NameOfAddress
  fn name_of_address(&self, addr: u64) -> String;
  /// 原版 NameOfConstant
  fn name_of_constant(&self, addr: u64) -> String;
  /// 原版 NameInCode (内嵌名字表)
  fn name_in_code(&self, addr: u64) -> String;
  /// 原版 RootRelativeName: 根寄存器相对偏移名 (未命中返回 None)
  fn root_relative_name(&self, _offset: i32) -> Option<String> {
    None // 原版默认 nullptr
  }
}

/// 原版 tmp_buffer_ 容量
pub const TMP_BUFFER_SIZE: usize = 128;

/// 原版 enum UnimplementedOpcodeAction : int8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum UnimplementedOpcodeAction {
  kContinueOnUnimplementedOpcode = 0,
  kAbortOnUnimplementedOpcode = 1,
}

/// 原版 class Disassembler: 通用反汇编接口 (实现随架构)
pub struct Disassembler<'a> {
  /// 原版 converter_
  converter_: &'a dyn NameConverter,
  /// 原版 unimplemented_opcode_action_
  unimplemented_opcode_action_: UnimplementedOpcodeAction,
  /// 原版 hit_unimplemented_opcode_ (仅 x64 实现)
  hit_unimplemented_opcode_: bool,
}

impl<'a> Disassembler<'a> {
  /// 原版构造 (默认 kAbortOnUnimplementedOpcode)
  #[must_use]
  pub fn new(
    converter: &'a dyn NameConverter,
    unimplemented_opcode_action: UnimplementedOpcodeAction,
  ) -> Self {
    Self {
      converter_: converter,
      unimplemented_opcode_action_: unimplemented_opcode_action,
      hit_unimplemented_opcode_: false,
    }
  }

  /// 原版 unimplemented_opcode_action()
  #[inline]
  #[must_use]
  pub const fn unimplemented_opcode_action(&self) -> UnimplementedOpcodeAction {
    self.unimplemented_opcode_action_
  }

  /// 原版 hit_unimplemented_opcode(): 仅 x64 实现, 其余架构恒 false
  #[inline]
  #[must_use]
  pub const fn hit_unimplemented_opcode(&self) -> bool {
    self.hit_unimplemented_opcode_
  }

  /// 原版 clear_hit_unimplemented_opcode()
  #[inline]
  pub const fn clear_hit_unimplemented_opcode(&mut self) {
    self.hit_unimplemented_opcode_ = false;
  }

  /// 原版 InstructionDecode(buffer, instruction): 反汇编一条, 返回指令字节长
  /// (平台实现见 disasm-<arch>.cc — x64 → v8-codegen-x64, arm64 → v8-codegen-arm64)
  pub fn instruction_decode(&self, _buffer: &mut [u8], _instruction: *const u8) -> i32 {
    todo!("待 Developer 修订: 平台 InstructionDecode 接线")
  }

  /// 原版 ConstantPoolSizeAt(instruction): 非池首返回 -1, 否则条目数
  pub fn constant_pool_size_at(&self, _instruction: *const u8) -> i32 {
    todo!("待 Developer 修订")
  }

  /// 原版静态 Disassemble(f, begin, end, action) — 骨架
  pub fn disassemble(
    _f: *mut (),
    _begin: *const u8,
    _end: *const u8,
    _action: UnimplementedOpcodeAction,
  ) {
    todo!("待 Developer 修订")
  }
}
`;

const p = writeSrc("v8-diagnostics", "disasm.rs", rust);
console.log(`disasm.h: NameConverter ${converterMethods.length} / 枚举 2 / Disassembler 5 → ${p}`);
