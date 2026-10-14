// 由 tool/src/torque/misc.js 从 v8/src/torque/kythe-data.h 生成; 修订就地改, 复跑覆盖
// Kythe 索引数据 (consumer trait + 数据收集)

use crate::{
  error::{Error, Result},
  source_positions::SourcePosition,
};

/// 原版 kythe_entity_t = uint64_t
pub type KytheEntity = u64;

/// 原版 KythePosition 1:1
#[derive(Debug, Clone, Default)]
pub struct KythePosition {
  pub file_path: String,
  pub start_offset: u64,
  pub end_offset: u64,
}

/// 原版 KytheConsumer::Kind 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KytheKind {
  Unspecified,
  Constant,
  Function,
  ClassField,
  Variable,
  Type,
}

/// 原版 KytheConsumer (纯虚基类 → trait)
pub trait KytheConsumer {
  /// 原版 AddDefinition(kind, name, pos) — 默认返回 0
  fn add_definition(&mut self, kind: KytheKind, name: &str, pos: &KythePosition) -> KytheEntity {
    let _ = (kind, name, pos);
    0
  }

  /// 原版 AddDefinition(kind, name, pos, is_extern, readable_name) 重载 — 默认转发
  fn add_definition_ext(
    &mut self,
    kind: KytheKind,
    name: &str,
    pos: &KythePosition,
    is_extern: bool,
    readable_name: &str,
  ) -> KytheEntity {
    let _ = (is_extern, readable_name);
    self.add_definition(kind, name, pos)
  }

  /// 原版 AddUse
  fn add_use(&mut self, kind: KytheKind, entity: KytheEntity, use_pos: &KythePosition);

  /// 原版 AddCall
  fn add_call(
    &mut self,
    kind: KytheKind,
    caller_entity: KytheEntity,
    call_pos: &KythePosition,
    callee_entity: KytheEntity,
  );
}

/// 原版 KytheData (ContextualClass → 结构体; consumer 单持有)
#[derive(Default)]
pub struct KytheData {
  consumer: Option<Box<dyn KytheConsumer>>,
}

impl KytheData {
  /// 原版 SetConsumer
  pub fn set_consumer(&mut self, consumer: Box<dyn KytheConsumer>) {
    self.consumer = Some(consumer);
  }

  /// 原版 KytheData::AddDefinition 系列 — 转发 consumer
  pub fn add_definition(
    &mut self,
    kind: KytheKind,
    name: &str,
    pos: &KythePosition,
  ) -> KytheEntity {
    // TODO(填体): 原版无 consumer 时静默; 位置换算 (SourcePosition → KythePosition)
    self
      .consumer
      .as_mut()
      .map_or(0, |c| c.add_definition(kind, name, pos))
  }

  pub fn add_use(&mut self, kind: KytheKind, entity: KytheEntity, use_pos: &KythePosition) {
    // TODO(填体)
    if let Some(c) = self.consumer.as_mut() {
      c.add_use(kind, entity, use_pos);
    }
  }

  pub fn add_call(
    &mut self,
    kind: KytheKind,
    caller: KytheEntity,
    pos: &KythePosition,
    callee: KytheEntity,
  ) {
    // TODO(填体)
    if let Some(c) = self.consumer.as_mut() {
      c.add_call(kind, caller, pos, callee);
    }
  }

  /// 原版 GetPosition: SourcePosition → KythePosition
  pub fn get_position(_pos: SourcePosition) -> Result<KythePosition> {
    // TODO(填体): SourceFileMap 路径 + 偏移换算
    Err(Error::Unimplemented)
  }
}

// ── 原版 src/torque/kythe-data.cc ──
// 签名清单
// L13: KythePosition MakeKythePosition(const SourcePosition& pos)
// L28: kythe_entity_t KytheData::AddConstantDefinition(const Value* constant)
// L43: void KytheData::AddConstantUse(SourcePosition use_position, const Value* constant)
// L53: kythe_entity_t KytheData::AddFunctionDefinition(Callable* callable)
// L69: void KytheData::AddCall(Callable* caller, SourcePosition call_position, Callable* callee)
// L86: kythe_entity_t KytheData::AddClassFieldDefinition(const Field* field)
// L100: void KytheData::AddClassFieldUse(SourcePosition use_position, const Field* field)
// L111: kythe_entity_t KytheData::AddBindingDefinition(Binding<LocalValue>* binding)
// L118: kythe_entity_t KytheData::AddBindingDefinition(Binding<LocalLabel>* binding)
// L125: kythe_entity_t KytheData::AddBindingDefinitionImpl( uint64_t binding_index, const std::string& name, const SourcePosition& ident_pos)
// L139: void KytheData::AddBindingUse(SourcePosition use_position, Binding<LocalValue>* binding)
// L149: void KytheData::AddBindingUse(SourcePosition use_position, Binding<LocalLabel>* binding)
// L160: kythe_entity_t KytheData::AddTypeDefinition(const Declarable* type_decl)
// L174: void KytheData::AddTypeUse(SourcePosition use_position, const Declarable* type_decl)
