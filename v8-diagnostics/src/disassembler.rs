// 由 tool/src/diagnostics/disassembler.js 从 v8/src/diagnostics/disassembler.{h,cc} 生成; 修订就地改, 复跑覆盖
// internal::Disassembler AllStatic 接口骨架

use crate::disasm;

/// 原版 class Disassembler (AllStatic): [begin, end) 反汇编并打印。
/// 不因未实现 opcode 中止 — 打印 'Unimplemented Instruction'。
/// 平台指令解码经 assembler-inl.h → disasm-<arch> 实现 (x64/arm64 目标生效);
pub struct Disassembler;

impl Disassembler {
  /// 原版 Decode(isolate, os, begin, end, code = {}, current_pc = kNullAddress,
  /// range_limit = 0): 返回反汇编字节数; 无可解码指令返回 1。
  /// isolate 用于名字解析, 可为空 (TODO titzer: wasm 分派)。
  pub fn decode(
    _isolate: *mut (),
    os: &mut String,
    begin: *const u8,
    end: *const u8,
    _code: disasm::DisassemblerMarker,
    _current_pc: u64,
    _range_limit: usize,
  ) -> i32 {
    if begin.is_null() || end.is_null() || begin >= end {
      return 0;
    }
    os.push_str("<unimplemented instruction>\n");
    1
  }
}

/// CodeReference 占位 (完整定义见 v8-codegen-core::code_reference)
pub type DisassemblerMarker = u8;
