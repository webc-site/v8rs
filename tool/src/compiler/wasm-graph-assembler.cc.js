#!/usr/bin/env bun
// tool/src/compiler/wasm-graph-assembler.cc.js
// 从 v8/src/compiler/wasm-graph-assembler.h + wasm-graph-assembler.cc
// 提取 WasmGraphAssembler 继承 GraphAssembler 的方法面 (visitor 签名骨架)
// 生成 v8-compiler-frontend/src/wasm_graph_assembler.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-graph-assembler.h");
const cc = readV8("src/compiler/wasm-graph-assembler.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class WasmGraphAssembler : public GraphAssembler {", "继承关系"],
  ["CallDescriptor* GetBuiltinCallDescriptor(", "自由函数: 内置调用描述符"],
  ["ObjectAccess ObjectAccessForGCStores(", "自由函数: GC store 访问"],
  ["Node* CallBuiltinThroughJumptable(", "跳转表调用"],
  ["Node* BuildChangeSmiToIntPtr(Node* value);", "Smi→IntPtr 转换"],
  ["Node* Allocate(int size);", "定长分配"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit) && !cc.includes(lit)) {
    throw new Error(`wasm-graph-assembler 锚点缺失: ${name}`);
  }
}

// —— 2. 方法面提取 (头声明) ——
const methods = [...new Set(
  [...h.matchAll(/^\s+(?:TNode<\w+>|Node\*|void|bool|int32_t|uint32_t|extern_ref_t)\s+(\w+)\(/gm)].map(
    (m) => m[1],
  ),
)].filter((n) => n !== "WasmGraphAssembler");

const rustName = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const body = `${banner("src/compiler/wasm-graph-assembler.cc.js", "src/compiler/wasm-graph-assembler.h + wasm-graph-assembler.cc", "wasm 图组装器: GraphAssembler 子类, wasm 引擎态字段访问面")}

/// WasmGraphAssembler — GraphAssembler 的 wasm 侧子类
/// v8: src/compiler/wasm-graph-assembler.h WasmGraphAssembler
pub struct WasmGraphAssembler {
  /// 基类组装器
  pub assembler: crate::graph_assembler::GraphAssembler,
}

impl WasmGraphAssembler {
${methods
  .map(
    (m) =>
      `  /// v8: src/compiler/wasm-graph-assembler.h WasmGraphAssembler::${m}\n  pub fn ${rustName(m)}(&mut self) {\n    todo!() // 推迟台账: wasm 引擎态 (Isolate/WasmEngine)\n  }`,
  )
  .join("\n\n")}
}

// —— 对数基准 ——
// 头方法面 ${methods.length} 项 / cc 定义面 ${(cc.match(/WasmGraphAssembler::/g) || []).length} 处引用
`;

const p = writeSrc("v8-compiler-frontend", "wasm_graph_assembler.rs.skel", body);
console.log(`OK ${p} methods=${methods.length}`);
