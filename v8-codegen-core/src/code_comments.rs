// 由 tool/src/codegen/code-comments.h.js 从 v8/src/codegen/code-comments.{h,cc} 生成; 修订就地改, 复跑覆盖
// 代码注释区布局 + 写入/迭代签名骨架

use std::{slice::from_raw_parts, str::from_utf8_unchecked};

use crate::{assembler_base::AssemblerBase, error::Result};

/// InstructionStream 注释区布局 (原版头注释):
/// | 4B: 区大小 uint32 (仅校验用) | 按 pc_offset 递增的 entry 数组 |
/// entry: | 4B pc_offset | 4B comment 长度(含 '\\0') | 变长注释(含 '\\0') |
pub const SECTION_SIZE_FIELD_BYTES: usize = 4;
pub const ENTRY_PC_OFFSET_BYTES: usize = 4;
pub const ENTRY_LENGTH_BYTES: usize = 4;

// v8: src/codegen/code-comments.cc (匿名 namespace) 偏移常量
/// 原版 kOffsetToFirstCommentEntry = kUInt32Size (跳过区大小头)
const K_OFFSET_TO_FIRST_COMMENT_ENTRY: usize = SECTION_SIZE_FIELD_BYTES;
/// 原版 kOffsetToPCOffset = 0
const K_OFFSET_TO_PC_OFFSET: usize = 0;
/// 原版 kOffsetToCommentSize = kOffsetToPCOffset + kUInt32Size
const K_OFFSET_TO_COMMENT_SIZE: usize = K_OFFSET_TO_PC_OFFSET + ENTRY_PC_OFFSET_BYTES;
/// 原版 kOffsetToCommentString = kOffsetToCommentSize + kUInt32Size
const K_OFFSET_TO_COMMENT_STRING: usize = K_OFFSET_TO_COMMENT_SIZE + ENTRY_LENGTH_BYTES;

/// 读取未对齐 u32 (原版 base::ReadUnalignedValue<uint32_t>)
///
/// # Safety
/// `ptr` 起 4 字节必须落在合法可读的注释区缓冲内 (调用方保证缓冲生命周期覆盖迭代器)。
#[inline]
unsafe fn read_unaligned_u32(ptr: *const u8) -> u32 {
  // SAFETY: 契约要求 ptr 起 4 字节可读; read_unaligned 无需对齐。
  unsafe { (ptr as *const u32).read_unaligned() }
}

/// 原版 CodeCommentEntry
#[derive(Debug, Clone, Default)]
pub struct CodeCommentEntry {
  /// 该注释所属 pc 偏移
  pub pc_offset: u32,
  /// 注释内容 (不含 \'\\0\'; 序列化时补)
  pub comment: String,
}

impl CodeCommentEntry {
  /// 原版 comment_length: 注释长 + 1 (终止 \'\\0\')
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
  /// 原版 code-comments.cc CodeCommentsWriter::Add: 追加注释 (pc_offset 递增序由调用方保证)
  pub fn add(&mut self, pc_offset: u32, comment: String) {
    let entry = CodeCommentEntry { pc_offset, comment };
    self.byte_count += entry.size();
    self.comments.push(entry);
  }

  /// 原版 code-comments.cc CodeCommentsWriter::Emit:
  /// dd 区大小 → 逐条 dd(pc_offset) dd(comment_length) db(注释字节) db('\\0')
  /// v8: src/codegen/code-comments.cc:66-77
  pub fn emit(&mut self, assm: &mut AssemblerBase) -> Result<()> {
    assm.emit_u32(self.section_size())?;
    for entry in &self.comments {
      assm.emit_u32(entry.pc_offset)?;
      assm.emit_u32(entry.comment_length())?;
      assm.emit_bytes(entry.comment.as_bytes())?;
      assm.emit_u8(0)?;
    }
    Ok(())
  }

  /// 原版 code-comments.cc CodeCommentsWriter::entry_count
  #[inline]
  #[must_use]
  pub fn entry_count(&self) -> usize {
    self.comments.len()
  }

  /// 原版 code-comments.cc CodeCommentsWriter::section_size = kOffsetToFirstCommentEntry + byte_count
  #[inline]
  #[must_use]
  pub const fn section_size(&self) -> u32 {
    K_OFFSET_TO_FIRST_COMMENT_ENTRY as u32 + self.byte_count
  }

