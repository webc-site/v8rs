// 由 tool/src/torque/misc.js 从 v8/src/torque/server-data.h 生成; 修订就地改, 复跑覆盖
// Language Server 数据 (定义跳转表)

use std::collections::BTreeMap;

use crate::{
  declarable::DeclarableId,
  global_context::GlobalContext,
  source_positions::{LineAndColumn, SourceId, SourcePosition},
  type_oracle::TypeOracle,
};

/// 原版 DefinitionsMap = map<SourcePosition, SourcePosition> → BTreeMap (确定性序)
pub type DefinitionsMap = BTreeMap<SourcePosition, SourcePosition>;

/// 原版 Symbols = vector<Declarable*>
pub type Symbols = Vec<DeclarableId>;

/// 原版 SymbolsMap = map<SourceId, Symbols>
pub type SymbolsMap = BTreeMap<SourceId, Symbols>;

/// 原版 LanguageServerData (ContextualClass → 结构体)
#[derive(Debug, Default)]
pub struct LanguageServerData {
  definitions_map: DefinitionsMap,
  symbols_map: SymbolsMap,
  global_context: Option<Box<GlobalContext>>,
  type_oracle: Option<Box<TypeOracle>>,
}

impl LanguageServerData {
  /// 原版 AddDefinition: token 位置 → 定义位置
  pub fn add_definition(&mut self, token: SourcePosition, definition: SourcePosition) {
    self.definitions_map.insert(token, definition);
  }

  /// 原版 FindDefinition: 同文件二分查找 token 所指定义
  #[must_use]
  pub fn find_definition(&self, source: SourceId, pos: LineAndColumn) -> Option<SourcePosition> {
    // TODO(填体): 原版按 (source, line, column) 上界二分
    let _ = (source, pos);
    None
  }

  /// 原版 SetGlobalContext (接管所有权 + 预处理符号表)
  pub fn set_global_context(&mut self, global_context: GlobalContext) {
    self.global_context = Some(Box::new(global_context));
    // TODO(填体): PrepareAllDeclarableSymbols (按 SourceId 拆分 + 过滤自动生成)
  }

  /// 原版 SetTypeOracle
  pub fn set_type_oracle(&mut self, type_oracle: TypeOracle) {
    self.type_oracle = Some(Box::new(type_oracle));
  }

  /// 原版 SymbolsForSourceId (原版 operator[] 默认空表 → 返回空切片, 登记偏差)
  #[must_use]
  pub fn symbols_for_source_id(&self, id: SourceId) -> &[DeclarableId] {
    self.symbols_map.get(&id).map_or(&[], |v| v.as_slice())
  }

  /// 原版 PrepareAllDeclarableSymbols (私有)
  pub fn prepare_all_declarable_symbols(&mut self) {
    // TODO(填体)
  }
}

// ── 原版 src/torque/server-data.cc ──
// 签名清单
// L17: void LanguageServerData::AddDefinition(SourcePosition token, SourcePosition definition)
// L22: std::optional<SourcePosition> LanguageServerData::FindDefinition( SourceId source, LineAndColumn pos)
// L37: void LanguageServerData::PrepareAllDeclarableSymbols()
