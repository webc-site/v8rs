#!/usr/bin/env bun
// tool/src/torque/declarations.cc.js
// 机械提取 v8/src/torque/declarations.cc: 声明表 .cc 侧方法定义头清单 (签名对数)

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/torque/declarations.cc");
const hSrc = readV8("src/torque/declarations.h");

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
if (defs.length < 30) throw new Error(`declarations.cc 定义头应 ≥30, 实得 ${defs.length}`);
const classes = {};
for (const d of defs) {
  const c = (d.match(/(\w+)::/) || [])[1];
  classes[c] = (classes[c] || 0) + 1;
}
// 对数: Declarations 为主体, namespace 级函数落 std/anonymous
if (!classes.Declarations) throw new Error("declarations.cc 缺 Declarations 方法定义");

// 头侧匿名命名空间辅助函数存在性 (DeclarationsChecker 等)
const checker = hSrc.includes("class Declarations") ? "Declarations" : null;
if (!checker) throw new Error("declarations.h 缺 Declarations");

let rust = banner("src/torque/declarations.cc.js", "src/torque/declarations.cc", "声明表 .cc 方法定义头对数清单");

rust += `
/// torque declarations.cc 方法定义头清单 (对数基准, 修订 declarations.rs 时对照)。
/// 类 → 方法定义数: ${Object.entries(classes).map(([k, v]) => `${k}=${v}`).join(", ")}

`;
for (const d of defs) {
  rust += `// ${d}\n`;
}
rust += `
pub mod declarations_cc_skel {
  /// 定义头总数对数守卫
  pub const DEF_COUNT: usize = ${defs.length};
}
`;

const p = writeSrc("v8-torque", "declarations_cc_generated.rs", rust);
console.log(`written ${p}; 定义头=${defs.length}, 类=${Object.keys(classes).length}`);
