#!/usr/bin/env bun
// tool/src/compiler/turboshaft/int64-lowering-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/int64-lowering-reducer.h
// → v8-turboshaft/src/int64_lowering_reducer.rs.skel
//
// 提取面: Int64LoweringReducer 类声明 + TURBOSHAFT_REDUCER_BOILERPLATE(Int64Lowering) +
// REDUCE 方法定义面 (平衡括号取参)。语义实现禁编造。
// word64 → word32 对表示
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/int64-lowering-reducer.h");

// 1. 类声明 + boilerplate 宏
if (!new RegExp(`class Int64LoweringReducer[^;]*`).test(h)) throw new Error("未找到 class Int64LoweringReducer");
if (!h.includes("TURBOSHAFT_REDUCER_BOILERPLATE(Int64Lowering)")) {
  throw new Error("未找到 TURBOSHAFT_REDUCER_BOILERPLATE(Int64Lowering)");
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
const expected = ["WordBinop","Shift","Word64MulWide","Comparison","Call","TailCall","Constant","Parameter","Return","WordUnary","Change","Load","Store","AtomicRMW","Phi","PendingLoopPhi","Simd128Splat","Simd128ExtractLane","Simd128ReplaceLane","FrameState"];
if (JSON.stringify(methods.map((x) => x.name)) !== JSON.stringify(expected)) {
  throw new Error(`REDUCE 名单不匹配: ${JSON.stringify(methods.map((x) => x.name))}`);
}

// 哨兵: 该族关键结构锚
for (const [re, label] of [
  [/using Word32OrWord32Pair = OpIndex;/, "Word32OrWord32Pair = OpIndex 别名"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/int64-lowering-reducer.h.js",
  "src/compiler/turboshaft/int64-lowering-reducer.h",
  "int64 降级 reducer (32 位平台)",
);
rust += `
/// v8: src/compiler/turboshaft/int64-lowering-reducer.h Int64LoweringReducer
/// word64 → word32 对表示
pub struct Int64LoweringReducer {
  pub placeholder: (),
}

impl Int64LoweringReducer {
`;
for (const m of methods) {
  const snakeName = m.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toLowerCase();
  rust += `  /// 原版 ${m.ret} REDUCE(${m.name})(${m.params})
  /// v8: src/compiler/turboshaft/int64-lowering-reducer.h Int64LoweringReducer::REDUCE(${m.name})
  pub fn reduce_${snakeName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}
`;

const p = writeSrc("v8-turboshaft", "int64_lowering_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`类=Int64LoweringReducer REDUCE=${methods.length} 哨兵=1`);
