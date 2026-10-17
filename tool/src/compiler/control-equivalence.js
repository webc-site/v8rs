#!/usr/bin/env bun
// v8/src/compiler/control-equivalence.cc + control-equivalence.h
// → v8-compiler-passes/src/control_equivalence.rs.skel
//
// 规则面: DFSDirection 枚举、kInvalidClass 常量、Bracket/DFSStackEntry/
// NodeData 三结构 + 全方法签名 (Run/ClassOf/Visit*/RunUndirectedDFS/
// DetermineParticipation/GetData/AllocateData/New*Number/Participates/
// Get*/Set*); 初始值 (dfs 0, class 1) 逐字核对。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/control-equivalence.h");
const cc = readV8("src/compiler/control-equivalence.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE ControlEquivalence final/, "ControlEquivalence"],
  [h, /enum DFSDirection \{ kInputDirection, kUseDirection \};/, "DFSDirection"],
  [h, /static const size_t kInvalidClass = static_cast<size_t>\(-1\);/, "kInvalidClass"],
  [h, /struct Bracket \{/, "Bracket"],
  [h, /struct NodeData : ZoneObject \{/, "NodeData"],
  [h, /void RunUndirectedDFS\(Node\* exit\);/, "RunUndirectedDFS"],
  [cc, /void ControlEquivalence::Run\(Node\* exit\) \{/, "Run 实现"],
  [cc, /void ControlEquivalence::RunUndirectedDFS\(Node\* exit\) \{/, "DFS 实现"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// 初始值: dfs_number_(0), class_number_(1)
if (!/dfs_number_\(0\),\n\s*class_number_\(1\),/.test(h)) throw new Error("初始值漂移");
// NodeData 位标志: visited / on_stack
if (!/bool visited : 1;.*bool on_stack : 1;/s.test(h)) throw new Error("位标志缺失");

const body = `${banner("src/compiler/control-equivalence.js", "src/compiler/control-equivalence.h + src/compiler/control-equivalence.cc", "ControlEquivalence — 控制等价类 (JPP PLDI94 无向环等价算法)")}
//! ControlEquivalence: 双向 BFS 参与集 + 无向 DFS 支配树括号法,
//! 求控制依赖等价类 (SESE 区域 / PST)。
//!
//! 规则面: 枚举 + 常量 + 3 结构 + 方法签名 ${13}。
//! 语义面 (推迟): 括号计数的类号分配 (cc VisitPre/Mid/Post/Backedge)。

#![allow(dead_code)]

/// v8: src/compiler/control-equivalence.h ControlEquivalence::DFSDirection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DFSDirection {
  /// kInputDirection
  Input,
  /// kUseDirection
  Use,
}

/// v8: src/compiler/control-equivalence.h ControlEquivalence::kInvalidClass
pub const K_INVALID_CLASS: usize = usize::MAX;

/// v8: src/compiler/control-equivalence.h ControlEquivalence::Bracket
/// DFS 括号: 方向 + 缓存类号/大小 + 起止节点
pub struct Bracket {
  /// 原 direction — 加入时的行进方向
  pub direction: DFSDirection,
  /// 原 recent_class — 栈顶时的缓存类号
  pub recent_class: usize,
  /// 原 recent_size — 栈顶时的缓存集合大小
  pub recent_size: usize,
  /// 原 from / to
  pub from: crate::node::Node,
  pub to: crate::node::Node,
}

/// v8: src/compiler/control-equivalence.h ControlEquivalence::DFSStackEntry
/// 无向 DFS 栈项: 双向迭代器 + 父节点
pub struct DFSStackEntry {
  /// 原 direction
  pub direction: DFSDirection,
  /// 原 parent_node / node
  pub parent_node: crate::node::Node,
  pub node: crate::node::Node,
}

/// v8: src/compiler/control-equivalence.h ControlEquivalence::NodeData
/// 逐节点: 类号 (kInvalidClass 起) + 括号表 + visited/on_stack 位
pub struct NodeData {
  /// 原 class_number — 初始 kInvalidClass
  pub class_number: usize,
  /// 原 blist
  pub blist: std::collections::LinkedList<Bracket>,
  /// 原 visited : 1
  pub visited: bool,
  /// 原 on_stack : 1
  pub on_stack: bool,
}

/// v8: src/compiler/control-equivalence.h ControlEquivalence
pub struct ControlEquivalence {
  /// 原 zone_
  zone: *mut crate::zone::Zone,
  /// 原 graph_
  graph: *const crate::graph::TFGraph,
  /// 原 dfs_number_ — 初始 0
  dfs_number: i32,
  /// 原 class_number_ — 初始 1
  class_number: i32,
  /// 原 node_data_ — 按 node id 索引
  node_data: Vec<*mut NodeData>,
}

impl ControlEquivalence {
  /// v8: src/compiler/control-equivalence.h ControlEquivalence::ControlEquivalence
  pub fn new(zone: *mut crate::zone::Zone, graph: *const crate::graph::TFGraph) -> Self {
    Self { zone, graph, dfs_number: 0, class_number: 1, node_data: Vec::new() }
  }

  /// v8: src/compiler/control-equivalence.cc ControlEquivalence::Run
  /// 原文: DetermineParticipation(exit) → RunUndirectedDFS(exit)
  pub fn run(&mut self, exit: crate::node::Node) {
    self.determine_participation(exit);
    self.run_undirected_dfs(exit);
  }

  /// v8: src/compiler/control-equivalence.h ControlEquivalence::ClassOf
  /// 原文: DCHECK_NE(kInvalidClass, GetClass(node))
  pub fn class_of(&self, node: crate::node::Node) -> usize {
    let c = self.get_class(node);
    debug_assert_ne!(K_INVALID_CLASS, c);
    c
  }

  /// v8: src/compiler/control-equivalence.cc ControlEquivalence::VisitPre
  pub fn visit_pre(&mut self, node: crate::node::Node) {
    todo!("类号分配 + 括号表初始化")
  }
  /// v8: src/compiler/control-equivalence.cc ControlEquivalence::VisitMid
  pub fn visit_mid(&mut self, node: crate::node::Node, direction: DFSDirection) {
    todo!("方向切换处理")
  }
  /// v8: src/compiler/control-equivalence.cc ControlEquivalence::VisitPost
  pub fn visit_post(&mut self, node: crate::node::Node, parent_node: crate::node::Node, direction: DFSDirection) {
    todo!("capping bracket 类号均摊")
  }
  /// v8: src/compiler/control-equivalence.cc ControlEquivalence::VisitBackedge
  pub fn visit_backedge(&mut self, from: crate::node::Node, to: crate::node::Node, direction: DFSDirection) {
    todo!("回边括号压栈")
  }
  /// v8: src/compiler/control-equivalence.cc ControlEquivalence::RunUndirectedDFS
  pub fn run_undirected_dfs(&mut self, exit: crate::node::Node) {
    todo!("双方向迭代 DFS: Pre/Mid/Post/Backedge 驱动")
  }
  /// v8: src/compiler/control-equivalence.cc ControlEquivalence::DetermineParticipationEnqueue
  pub fn determine_participation_enqueue(&mut self, queue: &mut std::collections::VecDeque<crate::node::Node>, node: crate::node::Node) {
    todo!("参与标记 + 邻接入队")
  }
  /// v8: src/compiler/control-equivalence.cc ControlEquivalence::DetermineParticipation
  pub fn determine_participation(&mut self, exit: crate::node::Node) {
    todo!("exit 反向 BFS 参与集")
  }

  /// v8: src/compiler/control-equivalence.h ControlEquivalence::GetData
  pub fn get_data(&mut self, node: crate::node::Node) -> *mut NodeData {
    todo!("按 id 索引, 越界扩容")
  }
  /// v8: src/compiler/control-equivalence.h ControlEquivalence::AllocateData
  pub fn allocate_data(&mut self, node: crate::node::Node) {
    todo!("zone 分配 NodeData")
  }
  /// v8: src/compiler/control-equivalence.h ControlEquivalence::NewClassNumber
  pub fn new_class_number(&mut self) -> i32 {
    let n = self.class_number;
    self.class_number += 1;
    n
  }
  /// v8: src/compiler/control-equivalence.h ControlEquivalence::NewDFSNumber
  pub fn new_dfs_number(&mut self) -> i32 {
    let n = self.dfs_number;
    self.dfs_number += 1;
    n
  }
  /// v8: src/compiler/control-equivalence.h ControlEquivalence::Participates
  pub fn participates(&mut self, node: crate::node::Node) -> bool {
    !self.get_data(node).is_null()
  }
  /// v8: src/compiler/control-equivalence.h ControlEquivalence::GetClass
  pub fn get_class(&self, node: crate::node::Node) -> usize {
    todo!("node_data[id].class_number")
  }
}

// 方法面计: Run/ClassOf/VisitPre/VisitMid/VisitPost/VisitBackedge/
// RunUndirectedDFS/DetermineParticipationEnqueue/DetermineParticipation/
// GetData/AllocateData/NewClassNumber/NewDFSNumber = 13 (+Participates/GetClass/SetClass)
`;

const p = writeSrc("v8-compiler-passes", "control_equivalence.rs.skel", body);
console.log(`written ${p}\n枚举=1 常量=1 结构=3 方法面=15 初始值=(dfs 0, class 1)`);
