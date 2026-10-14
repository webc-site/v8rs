#!/usr/bin/env bun
// tool/src/torque/declarable.cc.js
// 机械提取 v8/src/torque/declarable.cc: 可声明实体 .cc 侧定义头清单 + 带 NameVector 的打印函数

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/torque/declarable.cc");
const hSrc = readV8("src/torque/declarable.h");

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
if (defs.length !== 8) throw new Error(`declarable.cc 定义头应 8, 实得 ${defs.length}`);
const classes = {};
for (const d of defs) {
  const c = (d.match(/(\w+)::/) || [])[1];
  classes[c] = (classes[c] || 0) + 1;
}
if (!classes.QualifiedName) throw new Error("declarable.cc 缺 QualifiedName::Parse");
// declarable.h 实体族对数 (机械提取全部 class 名, 排除注释误配)
const rawClasses = [...hSrc.matchAll(/^\s*class (\w+)[\s:{]/gm)].map((m) => m[1]);
const present = [...new Set(rawClasses)];
if (present.length < 15) throw new Error(`declarable.h 实体应 ≥15, 实得 ${present.length}: ${present.join(",")}`);
if (!classes.Declarable && !classes.DeclarableBase) {
  // 实体定义可能全为自由函数 (打印/重载), 登记即可
  console.error(`declarable.cc 无类方法定义头, 全为自由函数: ${defs.length}`);
}

// cc 侧打印函数 (PrintDeclarable 体系)
const prints = (ccSrc.match(/std::ostream& operator<</g) || []).length;

let rust = banner("src/torque/declarable.cc.js", "src/torque/declarable.{h,cc}", "可声明实体族清单与定义头对数");

rust += `
/// torque declarable 实体族登记 (原版 declarable.h class 集, ${present.length} 项)
pub mod declarable_kinds {
  pub const KINDS: [&str; ${present.length}] = [${present.map((k) => `"${k}"`).join(", ")}];
}

/// declarable.cc 方法定义头清单 (对数基准, 修订 declarable.rs 时对照)。
${Object.entries(classes).length ? `/// 类 → 方法定义数: ${Object.entries(classes).map(([k, v]) => `${k}=${v}`).join(", ")}` : "/// 全为自由函数/运算符定义。"}

`;
for (const d of defs) {
  rust += `// ${d}\n`;
}
rust += `
pub mod declarable_cc_skel {
  /// 定义头总数对数守卫
  pub const DEF_COUNT: usize = ${defs.length};
  /// operator<< 打印定义数
  pub const PRINT_OVERLOADS: usize = ${prints};
}
`;

const p = writeSrc("v8-torque", "declarable_cc_generated.rs", rust);
console.log(`written ${p}; 定义头=${defs.length}, 实体=${present.length}, 打印=${prints}`);
