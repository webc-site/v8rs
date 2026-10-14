#!/usr/bin/env bun
// tool/src/compiler/turboshaft/machine-lowering-reducer-inl.h.js
// 机械提取 v8/src/compiler/turboshaft/machine-lowering-reducer-inl.h
// → v8-turboshaft/src/machine_lowering_reducer_inl.rs.skel
//
// 提取面: MachineLoweringReducer 类声明 + TURBOSHAFT_REDUCER_BOILERPLATE(MachineLowering) +
// REDUCE 方法定义面 (平衡括号取参) + 枚举变体表。语义实现禁编造。
// 60 个 REDUCE 覆盖 JS 语义操作降级
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/machine-lowering-reducer-inl.h");

// 1. 类声明 + boilerplate 宏
if (!new RegExp(`class MachineLoweringReducer[^;]*`).test(h)) throw new Error("未找到 class MachineLoweringReducer");
if (!h.includes("TURBOSHAFT_REDUCER_BOILERPLATE(MachineLowering)")) {
  throw new Error("未找到 TURBOSHAFT_REDUCER_BOILERPLATE(MachineLowering)");
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
if (methods.length !== 60) throw new Error(`REDUCE 数 ${methods.length} != 60`);

// 3. 枚举 StringEncoding (类内 enum class)
const enM = h.match(/enum (?:class )?StringEncoding\s*(:\s*\w+)?\s*\{([\s\S]*?)\};/);
if (!enM) throw new Error("未找到 enum class StringEncoding");
const enVariants = [...enM[2].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((x) => x[1]);
if (JSON.stringify(enVariants) !== JSON.stringify(["OneByte","TwoByte","Unknown"])) {
  throw new Error(`StringEncoding 变体 ${JSON.stringify(enVariants)} != 预期`);
}

// 哨兵: 该族关键结构锚
for (const [re, label] of [
  [/V<Untagged> REDUCE\(ChangeOrDeopt\)/, "链首锚 ChangeOrDeopt"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/machine-lowering-reducer-inl.h.js",
  "src/compiler/turboshaft/machine-lowering-reducer-inl.h",
  "机器级降级 reducer (simplified→machine)",
);
rust += `
/// v8: src/compiler/turboshaft/machine-lowering-reducer-inl.h StringEncoding — 类内 enum class
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringEncoding {
  /// 原版 kOneByte
  oneByte,
  /// 原版 kTwoByte
  twoByte,
  /// 原版 kUnknown
  unknown,
}

/// v8: src/compiler/turboshaft/machine-lowering-reducer-inl.h MachineLoweringReducer
/// 60 个 REDUCE 覆盖 JS 语义操作降级
pub struct MachineLoweringReducer {
  pub placeholder: (),
}

impl MachineLoweringReducer {
`;
for (const m of methods) {
  const snakeName = m.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toLowerCase();
  rust += `  /// 原版 ${m.ret} REDUCE(${m.name})(${m.params})
  /// v8: src/compiler/turboshaft/machine-lowering-reducer-inl.h MachineLoweringReducer::REDUCE(${m.name})
  pub fn reduce_${snakeName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}
`;

const p = writeSrc("v8-turboshaft", "machine_lowering_reducer_inl.rs.skel", rust);
console.log(`written ${p}`);
console.log(`类=MachineLoweringReducer REDUCE=${methods.length} 枚举=1 哨兵=1`);
