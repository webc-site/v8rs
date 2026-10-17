#!/usr/bin/env bun
// tool/src/diagnostics/disassembler.js
// 机械提取 v8/src/diagnostics/disassembler.{h,cc}: AllStatic Decode 接口 + cc 分派骨架
// → v8-diagnostics/src/disassembler.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/diagnostics/disassembler.h");
const ccSrc = readV8("src/diagnostics/disassembler.cc");

// 对数: 唯一静态方法 + cc 各架构分派锚点
if (!hSrc.includes("static int Decode(Isolate* isolate, std::ostream& os,")) throw new Error("缺 Decode 声明");
// 平台分派经 assembler-inl.h (arch 头) 传入; cc 自身以 ENABLE_DISASSEMBLER 双实现
if (!ccSrc.includes("#ifdef ENABLE_DISASSEMBLER") || !ccSrc.includes("#else  // ENABLE_DISASSEMBLER"))
  throw new Error("cc 缺 ENABLE_DISASSEMBLER 双实现");
const decodeDefs = [...ccSrc.matchAll(/^int Disassembler::Decode\(/gm)].length;
if (decodeDefs !== 2) throw new Error(`Decode 定义 ${decodeDefs} != 2`);

let rust = banner(
  "src/diagnostics/disassembler.js",
  "src/diagnostics/disassembler.{h,cc}",
  "internal::Disassembler AllStatic 接口骨架",
);

rust += `
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
    _os: &mut String,
    _begin: *const u8,
    _end: *const u8,
    _code: crate::disasm::DisassemblerMarker,
    _current_pc: u64,
    _range_limit: usize,
  ) -> i32 {
    todo!("待 Developer 修订: 对齐 disassembler.cc Decode 平台分派")
  }
}

/// CodeReference 占位 (完整定义见 v8-codegen-core::code_reference)
pub type DisassemblerMarker = u8;
`;

const p = writeSrc("v8-diagnostics", "disassembler.rs", rust);
console.log(`disassembler: 接口 1 / ENABLE_DISASSEMBLER 双实现 → ${p}`);
