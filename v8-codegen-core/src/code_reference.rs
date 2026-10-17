// 由 tool/src/codegen/code-reference.h.js 从 v8/src/codegen/code-reference.{h,cc} 生成; 修订就地改, 复跑覆盖
// CodeReference 判别联合 + 访问器签名骨架

use crate::code_desc::CodeDesc;

/// 原版 enum class Kind: NONE / CODE / WASM_CODE / CODE_DESC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Kind {
  None = 0,
  Code = 1,
  WasmCode = 2,
  CodeDesc = 3,
}

impl Kind {
  pub const NONE: Self = Self::None;
  pub const CODE: Self = Self::Code;
  pub const WASM_CODE: Self = Self::WasmCode;
  pub const CODE_DESC: Self = Self::CodeDesc;
}

/// 语义别名: 对齐原版测试名
pub type CodeReferenceKind = Kind;

/// 原版 CodeReference: 判别联合 (null_/wasm_code_/code_desc_/code_)
/// 指针族以 usize 承载 (地址语义), 生命周期由调用方保证。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeReference {
  kind_: Kind,
  /// union 载荷 (指针宽度)
  payload: usize,
}

impl Default for CodeReference {
  /// 原版默认构造: NONE + null
  fn default() -> Self {
    Self {
      kind_: Kind::None,
      payload: 0,
    }
  }
}

impl CodeReference {
  /// 读取种类判别
  #[inline]
  #[must_use]
  pub const fn kind(&self) -> Kind {
    self.kind_
  }

  /// 原版 CodeReference(wasm_code)
  #[must_use]
  pub const fn from_wasm_code(wasm_code: usize) -> Self {
    Self {
      kind_: Kind::WasmCode,
      payload: wasm_code,
    }
  }

  /// 原版 CodeReference(code_desc)
  #[must_use]
  pub const fn from_code_desc(code_desc: usize) -> Self {
    Self {
      kind_: Kind::CodeDesc,
      payload: code_desc,
    }
  }

  /// 从 CodeDesc 引用构造
  #[inline]
  #[must_use]
  pub fn from_code_desc_ref(code_desc: &CodeDesc) -> Self {
    Self::from_code_desc(code_desc as *const CodeDesc as usize)
  }

  /// 原版 CodeReference(code: Handle<Code>)
  #[must_use]
  pub const fn from_code(code: usize) -> Self {
    Self {
      kind_: Kind::Code,
      payload: code,
    }
  }

  /// 原版 is_null
  #[inline]
  #[must_use]
  pub const fn is_null(&self) -> bool {
    self.kind_ as u8 == Kind::None as u8
  }

  /// 原版 is_code
  #[inline]
  #[must_use]
  pub const fn is_code(&self) -> bool {
    self.kind_ as u8 == Kind::Code as u8
  }

  /// 原版 is_wasm_code
  #[inline]
  #[must_use]
  pub const fn is_wasm_code(&self) -> bool {
    self.kind_ as u8 == Kind::WasmCode as u8
  }

  // —— 访问器: CODE_DESC 分派 (CodeDescOps) 直读 desc 布局; CODE / WASM_CODE 依赖
  //    上层 Code / WasmCode 对象 (objects-rest / wasm), 按破环策略推迟登记 ——

  /// 读取 CODE_DESC 载荷指向的 CodeDesc。
  ///
  /// # Safety
  /// 构造契约: kind_ == CODE_DESC 时 payload 为指向存活 CodeDesc 的合法指针 (对应原版
  /// `const CodeDesc* code_desc_`)。
  #[inline]
  fn code_desc(&self) -> &CodeDesc {
    debug_assert!(self.kind_ == Kind::CodeDesc);
    // SAFETY: 契约保证 payload 为有效 &CodeDesc。
    unsafe { &*(self.payload as *const CodeDesc) }
  }

