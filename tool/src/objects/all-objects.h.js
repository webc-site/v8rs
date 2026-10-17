#!/usr/bin/env bun
// tool/src/objects/all-objects.h.js — 全对象清单头生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/all-objects.h + all-objects-inl.h → v8-objects-core/src/all_objects.rs
// 规则面: 纯 include 清单 (metagen harvest 输入), 无任何常量/函数;
//   展开: WASM 开 / INTL 开 / TEMPORAL 关 (对齐 object-predicates.h.js 配置)
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/all-objects.h");
const inl = readV8("src/objects/all-objects-inl.h");

// include 提取: 状态机 (base / wasm / intl / temporal-丢弃), WASM 开 / INTL 开 / TEMPORAL 关
function sections(src) {
  const unconditional = [];
  const wasm = [];
  const intl = [];
  const stack = []; // 保存外层状态
  let state = "base";
  for (const line of src.split("\n")) {
    const m = line.match(/^#include "([^"]+)"$/);
    if (m) {
      if (state === "base") unconditional.push(m[1]);
      else if (state === "wasm") wasm.push(m[1]);
      else if (state === "intl") intl.push(m[1]);
      continue;
    }
    if (/^#if V8_ENABLE_WEBASSEMBLY$/.test(line)) { stack.push(state); state = "wasm"; }
    else if (/^#ifdef V8_INTL_SUPPORT$/.test(line)) { stack.push(state); state = "intl"; }
    else if (/^#ifdef V8_TEMPORAL_SUPPORT$/.test(line)) { stack.push(state); state = "temporal"; }
    else if (/^#endif/.test(line)) { state = stack.pop() ?? "base"; }
  }
  return { unconditional, wasm, intl };
}
const hs = sections(h);
const is = sections(inl);

// —— 对数 must 断言 (独立手数: 无条件 104 / wasm 2 / intl 13; inl 无条件 86) ——
if (hs.unconditional.length !== 104) throw new Error(`all-objects.h 无条件 include 应 104, 实得 ${hs.unconditional.length}`);
if (hs.wasm.length !== 2) throw new Error(`all-objects.h wasm include 应 2, 实得 ${hs.wasm.length}`);
if (hs.intl.length !== 13) throw new Error(`all-objects.h intl include 应 13, 实得 ${hs.intl.length}`);
if (is.unconditional.length !== 86) throw new Error(`all-objects-inl.h 无条件 include 应 86, 实得 ${is.unconditional.length}`);
if (is.wasm.length !== 2) throw new Error(`all-objects-inl.h wasm include 应 2, 实得 ${is.wasm.length}`);
if (is.intl.length !== 13) throw new Error(`all-objects-inl.h intl include 应 13, 实得 ${is.intl.length}`);
// 锚点: 关键头必须在列
for (const need of ["src/objects/heap-object.h", "src/objects/code.h", "src/objects/turboshaft-types.h"])
  if (!hs.unconditional.includes(need)) throw new Error(`all-objects.h 缺 ${need}`);
if (!is.unconditional.includes("src/objects/tagged-impl-inl.h")) throw new Error("all-objects-inl.h 缺 tagged-impl-inl.h");
// TEMPORAL 段被正确丢弃
if (hs.unconditional.some((p) => p.includes("temporal"))) throw new Error("TEMPORAL 头泄漏进无条件清单");

const allH = [...hs.unconditional, ...hs.wasm, ...hs.intl];
const allI = [...is.unconditional, ...is.wasm, ...is.intl];

const body = `${banner("src/objects/all-objects.h.js", "src/objects/all-objects.h + all-objects-inl.h", "纯 include 清单; 配置 WASM 开/INTL 开/TEMPORAL 关")}
//! 全对象清单 (原版 all-objects.h / all-objects-inl.h)
//!
//! 原版语义: BUILD.gn 只把 all-objects.h 声明为 metagen harvest 的输入,
//! PRESUBMIT 校验每个 V8_OBJECT 头都在列; Rust 侧无 include 概念,
//! 此处以字符串清单登记"对象头全集", 供对数/遗漏检查 (12.1 函数级对账) 使用.

/// 声明头清单 (all-objects.h, 无条件 ${hs.unconditional.length} + wasm ${hs.wasm.length} + intl ${hs.intl.length} = ${allH.length})
pub const ALL_OBJECT_HEADERS: [&str; ${allH.length}] = [
${allH.map((p) => `  "${p}",`).join("\n")}
];

/// 内联头清单 (all-objects-inl.h, 无条件 ${is.unconditional.length} + wasm ${is.wasm.length} + intl ${is.intl.length} = ${allI.length})
pub const ALL_OBJECT_INL_HEADERS: [&str; ${allI.length}] = [
${allI.map((p) => `  "${p}",`).join("\n")}
];
`;

const p = writeSrc("v8-objects-core", "all_objects.rs", body);
console.log(`all-objects.h.js: 已生成 ${p} (声明头 ${allH.length}, 内联头 ${allI.length})`);
