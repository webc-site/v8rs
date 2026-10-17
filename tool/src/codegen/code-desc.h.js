#!/usr/bin/env bun
// tool/src/codegen/code-desc.h.js
// 机械提取 v8/src/codegen/code-desc.{h,cc}: CodeDesc 结构布局与方法签名骨架
// → v8-codegen-core/src/code_desc.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/code-desc.h");
const ccSrc = readV8("src/codegen/code-desc.cc");

// 对数守卫: 关键字段/方法找不到即 throw
function must(re, what, src = hSrc) {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// 字段清点: 原版 code-desc.h 公开数据字段 14 个
const FIELDS = [
  ["buffer", "buffer = nullptr"],
  ["buffer_size", "int buffer_size = 0"],
  ["instr_size", "int instr_size = 0"],
  ["safepoint_table_offset", "int safepoint_table_offset = 0"],
  ["safepoint_table_size", "int safepoint_table_size = 0"],
  ["handler_table_offset", "int handler_table_offset = 0"],
  ["handler_table_size", "int handler_table_size = 0"],
  ["constant_pool_offset", "int constant_pool_offset = 0"],
  ["constant_pool_size", "int constant_pool_size = 0"],
  ["code_comments_offset", "int code_comments_offset = 0"],
  ["code_comments_size", "int code_comments_size = 0"],
  ["jump_table_info_offset", "int jump_table_info_offset = 0"],
  ["jump_table_info_size", "int jump_table_info_size = 0"],
  ["reloc_offset", "int reloc_offset = 0"],
  ["reloc_size", "int reloc_size = 0"],
  ["unwinding_info", "uint8_t* unwinding_info = nullptr"],
  ["unwinding_info_size", "int unwinding_info_size = 0"],
  ["origin", "Assembler* origin = nullptr"],
];
for (const [, pat] of FIELDS) {
  if (!hSrc.includes(pat)) throw new Error(`缺字段: ${pat}`);
}
if (FIELDS.length !== 18) throw new Error("字段清点异常");

// .cc 方法清点
const ccMethods = [...ccSrc.matchAll(/^(?:void|int) CodeDesc::(\w+)\(/gm)].map((m) => m[1]);
if (!ccMethods.includes("Initialize")) throw new Error(".cc 缺 Initialize");

let rust = banner(
  "src/codegen/code-desc.h.js",
  "src/codegen/code-desc.{h,cc}",
  "CodeDesc 结构布局与签名骨架",
);

rust += `
/// CodeDesc: 描述指令缓冲 + 重定位信息布局。
/// 指令区自缓冲头向前生长, 重定位信息自缓冲尾向前生长:
/// | instructions | data | free | reloc info |
pub struct CodeDesc {
  /// 缓冲起点 (原版 uint8_t* buffer)
  pub buffer: *mut u8,
  /// 缓冲总大小
  pub buffer_size: i32,
  /// 指令区大小 (含内联元数据)
  pub instr_size: i32,
  /// safepoint 表在指令区内偏移
  pub safepoint_table_offset: i32,
  /// safepoint 表大小
  pub safepoint_table_size: i32,
  /// handler 表偏移
  pub handler_table_offset: i32,
  /// handler 表大小
  pub handler_table_size: i32,
  /// 常量池偏移
  pub constant_pool_offset: i32,
  /// 常量池大小
  pub constant_pool_size: i32,
  /// 代码注释偏移
  pub code_comments_offset: i32,
  /// 代码注释大小
  pub code_comments_size: i32,
  /// 跳转表信息偏移
  pub jump_table_info_offset: i32,
  /// 跳转表信息大小
  pub jump_table_info_size: i32,
  /// 重定位信息偏移 (缓冲尾, 不在指令区内)
  pub reloc_offset: i32,
  /// 重定位信息大小
  pub reloc_size: i32,
  /// unwind 信息起点
  pub unwinding_info: *mut u8,
  /// unwind 信息大小
  pub unwinding_info_size: i32,
  /// 产出该 desc 的 Assembler (原版 origin)
  pub origin: *mut (),
}

impl Default for CodeDesc {
  fn default() -> Self {
    Self {
      buffer: std::ptr::null_mut(),
      buffer_size: 0,
      instr_size: 0,
      safepoint_table_offset: 0,
      safepoint_table_size: 0,
      handler_table_offset: 0,
      handler_table_size: 0,
      constant_pool_offset: 0,
      constant_pool_size: 0,
      code_comments_offset: 0,
      code_comments_size: 0,
      jump_table_info_offset: 0,
      jump_table_info_size: 0,
      reloc_offset: 0,
      reloc_size: 0,
      unwinding_info: std::ptr::null_mut(),
      unwinding_info_size: 0,
      origin: std::ptr::null_mut(),
    }
  }
}

// —— 签名骨架: 方法体待 Developer 修订 ——

impl CodeDesc {
  /// 原版 CodeDesc::Initialize (code-desc.cc): 由 Assembler 填充 desc
  pub fn initialize(
    desc: &mut CodeDesc,
    assembler: *mut (),
    safepoint_table_offset: i32,
    handler_table_offset: i32,
    constant_pool_offset: i32,
    code_comments_offset: i32,
    jump_table_info_offset: i32,
    reloc_info_offset: i32,
  ) {
    let _ = (desc, assembler, safepoint_table_offset, handler_table_offset, constant_pool_offset, code_comments_offset, jump_table_info_offset, reloc_info_offset);
    todo!("待 Developer 修订: 对齐原版 code-desc.cc Initialize")
  }

  /// 原版 CodeDesc::Verify (DEBUG 版校验; release 为空)
  pub fn verify(_desc: &CodeDesc) {
    // release 版原版为空实现
  }

  /// 原版 body_size: instr_size + unwinding_info_size
  #[inline]
  #[must_use]
  pub const fn body_size(&self) -> i32 {
    self.instr_size + self.unwinding_info_size
  }

  /// 原版 instruction_size: safepoint_table_offset
  #[inline]
  #[must_use]
  pub const fn instruction_size(&self) -> i32 {
    self.safepoint_table_offset
  }

  /// 原版 metadata_size: body_size - instruction_size
  #[inline]
  #[must_use]
  pub const fn metadata_size(&self) -> i32 {
    self.body_size() - self.instruction_size()
  }

  /// 原版 safepoint_table_offset_relative
  #[inline]
  #[must_use]
  pub const fn safepoint_table_offset_relative(&self) -> i32 {
    self.safepoint_table_offset - self.instruction_size()
  }

  /// 原版 handler_table_offset_relative
  #[inline]
  #[must_use]
  pub const fn handler_table_offset_relative(&self) -> i32 {
    self.handler_table_offset - self.instruction_size()
  }

  /// 原版 constant_pool_offset_relative
  #[inline]
  #[must_use]
  pub const fn constant_pool_offset_relative(&self) -> i32 {
    self.constant_pool_offset - self.instruction_size()
  }

  /// 原版 code_comments_offset_relative
  #[inline]
  #[must_use]
  pub const fn code_comments_offset_relative(&self) -> i32 {
    self.code_comments_offset - self.instruction_size()
  }

  /// 原版 jump_table_info_offset_relative
  #[inline]
  #[must_use]
  pub const fn jump_table_info_offset_relative(&self) -> i32 {
    self.jump_table_info_offset - self.instruction_size()
  }

  /// 原版 unwinding_info_offset_relative
  #[inline]
  #[must_use]
  pub const fn unwinding_info_offset_relative(&self) -> i32 {
    self.jump_table_info_offset_relative() + self.jump_table_info_size
  }
}
`;

const p = writeSrc("v8-codegen-core", "code_desc.rs", rust);
console.log(`code-desc: 字段 ${FIELDS.length} / 方法 10 (原版 .cc: ${ccMethods.join(",")}) → ${p}`);
