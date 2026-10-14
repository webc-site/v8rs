#!/usr/bin/env bun
// tool/src/compiler/turboshaft/dead-code-elimination-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/dead-code-elimination-reducer.h
// → v8-turboshaft/src/dead_code_elimination_reducer.rs.skel
//
// 提取面: DeadCodeEliminationReducer 类声明 + TURBOSHAFT_REDUCER_BOILERPLATE(DeadCodeElimination) +
// REDUCE 方法定义面 (平衡括号取参) + 枚举变体表。语义实现禁编造。
// 不动点活跃分析 (operation/block 双状态)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/dead-code-elimination-reducer.h");

// 1. 类声明 + boilerplate 宏
if (!new RegExp(`class DeadCodeEliminationReducer[^;]*`).test(h)) throw new Error("未找到 class DeadCodeEliminationReducer");
if (!h.includes("TURBOSHAFT_REDUCER_BOILERPLATE(DeadCodeElimination)")) {
  throw new Error("未找到 TURBOSHAFT_REDUCER_BOILERPLATE(DeadCodeElimination)");
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

// 3. 枚举 Liveness (OperationState::Liveness : uint8_t (格 Live>Dead, LeastUpperBound=|; 非 enum class))
const enM = h.match(/enum (?:class )?Liveness\s*(:\s*\w+)?\s*\{([\s\S]*?)\};/);
if (!enM) throw new Error("未找到 enum class Liveness");
const enVariants = [...enM[2].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((x) => x[1]);
if (JSON.stringify(enVariants) !== JSON.stringify(["Dead","Live"])) {
  throw new Error(`Liveness 变体 ${JSON.stringify(enVariants)} != 预期`);
}

// 哨兵: 该族关键结构锚
for (const [re, label] of [
  [/struct ControlState/, "ControlState"],
  [/struct OperationState/, "OperationState"],
  [/class DeadCodeAnalysis/, "DeadCodeAnalysis"],
  [/static Liveness LeastUpperBound\(Liveness lhs, Liveness rhs\)/, "LeastUpperBound"],
  [/static_assert\(kDead == 0 && kLive == 1\);/, "格值断言 kDead=0 kLive=1"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/dead-code-elimination-reducer.h.js",
  "src/compiler/turboshaft/dead-code-elimination-reducer.h",
  "死代码消除 reducer",
);
rust += `
/// v8: src/compiler/turboshaft/dead-code-elimination-reducer.h Liveness — OperationState::Liveness : uint8_t (格 Live>Dead, LeastUpperBound=|; 非 enum class)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Liveness {
  /// 原版 kDead
  dead,
  /// 原版 kLive
  live,
}

/// v8: src/compiler/turboshaft/dead-code-elimination-reducer.h DeadCodeEliminationReducer
/// 不动点活跃分析 (operation/block 双状态)
pub struct DeadCodeEliminationReducer {
  pub placeholder: (),
}

impl DeadCodeEliminationReducer {
`;
for (const m of methods) {
  const snakeName = m.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toLowerCase();
  rust += `  /// 原版 ${m.ret} REDUCE(${m.name})(${m.params})
  /// v8: src/compiler/turboshaft/dead-code-elimination-reducer.h DeadCodeEliminationReducer::REDUCE(${m.name})
  pub fn reduce_${snakeName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}
`;

const p = writeSrc("v8-turboshaft", "dead_code_elimination_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`类=DeadCodeEliminationReducer REDUCE=${methods.length} 枚举=1 哨兵=5`);
