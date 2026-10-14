#!/usr/bin/env bun
// tool/src/codegen/code-comments.h.js
// 机械提取 v8/src/codegen/code-comments.{h,cc}: 注释区布局 + 结构 + 签名骨架
// → v8-codegen-core/src/code_comments.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/code-comments.h");
const ccSrc = readV8("src/codegen/code-comments.cc");

function must(re, what, src = hSrc) {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// 对数守卫: 布局常量 (原版 code-comments.cc 中 kNo... 无; 布局以 4 字节 size + entry 数组为准)
if (!hSrc.includes("struct CodeCommentEntry")) throw new Error("缺 CodeCommentEntry");
if (!hSrc.includes("class CodeCommentsWriter")) throw new Error("缺 CodeCommentsWriter");
if (!hSrc.includes("class V8_EXPORT_PRIVATE CodeCommentsIterator"))
  throw new Error("缺 CodeCommentsIterator");

// .cc 方法清点 (对数)
const ccWriterMethods = [...ccSrc.matchAll(/void CodeCommentsWriter::(\w+)\(/gm)].map((m) => m[1]);
const ccIterMethods = [...ccSrc.matchAll(/[\w:*&]+\s+CodeCommentsIterator::(\w+)\(/gm)].map((m) => m[1]);
for (const want of ["Add", "Emit"]) {
  if (!ccWriterMethods.includes(want)) throw new Error(`Writer 缺 ${want}`);
}
for (const want of ["size", "GetComment", "GetPCOffset", "Next", "HasCurrent"]) {
  if (!ccIterMethods.includes(want)) throw new Error(`Iterator 缺 ${want}`);
}

let rust = banner(
  "src/codegen/code-comments.h.js",
  "src/codegen/code-comments.{h,cc}",
  "代码注释区布局 + 写入/迭代签名骨架",
);

rust += `
/// InstructionStream 注释区布局 (原版头注释):
/// | 4B: 区大小 uint32 (仅校验用) | 按 pc_offset 递增的 entry 数组 |
/// entry: | 4B pc_offset | 4B comment 长度(含 \\'\\\\0\\') | 变长注释(含 \\'\\\\0\\') |
pub const SECTION_SIZE_FIELD_BYTES: usize = 4;
pub const ENTRY_PC_OFFSET_BYTES: usize = 4;
pub const ENTRY_LENGTH_BYTES: usize = 4;

/// 原版 CodeCommentEntry
#[derive(Debug, Clone, Default)]
pub struct CodeCommentEntry {
  /// 该注释所属 pc 偏移
  pub pc_offset: u32,
  /// 注释内容 (不含 \\'\\\\0\\'; 序列化时补)
  pub comment: String,
}

impl CodeCommentEntry {
  /// 原版 comment_length: 注释长 + 1 (终止 \\'\\\\0\\')
  #[inline]
  #[must_use]
  pub fn comment_length(&self) -> u32 {
    self.comment.len() as u32 + 1
  }

  /// 原版 size: pc(4) + len(4) + comment_length
  #[inline]
  #[must_use]
  pub fn size(&self) -> u32 {
    (ENTRY_PC_OFFSET_BYTES + ENTRY_LENGTH_BYTES) as u32 + self.comment_length()
  }
}

/// 原版 CodeCommentsWriter (字段: byte_count_, comments_)
#[derive(Debug, Clone, Default)]
pub struct CodeCommentsWriter {
  /// 区总字节数 (原版 byte_count_)
  byte_count: u32,
  /// 按 pc_offset 递增的注释表 (原版 comments_)
  comments: Vec<CodeCommentEntry>,
}

impl CodeCommentsWriter {
  /// 原版 Add: 追加注释 (pc_offset 递增序由调用方保证)
  pub fn add(&mut self, pc_offset: u32, comment: String) {
    let _ = (pc_offset, comment);
    todo!("待 Developer 修订: 对齐原版 code-comments.cc Add")
  }

  /// 原版 Emit: 写入 Assembler 注释区
  pub fn emit(&mut self, assm: *mut ()) {
    let _ = assm;
    todo!("待 Developer 修订: 对齐原版 code-comments.cc Emit")
  }

  /// 原版 entry_count
  #[inline]
  #[must_use]
  pub fn entry_count(&self) -> usize {
    self.comments.len()
  }

  /// 原版 section_size
  #[must_use]
  pub fn section_size(&self) -> u32 {
    todo!("待 Developer 修订: 对齐原版 code-comments.cc section_size")
  }
}

/// 原版 CodeCommentsIterator (字段: code_comments_start_, code_comments_size_, current_entry_)
pub struct CodeCommentsIterator {
  code_comments_start_: *const u8,
  code_comments_size_: u32,
  current_entry_: *const u8,
}

impl CodeCommentsIterator {
  /// 原版构造: (code_comments_start, code_comments_size)
  #[must_use]
  pub fn new(code_comments_start: *const u8, code_comments_size: u32) -> Self {
    Self {
      code_comments_start_: code_comments_start,
      code_comments_size_: code_comments_size,
      current_entry_: code_comments_start,
    }
  }

  /// 原版 size: 区大小
  #[inline]
  #[must_use]
  pub const fn size(&self) -> u32 {
    self.code_comments_size_
  }

  /// 原版 GetComment: 当前注释 (&'static 语义由缓冲生命周期保证)
  #[must_use]
  pub fn get_comment(&self) -> &str {
    todo!("待 Developer 修订: 对齐原版 code-comments.cc GetComment")
  }

  /// 原版 GetCommentSize
  #[must_use]
  pub fn get_comment_size(&self) -> u32 {
    todo!("待 Developer 修订: 对齐原版 code-comments.cc GetCommentSize")
  }

  /// 原版 GetPCOffset
  #[must_use]
  pub fn get_pc_offset(&self) -> u32 {
    todo!("待 Developer 修订: 对齐原版 code-comments.cc GetPCOffset")
  }

  /// 原版 Next: 前进到下一条
  pub fn next(&mut self) {
    todo!("待 Developer 修订: 对齐原版 code-comments.cc Next")
  }

  /// 原版 HasCurrent
  #[must_use]
  pub fn has_current(&self) -> bool {
    todo!("待 Developer 修订: 对齐原版 code-comments.cc HasCurrent")
  }
}
`;

const p = writeSrc("v8-codegen-core", "code_comments.rs", rust);
console.log(
  `code-comments: 布局常量 3 / Writer 方法 ${ccWriterMethods.length} / Iterator 方法 ${ccIterMethods.length} → ${p}`,
);
