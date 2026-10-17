// 由 tool/src/codegen/code-desc.h.js 从 v8/src/codegen/code-desc.{h,cc} 生成; 修订就地改, 复跑覆盖
// CodeDesc 结构布局与签名骨架

use std::ptr::null_mut;

use crate::assembler_base::AssemblerBase;

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
      buffer: null_mut(),
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
      unwinding_info: null_mut(),
      unwinding_info_size: 0,
      origin: null_mut(),
    }
  }
}

// —— 签名骨架: 方法体待 Developer 修订 ——

/// CodeDesc 初始化参数偏移集
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CodeDescOffsets {
  pub safepoint_table_offset: i32,
  pub handler_table_offset: i32,
  pub constant_pool_offset: i32,
  pub code_comments_offset: i32,
  pub jump_table_info_offset: i32,
  pub reloc_info_offset: i32,
}

impl CodeDesc {
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// 原版 CodeDesc::Initialize (code-desc.cc:14): 由 Assembler 填充 desc。
  /// 布局: | instructions | data | free | reloc info |
  /// v8: src/codegen/code-desc.cc:15-45
  pub fn initialize(&mut self, assembler: &AssemblerBase, offsets: CodeDescOffsets) {
    self.buffer = assembler.buffer_start();
    self.buffer_size = assembler.buffer_capacity() as i32;
    self.instr_size = assembler.instruction_size() as i32;

    self.jump_table_info_offset = offsets.jump_table_info_offset;
    self.jump_table_info_size = self.instr_size - self.jump_table_info_offset;

    self.code_comments_offset = offsets.code_comments_offset;
    self.code_comments_size = self.jump_table_info_offset - self.code_comments_offset;

    self.constant_pool_offset = offsets.constant_pool_offset;
    self.constant_pool_size = self.code_comments_offset - self.constant_pool_offset;

    self.handler_table_offset = offsets.handler_table_offset;
    self.handler_table_size = self.constant_pool_offset - self.handler_table_offset;

    self.safepoint_table_offset = offsets.safepoint_table_offset;
    self.safepoint_table_size = self.handler_table_offset - self.safepoint_table_offset;

    self.reloc_offset = offsets.reloc_info_offset;
    self.reloc_size = self.buffer_size - self.reloc_offset;

    self.unwinding_info_size = 0;
    self.unwinding_info = null_mut();

    self.origin = assembler as *const AssemblerBase as *mut ();

    CodeDesc::verify(self);
  }

  /// 原版 CodeDesc::Verify (DEBUG 版校验; release 为空)
  /// v8: src/codegen/code-desc.cc:49-80
  pub fn verify(desc: &CodeDesc) {
    // 原版仅 DEBUG 构建启用 DCHECK 面
    if cfg!(debug_assertions) {
      // Zero-size code objects upset the system.
      debug_assert!(desc.instr_size > 0, "instr_size > 0");
      debug_assert!(!desc.buffer.is_null(), "buffer 非空");

      // Instruction area layout invariants.
      debug_assert!(desc.safepoint_table_size >= 0, "safepoint_table_size >= 0");
      debug_assert_eq!(
        desc.safepoint_table_size + desc.safepoint_table_offset,
        desc.handler_table_offset,
        "safepoint 表尾即 handler 表头"
      );
      debug_assert!(desc.handler_table_size >= 0, "handler_table_size >= 0");
      debug_assert_eq!(
        desc.handler_table_size + desc.handler_table_offset,
        desc.constant_pool_offset,
        "handler 表尾即常量池头"
      );
      debug_assert!(desc.constant_pool_size >= 0, "constant_pool_size >= 0");
      debug_assert_eq!(
        desc.constant_pool_size + desc.constant_pool_offset,
        desc.code_comments_offset,
        "常量池尾即注释头"
      );
      debug_assert!(desc.code_comments_size >= 0, "code_comments_size >= 0");
      debug_assert_eq!(
        desc.code_comments_size + desc.code_comments_offset,
        desc.jump_table_info_offset,
        "注释尾即跳转表头"
      );
      debug_assert!(desc.jump_table_info_size >= 0, "jump_table_info_size >= 0");
      debug_assert_eq!(
        desc.jump_table_info_size + desc.jump_table_info_offset,
        desc.instr_size,
        "跳转表尾即指令区末"
      );

      debug_assert!(desc.reloc_offset >= 0, "reloc_offset >= 0");
      debug_assert!(desc.reloc_size >= 0, "reloc_size >= 0");
      debug_assert!(desc.unwinding_info_size >= 0, "unwinding_info_size >= 0");
    }
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
