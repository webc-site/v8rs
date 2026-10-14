#!/usr/bin/env bun
// v8/src/compiler/memory-lowering.cc + memory-lowering.h
// → v8-compiler-passes/src/memory_lowering.rs.skel
//
// 规则面: AllocationFolding 枚举 (2)、AllocationState 三工厂
// (Empty/Closed/Open 语义)、公开 Reduce* 面 8 + ComputeWriteBarrierKind 锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/memory-lowering.h");
const cc = readV8("src/compiler/memory-lowering.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class MemoryLowering final : public Reducer \{/, "MemoryLowering"],
  [h, /enum class AllocationFolding \{ kDoAllocationFolding, kDontAllocationFolding \};/, "AllocationFolding"],
  [h, /static AllocationState const\* Empty\(Zone\* zone\) \{/, "Empty 工厂"],
  [h, /static AllocationState const\* Closed\(AllocationGroup\* group, Node\* effect,/, "Closed 工厂"],
  [h, /static AllocationState const\* Open\(AllocationGroup\* group, intptr_t size,/, "Open 工厂"],
  [h, /Reduction ReduceAllocateRaw\(Node\* node, AllocationType allocation_type,/, "ReduceAllocateRaw 双参"],
  [h, /WriteBarrierKind ComputeWriteBarrierKind\(Node\* node, Node\* object,/, "ComputeWriteBarrierKind"],
  [cc, /Reduction MemoryLowering::ReduceAllocateRaw\(/, "cc ReduceAllocateRaw"],
  [cc, /WriteBarrierKind MemoryLowering::ComputeWriteBarrierKind\(/, "cc WriteBarrier"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// cc 方法面
const ccMethods = [...cc.matchAll(/^(?:Reduction|WriteBarrierKind|Node\*|void) MemoryLowering::(\w+)\(/gm)].map((m) => m[1]);
if (ccMethods.length < 12) throw new Error(`cc 方法 ${ccMethods.length} < 12`);

const body = `${banner("src/compiler/memory-lowering.js", "src/compiler/memory-lowering.h + src/compiler/memory-lowering.cc", "MemoryLowering — 内存访问/分配降级 (AllocationState 折叠 + 写屏障消除)")}
//! MemoryLowering: Allocate/Load*/Store* → 机器算子;
//! 分配折叠 (AllocationGroup/State) 与写屏障消除在此隐式完成。
//!
//! 规则面: 枚举 + AllocationState 三工厂 + 方法面 ${ccMethods.length}。
//! 语义面 (推迟): ComputeWriteBarrierKind 的值/对象判定 (cc)。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/memory-lowering.h MemoryLowering::AllocationFolding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationFolding {
  /// kDoAllocationFolding
  Do,
  /// kDontAllocationFolding (默认)
  Dont,
}

/// v8: src/compiler/memory-lowering.h MemoryLowering::AllocationState
/// 效应路径上的分配状态; size 为折叠上界 (不可折叠 = max int)
pub struct AllocationState {
  /// 原 group_
  pub group: *mut AllocationGroup,
  /// 原 size_ — 当前路径合并分配尺寸上界
  pub size: isize,
  /// 原 top_
  pub top: crate::node::Node,
  /// 原 effect_
  pub effect: crate::node::Node,
}

impl AllocationState {
  /// v8: src/compiler/memory-lowering.h AllocationState::Empty
  /// 原文: zone->New<AllocationState>() — 无组/无界/无 top
  pub fn empty(zone: &mut crate::zone::Zone) -> *const AllocationState {
    let _ = zone;
    todo!("空状态")
  }
  /// v8: src/compiler/memory-lowering.h AllocationState::Closed
  /// 原文: (group, effect) — 已关闭不可继续折叠
  pub fn closed(group: *mut AllocationGroup, effect: crate::node::Node, zone: &mut crate::zone::Zone) -> *const AllocationState {
    let _ = (group, effect, zone);
    todo!("关闭态")
  }
  /// v8: src/compiler/memory-lowering.h AllocationState::Open
  /// 原文: (group, size, top, effect) — 开放可折叠
  pub fn open(group: *mut AllocationGroup, size: isize, top: crate::node::Node, effect: crate::node::Node, zone: &mut crate::zone::Zone) -> *const AllocationState {
    let _ = (group, size, top, effect, zone);
    todo!("开放态")
  }

  /// v8: src/compiler/memory-lowering.cc AllocationState::IsYoungGenerationAllocation
  pub fn is_young_generation_allocation(&self) -> bool {
    todo!("组存在且 young")
  }
}

/// v8: src/compiler/machine-graph.js 备注: AllocationGroup 在 memory-lowering.cc 定义
/// (对象组: 同批折叠分配)
pub struct AllocationGroup {
  /// 原 allocation_ + young 位
  pub _semantic: (),
}

/// v8: src/compiler/memory-lowering.h MemoryLowering
pub struct MemoryLowering {
  /// 原 jsgraph_ / zone_ / graph_assembler_
  pub jsgraph: *mut crate::js_graph::JSGraph,
  pub zone: *mut crate::zone::Zone,
  pub graph_assembler: *mut crate::graph_assembler::JSGraphAssembler,
  /// 原 allocation_folding_ (默认 kDontAllocationFolding)
  pub allocation_folding: AllocationFolding,
}

impl MemoryLowering {
  /// v8: src/compiler/memory-lowering.cc MemoryLowering::reducer_name
  pub const REDUCER_NAME: &'static str = "MemoryReducer";

  /// v8: src/compiler/memory-lowering.cc MemoryLowering::Reduce (总入口)
  pub fn reduce(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    let _ = node;
    todo!("opcode 分派到各 Reduce*")
  }

  /// v8: src/compiler/memory-lowering.cc MemoryLowering::ReduceAllocateRaw (双参)
  /// 语义: 折叠判定 (size 上界) + top/limit 机器序列
  pub fn reduce_allocate_raw(&mut self, node: &crate::node::Node, allocation_type: crate::AllocationType, state: &mut *const AllocationState) -> crate::graph_reducer::Reduction {
    let _ = (node, allocation_type, state);
    todo!("分配折叠或独立分配")
  }
  /// v8: src/compiler/memory-lowering.cc MemoryLowering::ReduceLoadFromObject
  pub fn reduce_load_from_object(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    let _ = node;
    todo!("LoadFromObject → Load")
  }
  /// v8: src/compiler/memory-lowering.cc MemoryLowering::ReduceLoadElement
  pub fn reduce_load_element(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    let _ = node;
    todo!("ComputeIndex + Load")
  }
  /// v8: src/compiler/memory-lowering.cc MemoryLowering::ReduceLoadField
  pub fn reduce_load_field(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    let _ = node;
    todo!("Load(offset)")
  }
  /// v8: src/compiler/memory-lowering.cc MemoryLowering::ReduceStoreToObject (原默认参 state=nullptr)
  pub fn reduce_store_to_object(&mut self, node: &crate::node::Node, state: *const AllocationState) -> crate::graph_reducer::Reduction {
    let _ = (node, state);
    todo!("Store + 写屏障判定")
  }
  /// v8: src/compiler/memory-lowering.cc MemoryLowering::ReduceStoreElement (原默认参 state=nullptr)
  pub fn reduce_store_element(&mut self, node: &crate::node::Node, state: *const AllocationState) -> crate::graph_reducer::Reduction {
    let _ = (node, state);
    todo!("ComputeIndex + Store")
  }
  /// v8: src/compiler/memory-lowering.cc MemoryLowering::ReduceStoreField (原默认参 state=nullptr)
  pub fn reduce_store_field(&mut self, node: &crate::node::Node, state: *const AllocationState) -> crate::graph_reducer::Reduction {
    let _ = (node, state);
    todo!("Store(offset) + 写屏障判定")
  }
  /// v8: src/compiler/memory-lowering.cc MemoryLowering::ComputeWriteBarrierKind
  /// 语义: 值为 Smi / 组内引用 / 分配期 store → 削屏障
  pub fn compute_write_barrier_kind(&mut self, node: &crate::node::Node, object: &crate::node::Node, value: &crate::node::Node, state: *const AllocationState, write_barrier_kind: crate::WriteBarrierKind) -> crate::WriteBarrierKind {
    let _ = (node, object, value, state, write_barrier_kind);
    todo!("屏障削减链")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "memory_lowering.rs.skel", body);
console.log(`written ${p}\nAllocationFolding=2 State工厂=3 方法面(cc)=${ccMethods.length} 公开Reduce*=8`);
