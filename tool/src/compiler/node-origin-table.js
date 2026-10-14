#!/usr/bin/env bun
// tool/src/compiler/node-origin-table.js
// v8/src/compiler/node-origin-table.h + node-origin-table.cc
// → v8-compiler-passes/src/node_origin_table.rs.skel
// 节点溯源表: 解析 OriginKind 枚举 / NodeOrigin 字段与 Unknown 哨兵 /
// NodeOriginTable 的 Get/SetNodeOrigin 重载族 / Scope-PhaseScope 装饰器。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/node-origin-table.h");
const cc = readV8("src/compiler/node-origin-table.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`node-origin-table: ${msg}`);
};

for (const [s, name] of [
  ["enum OriginKind { kWasmBytecode, kGraphNode, kJSBytecode };", "OriginKind 三值"],
  ["static NodeOrigin Unknown() { return NodeOrigin(); }", "Unknown 工厂"],
  ["created_from_(std::numeric_limits<int64_t>::min())", "Unknown 哨兵 INT64_MIN"],
  ["class V8_EXPORT_PRIVATE NodeOriginTable final", "NodeOriginTable"],
  ["class V8_NODISCARD Scope final {", "Scope RAII"],
  ["class V8_NODISCARD PhaseScope final {", "PhaseScope RAII"],
  ["class Decorator;", "Decorator 前向"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— OriginKind 枚举 ——
const kinds = [...h.matchAll(/k(WasmBytecode|GraphNode|JSBytecode)/g)].map((m) => m[1]);
need(new Set(kinds).size === 3, `OriginKind 成员 ${kinds}`);

// —— NodeOriginTable 公有方法 (头文件签名解析; 外层类 public 区 = 最后一个 \n private: 之前) ——
const tableBody = h.slice(h.indexOf("class V8_EXPORT_PRIVATE NodeOriginTable"));
const pub = tableBody.slice(0, tableBody.lastIndexOf("\n private:"));
const methods = [];
for (const m of pub.matchAll(/^  (?!\s)(?:(explicit|static|virtual) )?([\w:<>&* ]+?) (\w+)\(([^()]*)\)( const)?;/gm)) {
  if (m[1] === "static") continue; // 静态面单独记
  methods.push({ ret: m[2].trim(), name: m[3], args: m[4].replace(/\s+/g, " ").trim(), const: !!m[5] });
}
const getOrigin = methods.filter((mm) => mm.name === "GetNodeOrigin");
const setOrigin = methods.filter((mm) => mm.name === "SetNodeOrigin");
need(getOrigin.length === 2, `GetNodeOrigin 重载 ${getOrigin.length}`);
need(setOrigin.length === 4, `SetNodeOrigin 重载 ${setOrigin.length}`);

// —— cc 定义对数 ——
const ccDefs = [
  "void NodeOrigin::PrintJson(std::ostream& out) const",
  "class NodeOriginTable::Decorator final : public GraphDecorator",
  "NodeOriginTable::PhaseScope::PhaseScope(NodeOriginTable* origins,",
  "NodeOriginTable::PhaseScope::~PhaseScope()",
  "NodeOriginTable::NodeOriginTable(TFGraph* graph)",
  "NodeOriginTable::NodeOriginTable(Zone* zone)",
  "void NodeOriginTable::AddDecorator()",
  "void NodeOriginTable::RemoveDecorator()",
  "NodeOrigin NodeOriginTable::GetNodeOrigin(Node* node) const",
  "NodeOrigin NodeOriginTable::GetNodeOrigin(NodeId id) const",
  "void NodeOriginTable::SetNodeOrigin(Node* node, const NodeOrigin& no)",
  "void NodeOriginTable::SetNodeOrigin(NodeId id, NodeId origin)",
  "void NodeOriginTable::PrintJson(std::ostream& os) const",
];
for (const d of ccDefs) need(cc.includes(d), `cc 缺 ${d}`);

const body = `${banner("src/compiler/node-origin-table.js", "src/compiler/node-origin-table.h + node-origin-table.cc", "节点溯源: origin 记录/查询/JSON 输出")}
//! node-origin-table — 每节点 origin (phase/reducer/kind/created_from)
//!
//! NodeOrigin 未知哨兵: created_from = i64::MIN;
//! 表体 NodeAuxData<NodeOrigin> 按 NodeId 索引;
//! Scope/PhaseScope 是 RAII 换当前 origin / phase 名; Decorator 挂图装饰器
//! 在节点创建时盖 origin 章。

/// 原版 NodeOrigin::OriginKind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginKind {
  /// kWasmBytecode
  WasmBytecode,
  /// kGraphNode
  GraphNode,
  /// kJSBytecode
  JsBytecode,
}

/// 原版 NodeOrigin — 单条溯源记录
pub struct NodeOrigin {
  /// phase_name_
  pub phase_name: &'static str,
  /// reducer_name_
  pub reducer_name: &'static str,
  /// origin_kind_
  pub origin_kind: OriginKind,
  /// created_from_ (i64::MIN = Unknown)
  pub created_from: i64,
}

impl NodeOrigin {
  /// 原版 Unknown — created_from = i64::MIN, 名空串
  /// v8: src/compiler/node-origin-table.h NodeOrigin::Unknown
  pub const fn unknown() -> Self {
    Self { phase_name: "", reducer_name: "", origin_kind: OriginKind::GraphNode, created_from: i64::MIN }
  }

  /// 原版 IsKnown — created_from >= 0
  /// v8: src/compiler/node-origin-table.h NodeOrigin::IsKnown
  pub fn is_known(&self) -> bool {
    self.created_from >= 0
  }

  /// 原版 PrintJson
  /// v8: src/compiler/node-origin-table.cc NodeOrigin::PrintJson
  pub fn print_json(&self, out: &mut String) {
    todo!("v8: src/compiler/node-origin-table.cc NodeOrigin::PrintJson")
  }
}

/// 原版 NodeOriginTable — NodeId → NodeOrigin 辅助表
pub struct NodeOriginTable<'z> {
  /// graph_
  pub graph: *mut TFGraph,
  /// decorator_
  pub decorator: Option<Decorator>,
  /// current_origin_
  pub current_origin: NodeOrigin,
  /// current_bytecode_position_
  pub current_bytecode_position: i32,
  /// current_phase_name_
  pub current_phase_name: &'static str,
  /// previous_phase_name_
  pub previous_phase_name: &'static str,
  /// table_ — NodeAuxData<NodeOrigin, UnknownNodeOrigin>
  pub table: NodeAuxData<NodeOrigin>,
  /// zone (构造用)
  pub zone: &'z Zone<'z>,
}

impl NodeOriginTable<'_> {
  /// 原版 AddDecorator — 挂图装饰器
  /// v8: src/compiler/node-origin-table.cc NodeOriginTable::AddDecorator
  pub fn add_decorator(&mut self) {
    todo!("v8: src/compiler/node-origin-table.cc NodeOriginTable::AddDecorator")
  }

  /// 原版 RemoveDecorator
  /// v8: src/compiler/node-origin-table.cc NodeOriginTable::RemoveDecorator
  pub fn remove_decorator(&mut self) {
    todo!("v8: src/compiler/node-origin-table.cc NodeOriginTable::RemoveDecorator")
  }

  /// 原版 GetNodeOrigin(Node*) — 经 NodeId 查表
  /// v8: src/compiler/node-origin-table.cc NodeOriginTable::GetNodeOrigin
  pub fn get_node_origin(&self, node: &Node) -> NodeOrigin {
    todo!("v8: src/compiler/node-origin-table.cc NodeOriginTable::GetNodeOrigin")
  }

  /// 原版 GetNodeOrigin(NodeId) — ${getOrigin.length} 重载之一
  /// v8: src/compiler/node-origin-table.cc NodeOriginTable::GetNodeOrigin
  pub fn get_node_origin_by_id(&self, id: NodeId) -> NodeOrigin {
    todo!("v8: src/compiler/node-origin-table.cc NodeOriginTable::GetNodeOrigin")
  }

  /// 原版 SetNodeOrigin — ${setOrigin.length} 个重载 (node/no; id/origin; id/origin/phase; id/kind/origin)
  /// v8: src/compiler/node-origin-table.cc NodeOriginTable::SetNodeOrigin
  pub fn set_node_origin(&mut self, id: NodeId, no: NodeOrigin) {
    todo!("v8: src/compiler/node-origin-table.cc NodeOriginTable::SetNodeOrigin")
  }

  /// 原版 SetCurrentPosition
  /// v8: src/compiler/node-origin-table.h NodeOriginTable::SetCurrentPosition
  pub fn set_current_position(&mut self, no: NodeOrigin) {
    self.current_origin = no;
  }

  /// 原版 SetCurrentBytecodePosition / GetCurrentBytecodePosition
  /// v8: src/compiler/node-origin-table.h NodeOriginTable::SetCurrentBytecodePosition
  pub fn set_current_bytecode_position(&mut self, offset: i32) {
    self.current_bytecode_position = offset;
  }

  /// 原版 PrintJson — 全表 JSON 导出 (turbolizer)
  /// v8: src/compiler/node-origin-table.cc NodeOriginTable::PrintJson
  pub fn print_json(&self, os: &mut String) {
    todo!("v8: src/compiler/node-origin-table.cc NodeOriginTable::PrintJson")
  }
}

/// 原版 NodeOriginTable::Scope — RAII: reducer 区间内换 current_origin
pub struct OriginScope<'t> {
  /// origins_
  pub origins: Option<&'t mut NodeOriginTable<'t>>,
  /// prev_origin_
  pub prev_origin: NodeOrigin,
}

/// 原版 NodeOriginTable::PhaseScope — RAII: phase 区间内换 phase 名
pub struct OriginPhaseScope<'t> {
  /// origins_
  pub origins: &'t mut NodeOriginTable<'t>,
  /// prev_phase_name_
  pub prev_phase_name: &'static str,
}

/// 原版 NodeOriginTable::Decorator — 节点创建时盖 origin 章
/// v8: src/compiler/node-origin-table.cc NodeOriginTable::Decorator
pub struct Decorator;
`;

const p = writeSrc("v8-compiler-passes", "node_origin_table.rs.skel", body);
console.log(`written ${p}\norigin_kind=${new Set(kinds).size} get_overloads=${getOrigin.length} set_overloads=${setOrigin.length} pub_methods=${methods.length} cc_defs=${ccDefs.length}`);
