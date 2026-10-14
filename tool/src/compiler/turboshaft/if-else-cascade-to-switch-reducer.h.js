#!/usr/bin/env bun
// tool/src/compiler/turboshaft/if-else-cascade-to-switch-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/if-else-cascade-to-switch-reducer.h
// → v8-turboshaft/src/if_else_cascade_to_switch_reducer.rs.skel
//
// 提取面: IfElseCascadeToSwitchReducer 类声明 + TURBOSHAFT_REDUCER_BOILERPLATE(IfElseCascadeToSwitch) +
// REDUCE 方法定义面 (平衡括号取参)。语义实现禁编造。
// 级联探测 + 纯块内联后消除
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/if-else-cascade-to-switch-reducer.h");

// 1. 类声明 + boilerplate 宏
if (!new RegExp(`class IfElseCascadeToSwitchReducer[^;]*`).test(h)) throw new Error("未找到 class IfElseCascadeToSwitchReducer");
if (!h.includes("TURBOSHAFT_REDUCER_BOILERPLATE(IfElseCascadeToSwitch)")) {
  throw new Error("未找到 TURBOSHAFT_REDUCER_BOILERPLATE(IfElseCascadeToSwitch)");
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
const expected = [];
if (JSON.stringify(methods.map((x) => x.name)) !== JSON.stringify(expected)) {
  throw new Error(`REDUCE 名单不匹配: ${JSON.stringify(methods.map((x) => x.name))}`);
}

// 哨兵: 该族关键结构锚
for (const [re, label] of [
  [/static bool ContainsOnlyPureOps\(const Block\* block, const Graph& graph\)/, "ContainsOnlyPureOps"],
  [/void InlineAllOperationsWithoutLast\(const Block\* input_block\)/, "InlineAllOperationsWithoutLast"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/if-else-cascade-to-switch-reducer.h.js",
  "src/compiler/turboshaft/if-else-cascade-to-switch-reducer.h",
  "if-else 级联转 switch reducer",
);
rust += `
/// v8: src/compiler/turboshaft/if-else-cascade-to-switch-reducer.h IfElseCascadeToSwitchReducer
/// 级联探测 + 纯块内联后消除
pub struct IfElseCascadeToSwitchReducer {
  pub placeholder: (),
}

impl IfElseCascadeToSwitchReducer {
`;
for (const m of methods) {
  const snakeName = m.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toLowerCase();
  rust += `  /// 原版 ${m.ret} REDUCE(${m.name})(${m.params})
  /// v8: src/compiler/turboshaft/if-else-cascade-to-switch-reducer.h IfElseCascadeToSwitchReducer::REDUCE(${m.name})
  pub fn reduce_${snakeName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}
`;

const p = writeSrc("v8-turboshaft", "if_else_cascade_to_switch_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`类=IfElseCascadeToSwitchReducer REDUCE=${methods.length} 哨兵=2`);
