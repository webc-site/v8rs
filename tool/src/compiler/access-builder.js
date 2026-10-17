#!/usr/bin/env bun
// v8/src/compiler/access-builder.h + access-builder.cc
// → v8-compiler-passes/src/access_builder.rs.skel
//
// AccessBuilder 为纯静态工厂 (无枚举/常量), 规则面 = 全部
// static (FieldAccess|ElementAccess) ForXxx(...) 签名 1:1 展开:
// 平衡括号扫描提取参数表, C++ 形参类型映射 Rust (int→i32, size_t→usize),
// 默认实参降级为 doc 注释。语义体 (各字段的 offset/machine_type 常量拼装)
// 在 .cc, 属对象布局层, 留 todo! + /// v8: 锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/access-builder.h");
const cc = readV8("src/compiler/access-builder.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE AccessBuilder final/, "AccessBuilder 类"],
  [h, /static FieldAccess ForExternalIntPtr\(\);/, "首签名 ForExternalIntPtr"],
  [h, /static FieldAccess ForContextSideProperty\(\);/, "尾签名 ForContextSideProperty"],
  [h, /static FieldAccess ForMap\(WriteBarrierKind write_barrier = kMapWriteBarrier,/, "ForMap 默认参"],
  [h, /static ElementAccess ForFixedArrayElement\(\);/, "ElementAccess 面"],
  [cc, /FieldAccess AccessBuilder::ForExternalIntPtr\(\) \{/, "cc 首实现"],
  [cc, /FieldAccess AccessBuilder::ForWasmDispatchTableLength\(\) \{/, "cc 尾实现 (WASM 块)"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— 签名提取: 平衡括号扫描 ——
function extractSigs(src) {
  const out = [];
  const re = /static (FieldAccess|ElementAccess) (For\w+)\(/g;
  let m;
  while ((m = re.exec(src))) {
    let depth = 1;
    let i = re.lastIndex;
    while (depth > 0 && i < src.length) {
      const c = src[i];
      if (c === "(" || c === "{") depth++;
      else if (c === ")" || c === "}") depth--;
      i++;
    }
    out.push({ ret: m[1], name: m[2], params: src.slice(re.lastIndex, i - 1) });
  }
  return out;
}
const sigs = extractSigs(h);
if (sigs.length !== 136) throw new Error(`头文件签名 ${sigs.length} != 136`);
if (sigs[0].name !== "ForExternalIntPtr" || sigs.at(-1).name !== "ForContextSideProperty") {
  throw new Error(`签名首尾漂移: ${sigs[0].name}..${sigs.at(-1).name}`);
}
// .cc 实现数 (条件编译 BIGINT_NEEDS_PADDING 未开 → 头 136 = cc 134 + 条件 2)
const ccImpls = [...cc.matchAll(/^(?:FieldAccess|ElementAccess) AccessBuilder::(For\w+)\(/gm)].map((m) => m[1]);
if (ccImpls.length !== 134) throw new Error(`cc 实现 ${ccImpls.length} != 134`);

// —— 参数解析: 逗号切分 (花括号/圆括号平衡) + 类型映射 ——
function splitParams(params) {
  if (!params.trim()) return [];
  const out = [];
  let depth = 0;
  let cur = "";
  for (const ch of params.replace(/\s+/g, " ")) {
    if ("({[".includes(ch)) depth++;
    else if (")}]".includes(ch)) depth--;
    if (ch === "," && depth === 0) {
      out.push(cur.trim());
      cur = "";
    } else cur += ch;
  }
  out.push(cur.trim());
  return out;
}
const typeMap = { int: "i32", "int32_t": "i32", "size_t": "usize", bool: "bool" };
const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
function parseParam(p) {
  const eq = p.indexOf("=");
  const body = eq >= 0 ? p.slice(0, eq).trim() : p;
  const def = eq >= 0 ? p.slice(eq + 1).trim() : null;
  // 形如 "WriteBarrierKind write_barrier" / "int index"; 类型可含 :: 与模板
  const sp = body.lastIndexOf(" ");
  const type = body.slice(0, sp).replace(/&/g, "").replace(/const /g, "").trim();
  const name = body.slice(sp + 1).trim();
  return { type: typeMap[type] ?? type, name: snake(name), def };
}

// —— 生成 ——
const fns = sigs
  .map((s) => {
    const ps = splitParams(s.params).map(parseParam);
    const args = ps.map((p) => `${p.name}: ${p.type}`).join(", ");
    const defs = ps.filter((p) => p.def).map((p) => `${p.name}=${p.def}`);
    const defNote = defs.length ? `\n  /// 原默认参: ${defs.join(", ")}` : "";
    const ret = s.ret;
    return [
      `  /// v8: src/compiler/access-builder.h AccessBuilder::${s.name}`,
      `  /// v8: src/compiler/access-builder.cc AccessBuilder::${s.name}${defNote}`,
      `  pub fn ${snake(s.name.replace(/^For/, "for_"))}(${args}) -> ${ret} {`,
      `    todo!("AccessBuilder::${s.name}")`,
      `  }`,
    ].join("\n");
  })
  .join("\n\n");

const body = `${banner("src/compiler/access-builder.js", "src/compiler/access-builder.h + src/compiler/access-builder.cc", "AccessBuilder — 堆对象字段/元素访问描述 (FieldAccess/ElementAccess) 静态工厂, 136 签名骨架")}
//! AccessBuilder — 全静态工厂, 无实例状态。
//!
//! 规则面 (本文件): 136 个 ForXxx 签名 1:1 展开。
//! 语义面 (推迟): 每个访问的 offset / machine_type / write_barrier /
//! 常量折叠类型 由对象布局层 (objects/) 决定, 见 .cc 各实现体。
//! 条件编译: ForBigIntOptionalPadding(BIGINT_NEEDS_PADDING) /
//! ForJSExternalObjectPointerHandle(V8_ENABLE_SANDBOX) /
//! ForWasmArrayLength+ForWasmDispatchTableLength(V8_ENABLE_WEBASSEMBLY)。

#![allow(dead_code, non_snake_case, unused_imports)]

/// 原 FieldAccess — 字段访问描述 (simplified-operator 层类型)
pub type FieldAccess = crate::simplified::FieldAccess;
/// 原 ElementAccess — 元素访问描述
pub type ElementAccess = crate::simplified::ElementAccess;

/// v8: src/compiler/access-builder.h AccessBuilder
pub struct AccessBuilder;

impl AccessBuilder {
${fns}
}
`;

const p = writeSrc("v8-compiler-passes", "access_builder.rs.skel", body);
console.log(
  [
    `written ${p}`,
    `h 签名=${sigs.length} cc 实现=${ccImpls.length} (差 2 = BIGINT_PADDING 未开)`,
    `FieldAccess=${sigs.filter((s) => s.ret === "FieldAccess").length} ElementAccess=${sigs.filter((s) => s.ret === "ElementAccess").length}`,
  ].join("\n"),
);
