#!/usr/bin/env bun
// tool/src/torque/types.cc.js
// 机械提取 v8/src/torque/types.cc: Type 体系 .cc 侧方法定义头清单 (签名对数)
// (types.h 类型声明已由 tool/src/torque/types.js 产出 types.rs, 不重复)

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/torque/types.cc");

// 逐行拼接 Class::Method(...) 定义头
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
// 对数: 类名集合与数量
const classCounts = {};
for (const d of defs) {
  const cls = d.match(/(\w+)::/)[1];
  classCounts[cls] = (classCounts[cls] || 0) + 1;
}
const expectedClasses = ["Type", "UnionType", "ClassType", "StructType", "AggregateType", "BuiltinPointerType", "AbstractType", "BitFieldStructType", "Signature", "VisitResult", "Field"];
for (const c of expectedClasses) {
  if (!classCounts[c]) throw new Error(`types.cc 缺 ${c} 方法定义`);
}
if (defs.length < 60) throw new Error(`types.cc 定义头应 ≥60, 实得 ${defs.length}`);

// 关键常量/静态断言
const asserts = [...ccSrc.matchAll(/static_assert\(([^;]+)\);/g)].map((m) => m[1].replace(/\s+/g, " ").trim());
console.error(`types.cc static_assert=${asserts.length}`);

let rust = banner("src/torque/types.cc.js", "src/torque/types.cc", "Type 体系 .cc 方法定义头对数清单");

rust += `
/// torque types.cc 方法定义头清单 (对数基准, 供 Developer 修订 types.rs 对照)。
/// 类 → 方法定义数: ${Object.entries(classCounts).map(([k, v]) => `${k}=${v}`).join(", ")}

`;
for (const d of defs) {
  rust += `// ${d}\n`;
}
rust += `
/// 类型体系骨架单测锚点 (机械登记)
pub mod types_cc_skel {
  /// 定义头总数对数守卫
  pub const DEF_COUNT: usize = ${defs.length};
}
`;

const p = writeSrc("v8-torque", "types_cc_generated.rs", rust);
console.log(`written ${p}; 定义头=${defs.length}, 类=${Object.keys(classCounts).length}`);
