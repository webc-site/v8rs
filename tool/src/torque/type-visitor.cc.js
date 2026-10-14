#!/usr/bin/env bun
// tool/src/torque/type-visitor.cc.js
// 机械提取 v8/src/torque/type-visitor.{h,cc}: TypeVisitor 访问分派与类型构造清单
// (既有 tool/src/torque/types.js 已产 type_visitor.rs, 此处为 .cc 对数清单, 落新文件)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/torque/type-visitor.h");
const ccSrc = readV8("src/torque/type-visitor.cc");

// 头侧访问器族 (TypeVisitor 全静态: visit 分派 + Visit* 结构遍历)
if (!/class TypeVisitor/.test(hSrc)) throw new Error("type-visitor.h 缺 TypeVisitor");
const visitNames = [...new Set([...hSrc.matchAll(/\b((?:visit|Visit|Compute)\w*)\(/g)].map((m) => m[1]))];
if (visitNames.length < 3) throw new Error(`visit 族应 ≥3, 实得 ${visitNames.length}: ${visitNames.join(",")}`);

// cc 侧定义头
function collectDefs(src) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    const l = ls[i];
    if (!/^[A-Za-z_][\w:<>,*&\s]*\b\w+::~?\w+\(/.test(l)) continue;
    let sig = l;
    let j = i;
    while (!/\{\s*$|;\s*$/.test(sig) && j + 1 < ls.length) sig += " " + ls[++j].trim();
    sig = sig.replace(/\{\s*$/, "").trim();
    if (sig.endsWith(";")) continue;
    out.push(sig);
  }
  return out;
}
const defs = collectDefs(ccSrc);
if (defs.length !== 11) throw new Error(`type-visitor.cc 定义头应 11, 实得 ${defs.length}`);
const classes = {};
for (const d of defs) {
  const c = (d.match(/(\w+)::/) || [])[1];
  classes[c] = (classes[c] || 0) + 1;
}
// cc 侧 visitor 实现类 (TypeVisitor 本体)
const implClasses = Object.keys(classes).filter((c) => /Visitor$/.test(c));
if (!implClasses.includes("TypeVisitor")) throw new Error(`type-visitor.cc 缺 TypeVisitor 实现`);

let rust = banner("src/torque/type-visitor.cc.js", "src/torque/type-visitor.{h,cc}", "TypeVisitor 访问器对数清单");

rust += `
/// TypeVisitor 访问器面 (原版 type-visitor.h; visit 族 ${visitNames.length} 项: ${visitNames.join(", ")})
pub mod type_visitor_surface {
  pub const VISIT_METHODS: [&str; ${visitNames.length}] = [${visitNames.map((c) => `"${c}"`).join(", ")}];
  /// cc 侧 visitor 实现类 (${implClasses.length} 项)
  pub const IMPL_CLASSES: [&str; ${implClasses.length}] = [${implClasses.map((c) => `"${c}"`).join(", ")}];
}

/// type-visitor.cc 方法定义头清单 (对数基准)。
/// 类 → 方法定义数: ${Object.entries(classes).map(([k, v]) => `${k}=${v}`).join(", ")}

`;
for (const d of defs) {
  rust += `// ${d}\n`;
}
rust += `
pub mod type_visitor_cc_skel {
  pub const DEF_COUNT: usize = ${defs.length};
}
`;

const p = writeSrc("v8-torque", "type_visitor_cc_generated.rs", rust);
console.log(`written ${p}; 定义头=${defs.length}, 实现类=${implClasses.length}, visit族=${visitNames.length}`);
