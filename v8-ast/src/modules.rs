//! 模块结构与描述符 (v8/src/ast/modules.h/.cc)

pub use crate::modules_generated::{
  AstModuleRequest, CellIndexKind, ModuleDescriptorEntry, ModuleImportPhase, ScannerLocation,
};
use crate::value::AstRawString;

/// 源码模块描述符 (原版 SourceTextModuleDescriptor)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceTextModuleDescriptor {
  module_requests: Vec<AstModuleRequest>,
  special_exports: Vec<ModuleDescriptorEntry>,
  namespace_imports: Vec<ModuleDescriptorEntry>,
  regular_exports: Vec<ModuleDescriptorEntry>,
  regular_imports: Vec<ModuleDescriptorEntry>,
}

impl SourceTextModuleDescriptor {
  pub fn new() -> Self {
    Self::default()
  }

  /// 请求模块列表
  pub fn module_requests(&self) -> &[AstModuleRequest] {
    &self.module_requests
  }

  /// 特殊导出列表 (星号导出与显式间接导出)
  pub fn special_exports(&self) -> &[ModuleDescriptorEntry] {
    &self.special_exports
  }

  /// 命名空间导入列表
  pub fn namespace_imports(&self) -> &[ModuleDescriptorEntry] {
    &self.namespace_imports
  }

  /// 常规导出列表 (本地导出)
  pub fn regular_exports(&self) -> &[ModuleDescriptorEntry] {
    &self.regular_exports
  }

  /// 常规导入列表
  pub fn regular_imports(&self) -> &[ModuleDescriptorEntry] {
    &self.regular_imports
  }

  /// 添加模块请求并返回其在列表中的序号索引
  pub fn add_module_request(
    &mut self,
    specifier: AstRawString,
    phase: ModuleImportPhase,
    position: i32,
  ) -> i32 {
    if let Some(pos) = self
      .module_requests
      .iter()
      .position(|r| r.specifier == specifier && r.phase == phase)
    {
      pos as i32
    } else {
      let index = self.module_requests.len() as i32;
      self
        .module_requests
        .push(AstModuleRequest::new(specifier, phase, position, index));
      index
    }
  }

  /// 添加导入: import x from "foo.js"; import {x as y} from "foo.js"
  pub fn add_import(
    &mut self,
    import_name: AstRawString,
    local_name: AstRawString,
    specifier: AstRawString,
    import_phase: ModuleImportPhase,
    loc: ScannerLocation,
  ) -> bool {
    if self
      .regular_imports
      .iter()
      .any(|e| e.local_name.as_ref() == Some(&local_name))
    {
      return false;
    }
    let req = self.add_module_request(specifier, import_phase, loc.beg_pos);
    let mut entry = ModuleDescriptorEntry::new(loc);
    entry.import_name = Some(import_name);
    entry.local_name = Some(local_name);
    entry.module_request = req;
    self.regular_imports.push(entry);
    true
  }

  /// 添加星号导入: import * as x from "foo.js"
  pub fn add_star_import(
    &mut self,
    local_name: AstRawString,
    specifier: AstRawString,
    import_phase: ModuleImportPhase,
    loc: ScannerLocation,
  ) -> bool {
    if self
      .namespace_imports
      .iter()
      .any(|e| e.local_name.as_ref() == Some(&local_name))
    {
      return false;
    }
    let req = self.add_module_request(specifier, import_phase, loc.beg_pos);
    let mut entry = ModuleDescriptorEntry::new(loc);
    entry.local_name = Some(local_name);
    entry.module_request = req;
    self.namespace_imports.push(entry);
    true
  }

  /// 添加空导入: import "foo.js"
  pub fn add_empty_import(
    &mut self,
    specifier: AstRawString,
    import_phase: ModuleImportPhase,
    loc: ScannerLocation,
  ) {
    self.add_module_request(specifier, import_phase, loc.beg_pos);
  }

