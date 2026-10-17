#!/usr/bin/env bun
// tool/src/compiler/wasm-address-reassociation.js
// v8/src/compiler/wasm-address-reassociation.h + wasm-address-reassociation.cc
// → v8-compiler-passes/src/wasm_address_reassociation.rs.skel
// 地址重结合: 解析 CandidateAddressKey 三元组与候选面方法。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-address-reassociation.h");
const cc = readV8("src/compiler/wasm-address-reassociation.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-address-reassociation: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE WasmAddressReassociation final {", "类定义"],
  ["using CandidateAddressKey = std::tuple<NodeId, NodeId, NodeId>;", "键三元组"],
  ["class CandidateBaseAddr {", "CandidateBaseAddr"],
  ["class CandidateMemOps : ZoneObject {", "CandidateMemOps"],
  ["void VisitTrappingMemOp(Node* node, NodeId effect_chain);", "VisitTrappingMemOp"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— 方法面 (真解析) ——
const methods = [...h.matchAll(/^\s{2}(?:bool|void|Node\*|size_t|int64_t) (\w+)\(([^()]*)\)(?: const)?;/gm)].map((m) => m[1]);
for (const m of ["ShouldTryOptimize", "CreateNewBase", "HasCandidateBaseAddr", "AddCandidate", "ReplaceInputs", "VisitTrappingMemOp"]) {
  need(methods.includes(m), `方法 ${m} 缺席 (实得 ${methods.join(",")})`);
}

// —— CandidateMemOps 面 ——
const cmo = h.slice(h.indexOf("class CandidateMemOps"));
const cmoMethods = [...cmo.slice(0, cmo.indexOf("\n  };")).matchAll(/^\s{4}(?:void|size_t|Node\*|int64_t) (\w+)\(/gm)].map((m) => m[1]);
need(cmoMethods.includes("AddCandidate") && cmoMethods.includes("GetNumNodes"), `CandidateMemOps ${cmoMethods}`);

// —— cc 对数 ——
const ccDefs = ["ShouldTryOptimize", "CreateNewBase", "HasCandidateBaseAddr", "AddCandidate", "ReplaceInputs", "VisitTrappingMemOp"].filter((m) => cc.includes(`WasmAddressReassociation::${m}(`));
need(ccDefs.length >= 5, `cc 定义 ${ccDefs.length}`);

const body = `${banner("src/compiler/wasm-address-reassociation.js", "src/compiler/wasm-address-reassociation.h + wasm-address-reassociation.cc", "wasm 内存访问基址+偏移重结合")}
//! wasm-address-reassociation — 同 (base, offset, effect 链) 的访存归并
//!
//! 键 = (NodeId, NodeId, NodeId); CandidateBaseAddr (base+offset 可合并对) →
//! CandidateMemOps (共享对象 + 即时偏移的 load/store 集)。

/// 原版 CandidateAddressKey — (base_id, offset_id, effect_chain_id)
pub type CandidateAddressKey = (NodeId, NodeId, NodeId);

/// 原版 CandidateBaseAddr — 可合并的 (base, offset) 节点对
pub struct CandidateBaseAddr {
  /// base_reg_
  pub base_reg: *mut Node,
  /// offset_reg_
  pub offset_reg: *mut Node,
}

/// 原版 CandidateMemOps — 共享访问集 (mem_ops 与 imm_offsets 平行数组)
pub struct CandidateMemOps<'z> {
  /// mem_ops_
  pub mem_ops: ZoneVector<*mut Node>,
  /// imm_offsets_
  pub imm_offsets: ZoneVector<i64>,
}

impl CandidateMemOps<'_> {
  /// 原版 AddCandidate
  /// v8: src/compiler/wasm-address-reassociation.cc CandidateMemOps::AddCandidate
  pub fn add_candidate(&mut self, mem_op: *mut Node, imm_offset: i64) {
    todo!("v8: src/compiler/wasm-address-reassociation.cc CandidateMemOps::AddCandidate")
  }

  /// 原版 GetNumNodes / mem_op / imm_offset
  /// v8: src/compiler/wasm-address-reassociation.h CandidateMemOps::GetNumNodes
  pub fn get_num_nodes(&self) -> usize {
    self.mem_ops.len()
  }
}

/// 原版 WasmAddressReassociation — 归并器
pub struct WasmAddressReassociation<'z> {
  /// graph_
  pub graph: *mut TFGraph<'z>,
  /// common_ / machine_
  pub common: *mut CommonOperatorBuilder<'z>,
  pub machine: *mut MachineOperatorBuilder<'z>,
  /// candidate_base_addrs_
  pub candidate_base_addrs: ZoneMap<CandidateAddressKey, CandidateBaseAddr>,
  /// candidates_
  pub candidates: ZoneMap<CandidateAddressKey, CandidateMemOps<'z>>,
  /// zone_
  pub zone: &'z Zone<'z>,
}

impl WasmAddressReassociation<'_> {
${methods.map((m) => `  /// 原版 ${m}\n  /// v8: src/compiler/wasm-address-reassociation.cc WasmAddressReassociation::${m}\n  pub fn ${m.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self /*, 按签名 */) {\n    todo!("v8: src/compiler/wasm-address-reassociation.cc WasmAddressReassociation::${m}")\n  }`).join("\n\n")}
}
`;

const p = writeSrc("v8-compiler-passes", "wasm_address_reassociation.rs.skel", body);
console.log(`written ${p}\nmethods=${methods.length} cmo_methods=${cmoMethods.length} cc_defs=${ccDefs.length}`);