  /// 原版 code-reference.cc CodeReference::constant_pool (CodeDescOps 支)
  #[must_use]
  pub fn constant_pool(&self) -> usize {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => {
        let d = self.code_desc();
        (d.buffer as usize).wrapping_add(d.constant_pool_offset as usize)
      }
      Kind::Code | Kind::WasmCode | Kind::None => 0,
    }
  }

  /// 原版 code-reference.cc CodeReference::instruction_start (CodeDescOps 支)
  #[must_use]
  pub fn instruction_start(&self) -> usize {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => self.code_desc().buffer as usize,
      Kind::Code | Kind::WasmCode => self.payload,
      Kind::None => 0,
    }
  }

  /// 原版 code-reference.cc CodeReference::instruction_end (CodeDescOps 支)
  #[must_use]
  pub fn instruction_end(&self) -> usize {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => {
        let d = self.code_desc();
        (d.buffer as usize).wrapping_add(d.instr_size as usize)
      }
      Kind::Code | Kind::WasmCode => self.payload,
      Kind::None => 0,
    }
  }

  /// 原版 code-reference.cc CodeReference::instruction_size (CodeDescOps 支)
  #[must_use]
  pub fn instruction_size(&self) -> i32 {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => self.code_desc().instr_size,
      Kind::Code | Kind::WasmCode | Kind::None => 0,
    }
  }

  /// 原版 code-reference.cc CodeReference::relocation_start (CodeDescOps 支)
  #[must_use]
  pub fn relocation_start(&self) -> usize {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => {
        let d = self.code_desc();
        (d.buffer as usize).wrapping_add(d.reloc_offset as usize)
      }
      Kind::Code | Kind::WasmCode | Kind::None => 0,
    }
  }

  /// 原版 code-reference.cc CodeReference::relocation_end (CodeDescOps 支)
  #[must_use]
  pub fn relocation_end(&self) -> usize {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => {
        let d = self.code_desc();
        (d.buffer as usize).wrapping_add(d.buffer_size as usize)
      }
      Kind::Code | Kind::WasmCode | Kind::None => 0,
    }
  }

  /// 原版 code-reference.cc CodeReference::relocation_size (CodeDescOps 支)
  #[must_use]
  pub fn relocation_size(&self) -> i32 {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => self.code_desc().reloc_size,
      Kind::Code | Kind::WasmCode | Kind::None => 0,
    }
  }

  /// 原版 code-reference.cc CodeReference::code_comments (CodeDescOps 支)
  #[must_use]
  pub fn code_comments(&self) -> usize {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => {
        let d = self.code_desc();
        (d.buffer as usize).wrapping_add(d.code_comments_offset as usize)
      }
      Kind::Code | Kind::WasmCode | Kind::None => 0,
    }
  }

  /// 原版 code-reference.cc CodeReference::code_comments_size (CodeDescOps 支)
  #[must_use]
  pub fn code_comments_size(&self) -> i32 {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => self.code_desc().code_comments_size,
      Kind::Code | Kind::WasmCode | Kind::None => 0,
    }
  }

  /// 原版 code-reference.cc CodeReference::jump_table_info (CodeDescOps 支)
  #[must_use]
  pub fn jump_table_info(&self) -> usize {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => {
        let d = self.code_desc();
        (d.buffer as usize).wrapping_add(d.jump_table_info_offset as usize)
      }
      Kind::Code | Kind::WasmCode | Kind::None => 0,
    }
  }

  /// 原版 code-reference.cc CodeReference::jump_table_info_size (CodeDescOps 支)
  #[must_use]
  pub fn jump_table_info_size(&self) -> i32 {
    debug_assert!(!self.is_null());
    match self.kind_ {
      Kind::CodeDesc => self.code_desc().jump_table_info_size,
      Kind::Code | Kind::WasmCode | Kind::None => 0,
    }
  }

  /// 原版 as_code: 判别须为 CODE
  #[must_use]
  pub const fn as_code(&self) -> usize {
    debug_assert!(self.kind_ as u8 == Kind::Code as u8);
    self.payload
  }

  /// 原版 as_wasm_code: 判别须为 WASM_CODE
  #[must_use]
  pub const fn as_wasm_code(&self) -> usize {
    debug_assert!(self.kind_ as u8 == Kind::WasmCode as u8);
    self.payload
  }
}