  /// 添加普通本地导出: export {x}; export {x as y}; export var x
  pub fn add_export(
    &mut self,
    local_name: AstRawString,
    export_name: AstRawString,
    loc: ScannerLocation,
  ) {
    let mut entry = ModuleDescriptorEntry::new(loc);
    entry.local_name = Some(local_name);
    entry.export_name = Some(export_name);
    entry.module_request = -1;
    self.regular_exports.push(entry);
  }

  /// 添加从外部重导出: export {x as y} from "foo.js"
  pub fn add_export_from(
    &mut self,
    export_name: AstRawString,
    import_name: AstRawString,
    specifier: AstRawString,
    import_phase: ModuleImportPhase,
    loc: ScannerLocation,
  ) {
    let req = self.add_module_request(specifier, import_phase, loc.beg_pos);
    let mut entry = ModuleDescriptorEntry::new(loc);
    entry.export_name = Some(export_name);
    entry.import_name = Some(import_name);
    entry.module_request = req;
    self.special_exports.push(entry);
  }

  /// 添加星号重导出: export * from "foo.js"
  pub fn add_star_export(
    &mut self,
    specifier: AstRawString,
    import_phase: ModuleImportPhase,
    loc: ScannerLocation,
  ) {
    let req = self.add_module_request(specifier, import_phase, loc.beg_pos);
    let mut entry = ModuleDescriptorEntry::new(loc);
    entry.module_request = req;
    self.special_exports.push(entry);
  }

  /// 将隐式间接导出转为显式间接导出 (原版 MakeIndirectExportsExplicit)
  ///
  /// 若 `export { b as c }` 中的 `b` 来自 `import { a as b } from "X"`,
  /// 则将其转为间接重导出并移至 `special_exports`。
  pub fn make_indirect_exports_explicit(&mut self) {
    let mut remaining_exports = Vec::with_capacity(self.regular_exports.len());
    for mut export_entry in self.regular_exports.drain(..) {
      if let Some(local_name) = &export_entry.local_name {
        if let Some(import_entry) = self
          .regular_imports
          .iter()
          .find(|imp| imp.local_name.as_ref() == Some(local_name))
        {
          export_entry.import_name = import_entry.import_name.clone();
          export_entry.module_request = import_entry.module_request;
          export_entry.location = import_entry.location;
          export_entry.local_name = None;
          self.special_exports.push(export_entry);
          continue;
        }
        if let Some(ns_import) = self
          .namespace_imports
          .iter()
          .find(|imp| imp.local_name.as_ref() == Some(local_name))
        {
          export_entry.import_name = None;
          export_entry.module_request = ns_import.module_request;
          export_entry.location = ns_import.location;
          export_entry.local_name = None;
          self.special_exports.push(export_entry);
          continue;
        }
      }
      remaining_exports.push(export_entry);
    }
    self.regular_exports = remaining_exports;
  }

  /// 分配槽位索引 (原版 AssignCellIndices)
  ///
  /// 常规导出: 按 local_name 排序分组, 分配 +1, +2, +3... (相同 local_name 共享相同槽位)
  /// 常规导入: 按 local_name 排序分配 -1, -2, -3...
  pub fn assign_cell_indices(&mut self) {
    self
      .regular_exports
      .sort_by(|a, b| a.local_name.cmp(&b.local_name));

    let mut export_index = 1;
    let mut i = 0;
    while i < self.regular_exports.len() {
      let current_key = self.regular_exports[i].local_name.clone();
      let assigned = export_index;
      while i < self.regular_exports.len() && self.regular_exports[i].local_name == current_key {
        self.regular_exports[i].cell_index = assigned;
        i += 1;
      }
      export_index += 1;
    }

    self
      .regular_imports
      .sort_by(|a, b| a.local_name.cmp(&b.local_name));

    let mut import_index = -1;
    for entry in &mut self.regular_imports {
      entry.cell_index = import_index;
      import_index -= 1;
    }
  }
}
