#!/usr/bin/env bun
// tool/src/compiler/turboshaft/branch-elimination-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/branch-elimination-reducer.h
// → v8-turboshaft/src/branch_elimination_reducer.rs.skel
//
// 提取面: BranchEliminationReducer 类声明 + TURBOSHAFT_REDUCER_BOILERPLATE(BranchElimination) +
// REDUCE 方法定义面 (平衡括号取参) + 枚举变体表。语义实现禁编造。
// 条件值 LayeredHashMap 缓存 + 支配前驱合并
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/branch-elimination-reducer.h");

// 1. 类声明 + boilerplate 宏
if (!new RegExp(`class BranchEliminationReducer[^;]*`).test(h)) throw new Error("未找到 class BranchEliminationReducer");
if (!h.includes("TURBOSHAFT_REDUCER_BOILERPLATE(BranchElimination)")) {
  throw new Error("未找到 TURBOSHAFT_REDUCER_BOILERPLATE(BranchElimination)");
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
const expected = ["Switch","Branch","Select","Goto","DeoptimizeIf","TrapIf"];
if (JSON.stringify(methods.map((x) => x.name)) !== JSON.stringify(expected)) {
  throw new Error(`REDUCE 名单不匹配: ${JSON.stringify(methods.map((x) => x.name))}`);
}

// 3. 枚举 KnownConditionType (类内 enum class : uint8_t (原版 k 前缀变体))
const enM = h.match(/enum (?:class )?KnownConditionType\s*(:\s*\w+)?\s*\{([\s\S]*?)\};/);
if (!enM) throw new Error("未找到 enum class KnownConditionType");
const enVariants = [...enM[2].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((x) => x[1]);
if (JSON.stringify(enVariants) !== JSON.stringify(["Invalid","AnyNonZero","Exact"])) {
  throw new Error(`KnownConditionType 变体 ${JSON.stringify(enVariants)} != 预期`);
}

// 哨兵: 该族关键结构锚
for (const [re, label] of [
  [/struct KnownCondition/, "KnownCondition"],
  [/known_conditions_/, "known_conditions_ (LayeredHashMap 条件缓存)"],
  [/LayeredHashMap<V<Word32>, KnownCondition> known_conditions_/, "known_conditions_ 声明 (LayeredHashMap<V<Word32>, KnownCondition>)"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/branch-elimination-reducer.h.js",
  "src/compiler/turboshaft/branch-elimination-reducer.h",
  "分支消除 reducer",
);
rust += `
/// v8: src/compiler/turboshaft/branch-elimination-reducer.h KnownConditionType — 类内 enum class : uint8_t (原版 k 前缀变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnownConditionType {
  /// 原版 kInvalid
  invalid,
  /// 原版 kAnyNonZero
  anyNonZero,
  /// 原版 kExact
  exact,
}

/// v8: src/compiler/turboshaft/branch-elimination-reducer.h BranchEliminationReducer
/// 条件值 LayeredHashMap 缓存 + 支配前驱合并
pub struct BranchEliminationReducer {
  pub placeholder: (),
}

impl BranchEliminationReducer {
`;
for (const m of methods) {
  const snakeName = m.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toLowerCase();
  rust += `  /// 原版 ${m.ret} REDUCE(${m.name})(${m.params})
  /// v8: src/compiler/turboshaft/branch-elimination-reducer.h BranchEliminationReducer::REDUCE(${m.name})
  pub fn reduce_${snakeName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}
`;

const p = writeSrc("v8-turboshaft", "branch_elimination_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`类=BranchEliminationReducer REDUCE=${methods.length} 枚举=1 哨兵=3`);
