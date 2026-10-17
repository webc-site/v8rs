#!/usr/bin/env bun
// tool/src/compiler/turbofan-graph-visualizer.js
// v8/src/compiler/turbofan-graph-visualizer.h + turbofan-graph-visualizer.cc
// → v8-compiler-passes/src/turbofan_graph_visualizer.rs.skel
// Turbolizer 输出: 解析 JsonPrint* 自由函数 / AsJSON 包装结构清单 /
// operator<< 面与 SourceIdAssigner。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/turbofan-graph-visualizer.h");
const cc = readV8("src/compiler/turbofan-graph-visualizer.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`turbofan-graph-visualizer: ${msg}`);
};

for (const [s, name] of [
  ["struct V8_EXPORT_PRIVATE TurboJsonFile : public std::ofstream {", "TurboJsonFile"],
  ["struct TurboCfgFile : public std::ofstream {", "TurboCfgFile"],
  ["struct SourcePositionAsJSON {", "SourcePositionAsJSON"],
  ["struct NodeOriginAsJSON {", "NodeOriginAsJSON"],
  ["class V8_EXPORT_PRIVATE SourceIdAssigner {", "SourceIdAssigner"],
  ["struct GraphAsJSON {", "GraphAsJSON"],
  ["struct AsC1V {", "AsC1V"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— JsonPrint* 自由函数 (真解析) ——
const jsonPrints = [...h.matchAll(/^void (JsonPrint\w+)\(/gm)].map((m) => m[1]);
need(jsonPrints.length === 5, `JsonPrint* ${jsonPrints.length} (${jsonPrints.join(",")})`);
for (const n of ["JsonPrintAllBytecodeSources", "JsonPrintBytecodeSource", "JsonPrintAllSourceWithPositions", "JsonPrintAllSourceWithPositionsWasm", "JsonPrintFunctionSource"]) {
  need(jsonPrints.includes(n), `${n} 缺席`);
}

// —— AsJSON 包装结构清单 ——
const asJsonStructs = [...h.matchAll(/^struct (\w*AsJSON\w*) \{/gm)].map((m) => m[1]);
need(asJsonStructs.length >= 8, `AsJSON 结构 ${asJsonStructs.length}`);
for (const n of ["SourcePositionAsJSON", "NodeOriginAsJSON", "GraphAsJSON", "AsC1VCompilation", "InstructionOperandAsJSON", "InstructionSequenceAsJSON"]) {
  need(asJsonStructs.includes(n) || h.includes(`struct ${n}`), `${n} 缺席`);
}

// —— operator<< 声明面 ——
const oshift = [...h.matchAll(/^std::ostream& operator<<\(/gm)].length;
need(oshift >= 10, `operator<< ${oshift}`);

// —— SourceIdAssigner 方法 ——
const siaMethods = [...h.slice(h.indexOf("class V8_EXPORT_PRIVATE SourceIdAssigner {")).matchAll(/^\s{2}int (\w+)\(/gm)].map((m) => m[1]);
need(siaMethods.includes("GetIdFor") && siaMethods.includes("GetIdAt"), `SourceIdAssigner ${siaMethods}`);

// —— cc 定义对数 ——
const ccDefs = [
  "TurboJsonFile::TurboJsonFile(",
  "TurboCfgFile::TurboCfgFile(",
  "int SourceIdAssigner::GetIdFor(",
  "void JsonPrintAllBytecodeSources(",
  "void JsonPrintBytecodeSource(",
  "void JsonPrintAllSourceWithPositions(",
  "void JsonPrintFunctionSource(",
  "std::ostream& operator<<(std::ostream& out,\n                         const SourcePositionAsJSON& asJSON)",
  "std::ostream& operator<<(std::ostream& out, const NodeOriginAsJSON& asJSON)",
  "std::ostream& operator<<(std::ostream& os, const GraphAsJSON& ad)",
  "std::ostream& operator<<(std::ostream& os, const AsC1VCompilation& ac)",
  "std::ostream& operator<<(std::ostream& os, const AsC1V& ac)",
];
for (const d of ccDefs) need(cc.includes(d), `cc 缺 ${d}`);
const jsonOfNum = (cc.match(/JsonOf\(/g) || []).length;

const body = `${banner("src/compiler/turbofan-graph-visualizer.js", "src/compiler/turbofan-graph-visualizer.h + turbofan-graph-visualizer.cc", "Turbolizer JSON/c1v 图导出")}
//! turbofan-graph-visualizer — trace-turbo 序列化族
//!
//! JsonPrint* 源码输出 ${jsonPrints.length} 个; AsJSON 包装结构 ${asJsonStructs.length} 个
//! (${asJsonStructs.slice(0, 6).join(", ")}, ...);
//! operator<< 声明 ${oshift} 个; JsonOf 转义调用 ${jsonOfNum} 处。

/// 原版 TurboJsonFile — trace 文件 (JSON)
/// v8: src/compiler/turbofan-graph-visualizer.cc TurboJsonFile
pub struct TurboJsonFile;

/// 原版 TurboCfgFile — cfg 文件
/// v8: src/compiler/turbofan-graph-visualizer.cc TurboCfgFile
pub struct TurboCfgFile;

// —— AsJSON 包装结构清单 ——
${asJsonStructs.map((n) => `/// v8: src/compiler/turbofan-graph-visualizer.h struct ${n}\npub struct ${n};`).join("\n")}

/// 原版 SourceIdAssigner — SFI 去重编号器
pub struct SourceIdAssigner {
  /// printed_
  pub printed: Vec<Handle<SharedFunctionInfo>>,
  /// source_ids_
  pub source_ids: Vec<i32>,
}

impl SourceIdAssigner {
  /// 原版 GetIdFor — 新 SFI 分配递增 id, 已见则复用
  /// v8: src/compiler/turbofan-graph-visualizer.cc SourceIdAssigner::GetIdFor
  pub fn get_id_for(&mut self, shared: &Handle<SharedFunctionInfo>) -> i32 {
    todo!("v8: src/compiler/turbofan-graph-visualizer.cc SourceIdAssigner::GetIdFor")
  }

  /// 原版 GetIdAt
  /// v8: src/compiler/turbofan-graph-visualizer.h SourceIdAssigner::GetIdAt
  pub fn get_id_at(&self, pos: usize) -> i32 {
    self.source_ids[pos]
  }
}

${jsonPrints.map((n) => `/// 原版 ${n}\n/// v8: src/compiler/turbofan-graph-visualizer.cc ${n}\npub fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(os: &mut String /*, 按签名 */) {\n  todo!("v8: src/compiler/turbofan-graph-visualizer.cc ${n}")\n}`).join("\n\n")}

/// 原版 operator<<(GraphAsJSON) — 全图 JSON (节点/边/源位置/origin)
/// v8: src/compiler/turbofan-graph-visualizer.cc operator<<(GraphAsJSON)
pub fn graph_as_json(os: &mut String, g: &TFGraph, positions: &SourcePositionTable, origins: &NodeOriginTable) {
  todo!("v8: src/compiler/turbofan-graph-visualizer.cc operator<<(GraphAsJSON)")
}

/// 原版 operator<<(AsC1V) — Sea-of-Nodes c1v 可视化格式
/// v8: src/compiler/turbofan-graph-visualizer.cc operator<<(AsC1V)
pub fn as_c1v(os: &mut String, phase: &str, schedule: &Schedule, positions: &SourcePositionTable, types: Option<&Type>, origins: &NodeOriginTable) {
  todo!("v8: src/compiler/turbofan-graph-visualizer.cc operator<<(AsC1V)")
}
`;

const p = writeSrc("v8-compiler-passes", "turbofan_graph_visualizer.rs.skel", body);
console.log(`written ${p}\njson_prints=${jsonPrints.length} as_json_structs=${asJsonStructs.length} oshift=${oshift} sia=${siaMethods.length} cc_defs=${ccDefs.length} json_of=${jsonOfNum}`);
