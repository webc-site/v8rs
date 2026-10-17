// 由 tool/src/debug/synthetic-module.js 从 v8/src/objects/synthetic-module.{h,-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// 全族: Synthetic Module Record 布局 + 生命周期方法骨架
//! SyntheticModule: 嵌入器构造的合成模块 (WebIDL Synthetic Module Record; debug 域)。
//!
//! 布局 4 槽 + 模块生命周期 (SetExport/ResolveExport/Instantiate/Evaluate) 骨架。

use crate::{
  Error,
  module::{
    Cell, ExportEntry, ExportsTable, JsPromiseView, K_EVALUATED, K_EVALUATING, K_LINKED,
    ModuleBase, PromiseState, Slot, module_record_error,
  },
};

// ── 布局 (源码序; h TaggedMember 与 tq 形式类 1:1) ──
pub const SYNTHETIC_MODULE_FIELDS: [(&str, &str); 4] = [
  ("name", "String"),
  ("export_names", "FixedArray"),
  ("evaluation_steps", "Foreign"),
  ("host_defined_options", "Object"),
];

/// v8: src/objects/synthetic-module.cc SyntheticModule::SetExport
/// — 在 exports 表查找 export_name 对应 Cell; 未命中或不是 Cell 则抛 ReferenceError
/// (kModuleExportUndefined)。命中则设值。
/// 转交 [`crate::module::ExportsTable`].
pub fn synthetic_module_set_export(
  exports: &mut ExportsTable,
  export_name: &str,
  export_value: Slot,
) -> Result<bool, Error> {
  match exports.lookup_mut(export_name) {
    Some(ExportEntry::Cell(cell)) => {
      cell.value = export_value;
      Ok(true)
    }
    // 不是 Cell (间接表项或未命中) → 抛 ReferenceError
    Some(ExportEntry::Indirect(_)) | None => Err(Error::EvaluationFailed(format!(
      "Module export {} is not defined",
      export_name
    ))),
  }
}

/// v8: src/objects/synthetic-module.cc SyntheticModule::SetExportStrict
/// — 与 [`synthetic_module_set_export`] 同, 但原版为 CHECK (即必须为 Cell, 否则崩溃)。
/// 在 Rust 转写中, 非 Cell 也返回 Err。
pub fn synthetic_module_set_export_strict(
  exports: &mut ExportsTable,
  export_name: &str,
  export_value: Slot,
) -> Result<bool, Error> {
  synthetic_module_set_export(exports, export_name, export_value)
}

/// v8: src/objects/synthetic-module.cc SyntheticModule::ResolveExport
/// — 按 export_name 在 exports 中查找 Cell; 命中即 Some(Cell), 否则 None;
/// must_resolve 时非 Cell 视作 SyntaxError (原版 kUnresolvableExport)。
pub fn synthetic_module_resolve_export(
  exports: &ExportsTable,
  export_name: &str,
  must_resolve: bool,
  module_specifier: &str,
) -> Result<Option<Cell>, Error> {
  match exports.lookup(export_name) {
    Some(ExportEntry::Cell(cell)) => Ok(Some(*cell)),
    _ if !must_resolve => Ok(None),
    _ => Err(Error::EvaluationFailed(format!(
      "{} has no export named {}",
      module_specifier, export_name
    ))),
  }
}

/// v8: src/objects/synthetic-module.cc SyntheticModule::PrepareInstantiate
/// — 为 export_names 中每个名字创建初始 Cell(undefined), 并写入 exports 表。
/// 原版使用 Factory::NewCell (value = undefined)。
pub fn synthetic_module_prepare_instantiate(base: &mut ModuleBase, export_names: &[&str]) -> bool {
  for name in export_names {
    base.exports.insert(
      name,
      ExportEntry::Cell(Cell {
        value: Slot::Undefined,
      }),
    );
  }
  true
}

/// v8: src/objects/synthetic-module.cc SyntheticModule::FinishInstantiate
/// — SyntheticModule 无间接导出, 仅将 status 设为 kLinked。
/// namespace 校验由 [`crate::module`] 上层负责。
pub fn synthetic_module_finish_instantiate(base: &mut ModuleBase) -> bool {
  base.status = K_LINKED;
  true
}

/// v8: src/objects/synthetic-module.cc SyntheticModule::Evaluate
/// — 原版调用 evaluation_steps Foreign 回调 (embedder 注入), 状态流转 kEvaluating → kEvaluated;
/// 失败记录异常 (RecordError) 状态转 kErrored。
/// 本层以闭包参数注入 evaluation_steps; 返回 None 表示求值失败。
pub fn synthetic_module_evaluate<F>(base: &mut ModuleBase, evaluation_steps: F) -> Option<Slot>
where
  F: FnOnce(&mut ModuleBase) -> Result<Slot, Slot>,
{
  base.status = K_EVALUATING;
  match evaluation_steps(base) {
    Ok(capability) => {
      base.status = K_EVALUATED;
      base.top_level_capability = Some(JsPromiseView {
        status: PromiseState::Fulfilled,
        result: capability,
        has_handler: false,
      });
      Some(capability)
    }
    Err(err) => {
      module_record_error(base, err, true);
      None
    }
  }
}

/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(SYNTHETIC_MODULE_FIELDS.len() == 4);
  assert!(str_eq(SYNTHETIC_MODULE_FIELDS[0].0, "name"));
  assert!(str_eq(SYNTHETIC_MODULE_FIELDS[0].1, "String"));
  assert!(str_eq(SYNTHETIC_MODULE_FIELDS[2].1, "Foreign"));
  assert!(str_eq(SYNTHETIC_MODULE_FIELDS[3].1, "Object"));
};
