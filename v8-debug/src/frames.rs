// 由 tool/src/debug/frames.js 生成
// 栈帧/求值/覆盖率/wasm 对象组

use strum::{Display, EnumIter, EnumString};

pub const K_NO_SOURCE_POSITION: i32 = -1;

/// 调试字节码重定向模式 (来源: debug-frames.h:71, 2 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum RedirectMode {
  #[strum(serialize = "kUseOriginalBytecode", serialize = "UseOriginalBytecode")]
  UseOriginalBytecode,
  #[strum(serialize = "kUseDebugBytecode", serialize = "UseDebugBytecode")]
  UseDebugBytecode,
}

impl RedirectMode {
  pub const fn is_original(&self) -> bool {
    matches!(self, Self::UseOriginalBytecode)
  }

  pub const fn is_debug(&self) -> bool {
    matches!(self, Self::UseDebugBytecode)
  }
}

/// 代码覆盖率块结构体 (来源: debug-coverage.h:21)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverageBlock {
  pub start: i32,
  pub end: i32,
  pub count: u32,
}

impl CoverageBlock {
  pub const fn new(start: i32, end: i32, count: u32) -> Self {
    Self { start, end, count }
  }
}

impl Default for CoverageBlock {
  fn default() -> Self {
    Self {
      start: K_NO_SOURCE_POSITION,
      end: K_NO_SOURCE_POSITION,
      count: 0,
    }
  }
}

/// 代码覆盖率函数结构体 (来源: debug-coverage.h:31)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CoverageFunction {
  pub start: i32,
  pub end: i32,
  pub count: u32,
  pub name: String,
  pub blocks: Vec<CoverageBlock>,
  pub has_block_coverage: bool,
}

impl CoverageFunction {
  pub fn new(
    start: i32,
    end: i32,
    count: u32,
    name: impl Into<String>,
    blocks: Vec<CoverageBlock>,
    has_block_coverage: bool,
  ) -> Self {
    Self {
      start,
      end,
      count,
      name: name.into(),
      blocks,
      has_block_coverage,
    }
  }
}

/// 脚本代码覆盖率信息 (来源: debug-coverage.h:44)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CoverageScript {
  pub script_id: i32,
  pub functions: Vec<CoverageFunction>,
}

impl CoverageScript {
  pub fn new(script_id: i32, functions: Vec<CoverageFunction>) -> Self {
    Self {
      script_id,
      functions,
    }
  }
}

/// 汇总覆盖率数据结构体 (来源: debug-coverage.h:53)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Coverage {
  pub scripts: Vec<CoverageScript>,
}

impl Coverage {
  pub fn new(scripts: Vec<CoverageScript>) -> Self {
    Self { scripts }
  }

  pub fn is_empty(&self) -> bool {
    self.scripts.is_empty()
  }

  pub fn len(&self) -> usize {
    self.scripts.len()
  }
}

/// Wasm 对象字段布局偏移常量 (来源: debug-wasm-objects.h:51-53)
pub struct WasmValueFieldOffsets;

impl WasmValueFieldOffsets {
  pub const K_TYPE_OFFSET: usize = 0;
  pub const K_VALUE_OFFSET: usize = 8;
  pub const K_SIZE: usize = 16;
}
