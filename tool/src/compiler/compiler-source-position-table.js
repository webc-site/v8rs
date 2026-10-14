#!/usr/bin/env bun
// v8/src/compiler/compiler-source-position-table.cc + compiler-source-position-table.h
// → v8-compiler-passes/src/compiler_source_position_table.rs.skel
//
// 规则面: Scope RAII 语义 (构造存 prev / Init 仅知位才覆盖 / 析构还原)、
// enable/disable 位、NodeAuxData 表字段; 方法签名 1:1。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/compiler-source-position-table.h");
const cc = readV8("src/compiler/compiler-source-position-table.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE SourcePositionTable final/, "SourcePositionTable"],
  [h, /class V8_NODISCARD Scope final \{/, "Scope RAII"],
  [h, /if \(position\.IsKnown\(\)\) source_positions_->current_position_ = position;/, "Init 语义"],
  [h, /NodeAuxData<SourcePosition, UnknownSourcePosition> table_;/, "NodeAuxData 表"],
  [h, /bool enabled_ = true;/, "enabled 默认"],
  [cc, /void SourcePositionTable::AddDecorator\(\) \{/, "AddDecorator"],
  [cc, /void SourcePositionTable::PrintJson\(std::ostream& os\) const \{/, "PrintJson"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// cc 方法面
const ccMethods = [...cc.matchAll(/^(?:void|SourcePosition|bool) SourcePositionTable::(\w+)\(/gm)].map((m) => m[1]);
if (ccMethods.join() !== "AddDecorator,RemoveDecorator,GetSourcePosition,GetSourcePosition,SetSourcePosition,PrintJson") {
  throw new Error(`cc 方法漂移: ${ccMethods}`);
}

const body = `${banner("src/compiler/compiler-source-position-table.js", "src/compiler/compiler-source-position-table.h + src/compiler/compiler-source-position-table.cc", "SourcePositionTable — 节点 id → 源位置映射 (Decorator 联动 + Scope RAII)")}
//! SourcePositionTable: 建/查/改节点源位置, current_position_ 随 Scope
//! RAII 压栈还原; Decorator 在图变时自动登记新节点位置。
//!
//! 规则面: Scope Init 语义 + enable 位 + 方法签名 6。
//! 语义面 (推迟): Decorator 红利 (node-added 回调), PrintJson 序列化。

#![allow(dead_code)]

/// v8: src/compiler/compiler-source-position-table.h SourcePositionTable::Scope
/// RAII: 构造存 prev_position_, Init 仅 Known 位才覆盖;
/// 析构还原 current_position_ (双构造: 直接位 / 取自节点)
pub struct SourcePositionScope<'a> {
  source_positions: &'a mut SourcePositionTable,
  /// 原 prev_position_
  prev_position: crate::source_position::SourcePosition,
}

impl Drop for SourcePositionScope<'_> {
  /// v8: src/compiler/compiler-source-position-table.h Scope::~Scope
  fn drop(&mut self) {
    self.source_positions.current_position = self.prev_position;
  }
}

impl<'a> SourcePositionScope<'a> {
  /// v8: src/compiler/compiler-source-position-table.h Scope::Init
  /// 原文: if (position.IsKnown()) current_position_ = position;
  fn init(&mut self, position: crate::source_position::SourcePosition) {
    if position.is_known() {
      self.source_positions.current_position = position;
    }
  }
}

/// v8: src/compiler/compiler-source-position-table.h SourcePositionTable
pub struct SourcePositionTable {
  /// 原 graph_
  pub graph: *mut crate::graph::TFGraph,
  /// 原 decorator_
  pub decorator: *mut Decorator,
  /// 原 current_position_
  pub current_position: crate::source_position::SourcePosition,
  /// 原 table_ — 节点 id → SourcePosition, 缺省 Unknown
  pub table: crate::node_aux_data::NodeAuxData<crate::source_position::SourcePosition>,
  /// 原 enabled_ (默认 true)
  pub enabled: bool,
}

impl SourcePositionTable {
  /// v8: src/compiler/compiler-source-position-table.cc SourcePositionTable::SourcePositionTable
  pub fn new(graph: *mut crate::graph::TFGraph) -> Self {
    todo!("NodeAuxData 以 UnknownSourcePosition 初始化")
  }

  /// v8: src/compiler/compiler-source-position-table.cc SourcePositionTable::AddDecorator
  pub fn add_decorator(&mut self) {
    todo!("Decorator 挂到 graph")
  }
  /// v8: src/compiler/compiler-source-position-table.cc SourcePositionTable::RemoveDecorator
  pub fn remove_decorator(&mut self) {
    todo!("Decorator 摘除")
  }

  /// v8: src/compiler/compiler-source-position-table.cc SourcePositionTable::GetSourcePosition (Node 版)
  pub fn get_source_position(&self, node: &crate::node::Node) -> crate::source_position::SourcePosition {
    todo!("table_.Find(node->id())")
  }
  /// v8: src/compiler/compiler-source-position-table.cc SourcePositionTable::GetSourcePosition (NodeId 版)
  pub fn get_source_position_by_id(&self, id: crate::node::NodeId) -> crate::source_position::SourcePosition {
    todo!("table_.Find(id)")
  }
  /// v8: src/compiler/compiler-source-position-table.cc SourcePositionTable::SetSourcePosition
  /// 原文: if (!IsEnabled()) return; 仅 Known 位才入表
  pub fn set_source_position(&mut self, node: &crate::node::Node, position: crate::source_position::SourcePosition) {
    if !self.enabled {
      return;
    }
    todo!("Known 过滤 + table_.Set(id, position)")
  }

  /// v8: src/compiler/compiler-source-position-table.h SourcePositionTable::SetCurrentPosition
  pub fn set_current_position(&mut self, pos: crate::source_position::SourcePosition) {
    self.current_position = pos;
  }
  /// v8: src/compiler/compiler-source-position-table.h SourcePositionTable::GetCurrentPosition
  pub fn get_current_position(&self) -> crate::source_position::SourcePosition {
    self.current_position
  }
  /// v8: src/compiler/compiler-source-position-table.h SourcePositionTable::Disable
  pub fn disable(&mut self) {
    self.enabled = false;
  }
  /// v8: src/compiler/compiler-source-position-table.h SourcePositionTable::Enable
  pub fn enable(&mut self) {
    self.enabled = true;
  }
  /// v8: src/compiler/compiler-source-position-table.h SourcePositionTable::IsEnabled
  pub fn is_enabled(&self) -> bool {
    self.enabled
  }

  /// v8: src/compiler/compiler-source-position-table.cc SourcePositionTable::PrintJson
  pub fn print_json(&self, os: &mut impl std::io::Write) {
    todo!("逐节点 JSON 序列化")
  }
}

/// v8: src/compiler/compiler-source-position-table.h SourcePositionTable::Decorator
/// 私有内嵌类: 节点新增时取 current_position_ 入表
pub struct Decorator {
  pub _semantic: (),
}
`;

const p = writeSrc("v8-compiler-passes", "compiler_source_position_table.rs.skel", body);
console.log(`written ${p}\ncc方法=${ccMethods.length} (GetSourcePosition 重载 2) RAII字段=2 enable默认=true`);
