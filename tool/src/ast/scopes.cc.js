#!/usr/bin/env bun
// tool/src/ast/scopes.cc.js
// 机械提取 v8/src/ast/scopes.cc: 作用域解析实现定义头清单 (>2k 行, 只取定义头)

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/ast/scopes.cc");

function collectDefs(src, prefixes) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    const l = ls[i];
    if (!/^\s*\/\//.test(l)) {
      for (const prefix of prefixes) {
        if (new RegExp(`^[\\w:<>,*&\\s]*\\b${prefix}::~?\\w+\\(`).test(l)) {
          let sig = l;
          let j = i;
          while (!/\{\}?\s*$|;\s*$/.test(sig) && j + 1 < ls.length) sig += " " + ls[++j].trim();
          sig = sig.replace(/\{\s*$/, "").replace(/\{\}\s*$/, "").trim();
          if (!sig.endsWith(";")) out.push(sig);
          break;
        }
      }
    }
  }
  return out;
}

const prefixes = ["Scope", "DeclarationScope", "ModuleScope", "ClassScope"];
const allDefs = collectDefs(ccSrc, prefixes);
if (allDefs.length < 80) throw new Error(`scopes.cc 定义头应 ≥80, 实得 ${allDefs.length}`);

// 按前缀分组对数
const groups = {};
for (const d of allDefs) {
  const c = (d.match(/(\w+)::/) || [])[1];
  groups[c] = (groups[c] || 0) + 1;
}
for (const need of ["Scope", "DeclarationScope"]) {
  if (!groups[need]) throw new Error(`scopes.cc 缺 ${need} 定义`);
}

// 关键锚点
for (const k of ["Scope::AllocateVariables", "DeclarationScope::AllocateParameterLocals"]) {
  if (!ccSrc.includes(k)) throw new Error(`scopes.cc 缺 ${k}`);
}

let rust = banner("src/ast/scopes.cc.js", "src/ast/scopes.cc", "作用域解析定义头对数清单");

rust += `
/// scopes.cc 定义头清单 (对数基准, 共 ${allDefs.length} 项)。
/// 类 → 定义数: ${Object.entries(groups).map(([k, v]) => `${k}=${v}`).join(", ")}

`;
for (const d of allDefs) {
  rust += `// ${d}\n`;
}
rust += `
pub mod scopes_cc_skel {
  /// 变量分配入口锚点 (原版 AllocateVariables / AllocateParameterLocals)
  pub const ALLOC_ANCHORS: [&str; 2] = ["Scope::AllocateVariables", "DeclarationScope::AllocateParameterLocals"];
  pub const DEF_TOTAL: usize = ${allDefs.length};
}
`;

const p = writeSrc("v8-ast", "scopes_cc_generated.rs", rust);
console.log(`written ${p}; 定义头=${allDefs.length}`);
