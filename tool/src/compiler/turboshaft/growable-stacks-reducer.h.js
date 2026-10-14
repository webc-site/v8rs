#!/usr/bin/env bun
// tool/src/compiler/turboshaft/growable-stacks-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/growable-stacks-reducer.h
// → v8-turboshaft/src/growable_stacks_reducer.rs.skel
//
// 提取面: GrowableStacksReducer 类声明 + TURBOSHAFT_REDUCER_BOILERPLATE(GrowableStacks) +
// REDUCE 方法定义面 (平衡括号取参)。语义实现禁编造。
// WasmStackCheck → 条件栈增长调用
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/growable-stacks-reducer.h");

// 1. 类声明 + boilerplate 宏
if (!new RegExp(`class GrowableStacksReducer[^;]*`).test(h)) throw new Error("未找到 class GrowableStacksReducer");
if (!h.includes("TURBOSHAFT_REDUCER_BOILERPLATE(GrowableStacks)")) {
  throw new Error("未找到 TURBOSHAFT_REDUCER_BOILERPLATE(GrowableStacks)");
}

// 2. REDUCE 方法定义提取: 返回类型 + 名称 + 平衡括号参数串
function reduceMethods(src) {
  const out = [];
  const re = /([^\n;{}]+?)\bREDUCE\((\w+)\)\s*\(/g;
  let m;
  while ((m = re.exec(src)) !== null) {
    const name = m[2];
    // 从 ( 起扫描平衡括号取参数
    let depth = 0, i = re.lastIndex - 1, start = -1;
    for (; i < src.length; i++) {
      const ch = src[i];
      if (ch === "(") { depth++; if (start < 0) start = i; }
      else if (ch === ")") { depth--; if (depth === 0) break; }
    }
    const params = src.slice(start + 1, i).replace(/\s+/g, " ").trim();
    const ret = m[1].trim().split(/\s+/).pop();
    out.push({ name, ret, params });
  }
  return out;
}
const methods = reduceMethods(h);
const expected = ["WasmStackCheck"];
if (JSON.stringify(methods.map((x) => x.name)) !== JSON.stringify(expected)) {
  throw new Error(`REDUCE 名单不匹配: ${JSON.stringify(methods.map((x) => x.name))}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/growable-stacks-reducer.h.js",
  "src/compiler/turboshaft/growable-stacks-reducer.h",
  "可增长栈 reducer",
);
rust += `
/// v8: src/compiler/turboshaft/growable-stacks-reducer.h GrowableStacksReducer
/// WasmStackCheck → 条件栈增长调用
pub struct GrowableStacksReducer {
  pub placeholder: (),
}

impl GrowableStacksReducer {
`;
for (const m of methods) {
  const snakeName = m.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toLowerCase();
  rust += `  /// 原版 ${m.ret} REDUCE(${m.name})(${m.params})
  /// v8: src/compiler/turboshaft/growable-stacks-reducer.h GrowableStacksReducer::REDUCE(${m.name})
  pub fn reduce_${snakeName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}
`;

const p = writeSrc("v8-turboshaft", "growable_stacks_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`类=GrowableStacksReducer REDUCE=${methods.length} 哨兵=0`);
