#!/usr/bin/env bun
// v8/src/compiler/escape-analysis.cc + escape-analysis.h
// → v8-compiler-passes/src/escape_analysis.rs.skel
//
// 规则面: State 枚举 (4 变体) + kNumStates、Variable (kInvalid=-1)、
// Dependable/VirtualObject/EscapeAnalysisResult/EffectGraphReducer/
// EscapeAnalysis 六类型签名; FieldAt 的对齐/越界/OOB 逃逸规则逐字核对。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/escape-analysis.h");
const cc = readV8("src/compiler/escape-analysis.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /enum class State : uint8_t \{ kUnvisited = 0, kRevisit, kOnStack, kVisited \};/, "State 枚举"],
  [h, /class Variable \{/, "Variable"],
  [h, /class Dependable : public ZoneObject \{/, "Dependable"],
  [h, /class VirtualObject : public Dependable \{/, "VirtualObject"],
  [h, /class EscapeAnalysisResult \{/, "EscapeAnalysisResult"],
  [h, /class V8_EXPORT_PRIVATE EscapeAnalysis final/, "EscapeAnalysis"],
  [h, /CHECK\(IsAligned\(offset, kTaggedSize\)\);/, "FieldAt 对齐检查"],
  [h, /Reading out-of-bounds can only happen in unreachable/, "OOB 逃逸注释"],
  [cc, /void EscapeAnalysis::Reduce\(Node\* node, Reduction\* reduction\) \{/, "Reduce 实现"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// State 变体序
const st = h.match(/enum class State : uint8_t \{([\s\S]*?)\};/)[1];
const states = [...st.matchAll(/k(\w+)/g)].map((m) => m[1]);
if (states.join() !== "Unvisited,Revisit,OnStack,Visited") throw new Error(`State 漂移: ${states}`);
if (!/const uint8_t kNumStates = static_cast<uint8_t>\(State::kVisited\) \+ 1;/.test(h)) throw new Error("kNumStates 缺失");
if (!/static const Id kInvalid = -1;/.test(h)) throw new Error("Variable kInvalid 缺失");

const body = `${banner("src/compiler/escape-analysis.js", "src/compiler/escape-analysis.h + src/compiler/escape-analysis.cc", "EscapeAnalysis — 标量替换前置的逃逸分析 (VirtualObject/VariableTracker/EffectGraphReducer)")}
//! EscapeAnalysis: 以 Variable 为抽象存储, 追踪分配对象字段, 判逃逸。
//!
//! 规则面: State 枚举 (${states.length}) + 六类型签名。
//! 语义面 (推迟): VariableTracker 状态机 (cc 中 900 行主逻辑)。

#![allow(dead_code)]

/// v8: src/compiler/escape-analysis.h State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
  /// kUnvisited = 0
  Unvisited = 0,
  /// kRevisit
  Revisit,
  /// kOnStack
  OnStack,
  /// kVisited
  Visited,
}

/// v8: src/compiler/escape-analysis.h kNumStates
pub const K_NUM_STATES: u8 = State::Visited as u8 + 1;

/// v8: src/compiler/escape-analysis.h Variable
/// 抽象存储位置 ( SSA 化由 VariableTracker 完成); kInvalid = -1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variable(i32);

impl Variable {
  /// v8: src/compiler/escape-analysis.h Variable::kInvalid
  pub const INVALID: Variable = Variable(-1);

  /// v8: src/compiler/escape-analysis.h Variable::Invalid
  pub const fn invalid() -> Self {
    Self::INVALID
  }
}

/// v8: src/compiler/escape-analysis.h Dependable
/// 值依赖登记: 变化时 Revisit 依赖节点
pub struct Dependable {
  /// 原 dependants_
  pub dependants: Vec<crate::node::Node>,
}

impl Dependable {
  /// v8: src/compiler/escape-analysis.h Dependable::AddDependency
  pub fn add_dependency(&mut self, node: crate::node::Node) {
    self.dependants.push(node);
  }
  /// v8: src/compiler/escape-analysis.h Dependable::RevisitDependants
  /// 原文: 逐节点 reducer->Revisit(node); 完后清空
  pub fn revisit_dependants(&mut self, reducer: &mut dyn crate::graph_reducer::Editor) {
    for node in self.dependants.drain(..) {
      reducer.revisit(node);
    }
  }
}

/// v8: src/compiler/escape-analysis.h VirtualObject
/// 虚拟对象: 分配点 + 字段 Variable 向量 + 逃逸位
pub struct VirtualObject {
  /// 原 escaped_ (默认 false)
  pub escaped: bool,
  /// 原 id_ (uint32)
  pub id: u32,
  /// 原 fields_ — kTaggedSize 粒度
  pub fields: Vec<Variable>,
}

impl VirtualObject {
  /// v8: src/compiler/escape-analysis.h VirtualObject::FieldAt
  /// 规则 (原文): offset 须 kTaggedSize 对齐 且未逃逸;
  /// offset >= size() → Nothing (不可达代码, 依赖者应将对象判逃逸);
  /// 否则 Just(fields_[offset / kTaggedSize])
  pub fn field_at(&self, offset: i32) -> Result<Variable, ()> {
    assert!(offset % crate::K_TAGGED_SIZE == 0);
    assert!(!self.has_escaped());
    if offset >= self.size() {
      return Err(()); // 原注释: 不可达代码, 避免死节点
    }
    Ok(self.fields[(offset / crate::K_TAGGED_SIZE) as usize])
  }
  /// v8: src/compiler/escape-analysis.h VirtualObject::FieldAt (Maybe 版)
  pub fn field_at_maybe(&self, maybe_offset: Option<i32>) -> Result<Variable, ()> {
    let Some(offset) = maybe_offset else { return Err(()) };
    self.field_at(offset)
  }

  /// v8: src/compiler/escape-analysis.h VirtualObject::id
  pub fn id(&self) -> u32 {
    self.id
  }
  /// v8: src/compiler/escape-analysis.h VirtualObject::size
  /// 原文: kTaggedSize * fields_.size()
  pub fn size(&self) -> i32 {
    (crate::K_TAGGED_SIZE * self.fields.len() as i32) as i32
  }
  /// v8: src/compiler/escape-analysis.h VirtualObject::SetEscaped
  pub fn set_escaped(&mut self) {
    self.escaped = true;
  }
  /// v8: src/compiler/escape-analysis.h VirtualObject::HasEscaped
  pub fn has_escaped(&self) -> bool {
    self.escaped
  }
}

/// v8: src/compiler/escape-analysis.h EscapeAnalysisResult
pub struct EscapeAnalysisResult {
  /// 原 tracker_
  pub tracker: *mut EscapeAnalysisTracker,
}

impl EscapeAnalysisResult {
  /// v8: src/compiler/escape-analysis.cc EscapeAnalysisResult::GetVirtualObject
  pub fn get_virtual_object(&self, node: &crate::node::Node) -> *const VirtualObject {
    todo!("tracker 查询")
  }
  /// v8: src/compiler/escape-analysis.cc EscapeAnalysisResult::GetVirtualObjectField
  pub fn get_virtual_object_field(&self, vobject: &VirtualObject, field: i32, effect: &crate::node::Node) -> crate::node::Node {
    todo!("字段 → 值节点")
  }
  /// v8: src/compiler/escape-analysis.cc EscapeAnalysisResult::GetReplacementOf
  pub fn get_replacement_of(&self, node: &crate::node::Node) -> crate::node::Node {
    todo!("替换映射查询")
  }
}

/// v8: src/compiler/escape-analysis.h EscapeAnalysis (EffectGraphReducer 派生)
pub struct EscapeAnalysis {
  /// 原 tracker_
  pub tracker: *mut EscapeAnalysisTracker,
  /// 原 jsgraph_
  pub jsgraph: *mut crate::js_graph::JSGraph,
}

impl EscapeAnalysis {
  /// v8: src/compiler/escape-analysis.cc EscapeAnalysis::Reduce
  pub fn reduce(&mut self, node: &crate::node::Node, reduction: &mut crate::graph_reducer::Reduction) {
    let _ = (node, reduction);
    todo!("按 opcode 分派 VariableTracker/EscapeAnalysisTracker")
  }
  /// v8: src/compiler/escape-analysis.h EscapeAnalysis::analysis_result
  /// 原文: DCHECK(Complete()) 后包装 tracker_
  pub fn analysis_result(&self) -> EscapeAnalysisResult {
    debug_assert!(self.complete());
    EscapeAnalysisResult { tracker: self.tracker }
  }
  /// v8: src/compiler/escape-analysis.h EffectGraphReducer::Complete
  /// 原文: stack_.empty() && revisit_.empty()
  fn complete(&self) -> bool {
    todo!("归约器队列状态")
  }
}

/// v8: src/compiler/escape-analysis.cc EscapeAnalysisTracker
/// (匿名命名空间内主数据结构, 字段推迟)
pub struct EscapeAnalysisTracker {
  pub _semantic: (),
}
`;

const p = writeSrc("v8-compiler-passes", "escape_analysis.rs.skel", body);
console.log(`written ${p}\nState=${states.length} 类型=6 FieldAt规则=3 (对齐/越界/索引)`);
