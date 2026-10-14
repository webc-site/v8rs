#!/usr/bin/env bun
// tool/src/ast/ast.cc.js
// 机械提取 v8/src/ast/ 实现族 .cc 定义头清单:
// ast.cc + ast-value-factory.cc + modules.cc + variables.cc
// (>2k 行, 按规则只取定义头对数清单)

import { readV8, writeSrc, banner } from "../../lib.js";

function collectDefs(src, prefix) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    const l = ls[i];
    if (!new RegExp(`^[\\w:<>,*&\\s]*\\b${prefix}::~?\\w+\\(`).test(l) || /^\s*\/\//.test(l)) continue;
    let sig = l;
    let j = i;
    while (!/\{\}?\s*$|;\s*$/.test(sig) && j + 1 < ls.length) sig += " " + ls[++j].trim();
    sig = sig.replace(/\{\s*$/, "").replace(/\{\}\s*$/, "").trim();
    if (sig.endsWith(";")) continue;
    out.push(sig);
  }
  return out;
}

const files = [
  { cc: "src/ast/ast.cc", cls: "Expression", min: 15 },
  { cc: "src/ast/ast.cc", cls: "FunctionLiteral", min: 10 },
  { cc: "src/ast/ast.cc", cls: "AstNode", min: 2 },
  { cc: "src/ast/ast-value-factory.cc", cls: "AstRawString", min: 1 },
  { cc: "src/ast/ast-value-factory.cc", cls: "AstValueFactory", min: 1 },
  { cc: "src/ast/modules.cc", cls: "SourceTextModuleDescriptor", min: 3 },
  { cc: "src/ast/variables.cc", cls: "Variable", min: 2 },
];

const sections = [];
for (const f of files) {
  const src = readV8(f.cc);
  const defs = collectDefs(src, f.cls);
  if (defs.length < f.min) throw new Error(`${f.cc} ${f.cls} 定义头应 ≥${f.min}, 实得 ${defs.length}`);
  sections.push({ file: f.cc, cls: f.cls, defs });
}

// 关键锚点: ast.cc 侧 AstNode 构造与重组; variables.cc 侧 Variable 模式判定
const astCc = readV8("src/ast/ast.cc");
if (!astCc.includes("AstNode::")) throw new Error("ast.cc 缺 AstNode 定义");
const varCc = readV8("src/ast/variables.cc");
if (!varCc.includes("Variable::")) throw new Error("variables.cc 缺 Variable 定义");

let rust = banner(
  "src/ast/ast.cc.js",
  "src/ast/ast.cc + ast-value-factory.cc + modules.cc + variables.cc",
  "AST 实现族定义头对数清单",
);

rust += `
`;
for (const s of sections) {
  rust += `// ${s.file} — ${s.cls} 定义头清单 (${s.defs.length} 项):\n${s.defs.map((d) => "// " + d).join("\n")}\n\n`;
}
rust += `pub mod ast_cc_skel {
  /// 四文件定义头总数对数守卫
  pub const DEF_TOTAL: usize = ${sections.reduce((a, s) => a + s.defs.length, 0)};
}
`;

const p = writeSrc("v8-ast", "ast_cc_generated.rs", rust);
console.log(`written ${p}; ${sections.map((s) => `${s.cls}=${s.defs.length}`).join(", ")}`);
