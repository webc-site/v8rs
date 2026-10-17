// 由 tool/src/ast/modules.h.js 从 v8/src/ast/modules.h + include/v8-callbacks.h 生成; 修订就地改, 复跑覆盖
// 模块导入/导出元数据结构与枚举定义

use strum::{Display, FromRepr};

use crate::value::AstRawString;

/// 源码位置 (对应原版 Scanner::Location)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct ScannerLocation {
  pub beg_pos: i32,
  pub end_pos: i32,
}

impl ScannerLocation {
  pub const fn new(beg_pos: i32, end_pos: i32) -> Self {
    Self { beg_pos, end_pos }
  }
  pub const fn is_valid(&self) -> bool {
    self.beg_pos >= 0 && self.end_pos >= self.beg_pos
  }
}

/// 模块导入阶段 (原版 enum class ModuleImportPhase)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr, Default)]
#[repr(u8)]
pub enum ModuleImportPhase {
  Source = 0,
  Defer = 1,
  #[default]
  Evaluation = 2,
}

/// 槽位类别 (原版 enum CellIndexKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
#[repr(u8)]
pub enum CellIndexKind {
  Invalid = 0,
  Export = 1,
  Import = 2,
}

impl CellIndexKind {
  /// 从 cell_index 解码槽位类别
  pub const fn from_cell_index(cell_index: i32) -> Self {
    if cell_index > 0 {
      Self::Export
    } else if cell_index < 0 {
      Self::Import
    } else {
      Self::Invalid
    }
  }
}

/// 模块导入请求 (原版 AstModuleRequest)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstModuleRequest {
  pub specifier: AstRawString,
  pub phase: ModuleImportPhase,
  pub position: i32,
  pub index: i32,
}

impl AstModuleRequest {
  pub fn new(specifier: AstRawString, phase: ModuleImportPhase, position: i32, index: i32) -> Self {
    Self {
      specifier,
      phase,
      position,
      index,
    }
  }
}

/// 模块导入/导出项条目 (原版 SourceTextModuleDescriptor::Entry)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDescriptorEntry {
  pub location: ScannerLocation,
  pub export_name: Option<AstRawString>,
  pub local_name: Option<AstRawString>,
  pub import_name: Option<AstRawString>,
  pub module_request: i32,
  pub cell_index: i32,
}

impl ModuleDescriptorEntry {
  pub fn new(location: ScannerLocation) -> Self {
    Self {
      location,
      export_name: None,
      local_name: None,
      import_name: None,
      module_request: -1,
      cell_index: 0,
    }
  }

  pub fn cell_index_kind(&self) -> CellIndexKind {
    CellIndexKind::from_cell_index(self.cell_index)
  }
}

// ── 对数不变式断言 ──
const _: () = {
  assert!(ModuleImportPhase::Source as u8 == 0);
  assert!(ModuleImportPhase::Defer as u8 == 1);
  assert!(ModuleImportPhase::Evaluation as u8 == 2);
  assert!(CellIndexKind::Invalid as u8 == 0);
  assert!(CellIndexKind::Export as u8 == 1);
  assert!(CellIndexKind::Import as u8 == 2);
};