  /// 将所有注释序列化为完整字节缓冲 (含 4B 区大小头)
  #[must_use]
  pub fn emit_to_vec(&self) -> Vec<u8> {
    let sz = self.section_size() as usize;
    let mut vec = Vec::with_capacity(sz);
    vec.extend_from_slice(&(self.section_size()).to_ne_bytes());
    for entry in &self.comments {
      vec.extend_from_slice(&entry.pc_offset.to_ne_bytes());
      vec.extend_from_slice(&entry.comment_length().to_ne_bytes());
      vec.extend_from_slice(entry.comment.as_bytes());
      vec.push(0);
    }
    vec
  }
}

/// 原版 CodeCommentsIterator (字段: code_comments_start_, code_comments_size_, current_entry_)
pub struct CodeCommentsIterator {
  code_comments_start_: *const u8,
  code_comments_size_: u32,
  current_entry_: *const u8,
}

impl CodeCommentsIterator {
  /// 原版 code-comments.cc CodeCommentsIterator::CodeCommentsIterator:
  /// current_entry_ 起点须跳过 4B 区大小头 (code_comments_start + kOffsetToFirstCommentEntry)
  ///
  /// # Safety
  /// 调用方须确保 code_comments_start 指向有效内存且在迭代器生命周期内保持存活。
  #[must_use]
  pub unsafe fn new(code_comments_start: *const u8, code_comments_size: u32) -> Self {
    debug_assert_ne!(code_comments_start as usize, 0, "kNullAddress");
    // 原版 DCHECK_IMPLIES(code_comments_size, size == ReadUnalignedValue<uint32_t>(start))
    debug_assert!(
      code_comments_size == 0
        || code_comments_size
          == unsafe {
            // SAFETY: 构造契约保证 start 起 4B 区大小头可读。
            read_unaligned_u32(code_comments_start)
          },
      "区大小头与传入 size 不一致"
    );
    Self {
      code_comments_start_: code_comments_start,
      code_comments_size_: code_comments_size,
      current_entry_: unsafe { code_comments_start.add(K_OFFSET_TO_FIRST_COMMENT_ENTRY) },
    }
  }

  /// 原版 size: 区大小
  #[inline]
  #[must_use]
  pub const fn size(&self) -> u32 {
    self.code_comments_size_
  }

  /// 原版 GetComment: 当前注释 (&'static 语义由缓冲生命周期保证)
  ///
  /// # Safety
  /// 依赖构造契约: code_comments_start 缓冲在本迭代器有效期内可读且注释为合法 UTF-8
  /// (由 CodeCommentsWriter 写入 std::string 保证)。
  #[must_use]
  pub fn get_comment(&self) -> &str {
    let comment_size = self.get_comment_size() as usize;
    // 原版 CHECK_EQ(GetCommentSize(), strlen(comment_string) + 1): 去掉终止 '\\0'
    debug_assert!(comment_size >= 1, "注释长度至少含终止符");
    let ptr = unsafe { self.current_entry_.add(K_OFFSET_TO_COMMENT_STRING) };
    unsafe {
      // SAFETY: ptr 起 comment_size-1 字节为 writer 写入的合法 UTF-8 (契约)。
      let bytes = from_raw_parts(ptr, comment_size - 1);
      from_utf8_unchecked(bytes)
    }
  }

  /// 原版 GetCommentSize = ReadUnalignedValue<uint32_t>(current + kOffsetToCommentSize)
  #[must_use]
  pub fn get_comment_size(&self) -> u32 {
    unsafe {
      // SAFETY: 构造契约保证 current_entry_ 处于合法 entry 内。
      read_unaligned_u32(self.current_entry_.add(K_OFFSET_TO_COMMENT_SIZE))
    }
  }

  /// 原版 GetPCOffset = ReadUnalignedValue<uint32_t>(current + kOffsetToPCOffset)
  #[must_use]
  pub fn get_pc_offset(&self) -> u32 {
    unsafe {
      // SAFETY: 构造契约保证 current_entry_ 处于合法 entry 内。
      read_unaligned_u32(self.current_entry_.add(K_OFFSET_TO_PC_OFFSET))
    }
  }

  /// 原版 Next: current_entry_ += kOffsetToCommentString + GetCommentSize()
  pub fn next(&mut self) {
    self.current_entry_ = unsafe {
      self
        .current_entry_
        .add(K_OFFSET_TO_COMMENT_STRING + self.get_comment_size() as usize)
    };
  }

  /// 原版 HasCurrent: current_entry_ < code_comments_start_ + size()
  #[must_use]
  pub fn has_current(&self) -> bool {
    (self.current_entry_ as usize) < (self.code_comments_start_ as usize) + self.size() as usize
  }
}
