#!/usr/bin/env bun
// tool/src/torque/type-oracle.cc.js
// 机械提取 v8/src/torque/type-oracle.{h,cc}: 工厂/谓词方法清单 (签名对数)
// (既有 tool/src/torque/types.js 已产 type_oracle.rs, 此处为 .cc 对数清单, 落新文件)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/torque/type-oracle.h");
const ccSrc = readV8("src/torque/type-oracle.cc");

// 头侧静态方法面 (工厂 Get*/谓词 Is*/转换*)
const staticFns = [...hSrc.matchAll(/\n  static [^\n]*?\b((?:Get|Create|Match)\w+)\(/g)].map((m) => m[1]);
const factories = staticFns.filter((f) => f.startsWith("Get"));
const others = staticFns.filter((f) => !f.startsWith("Get"));
if (factories.length !== 74) throw new Error(`TypeOracle 工厂应 74, 实得 ${factories.length}`);
if (others.length < 1) throw new Error(`TypeOracle 非 Get 静态面应 ≥1, 实得 ${others.length}: ${others.join(",")}`);

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
if (defs.length !== 5) throw new Error(`type-oracle.cc 定义头应 5, 实得 ${defs.length}`);

let rust = banner("src/torque/type-oracle.cc.js", "src/torque/type-oracle.{h,cc}", "TypeOracle 方法面对数清单");

rust += `
/// TypeOracle 静态方法面 (原版 type-oracle.h)
pub mod type_oracle_surface {
  /// 工厂 (Get* 前缀, ${factories.length} 项)
  pub const FACTORIES: [&str; ${factories.length}] = [${factories.map((f) => `"${f}"`).join(", ")}];
  /// 非工厂静态面 (Create*/Match* 等, ${others.length} 项)
  pub const OTHERS: [&str; ${others.length}] = [${others.map((f) => `"${f}"`).join(", ")}];
}

/// type-oracle.cc 方法定义头清单 (对数基准)
`;
for (const d of defs) {
  rust += `// ${d}\n`;
}
rust += `
pub mod type_oracle_cc_skel {
  pub const DEF_COUNT: usize = ${defs.length};
}
`;

const p = writeSrc("v8-torque", "type_oracle_cc_generated.rs", rust);
console.log(`written ${p}; 工厂=${factories.length}, 其他静态=${others.length}, 定义头=${defs.length}`);
