// bounds.h → v8-base/src/bounds.rs 转换脚本
// 规则: 提取 IsInRange/IsInHalfOpenRange/IsInBounds/ClampToBounds 四个模板的
// 判定表达式, 产出 Rust 泛型骨架 (语义就地修订)
import { readV8, writeSrc, banner, lines } from "../../lib.js";

const src = readV8("src/base/bounds.h");

// 圈定四个函数的原文行区间 (规则区证据)
const regions = {
  isInRange: lines(src, 16, 28),
  isInRangePtr: lines(src, 30, 37),
  isInHalfOpenRange: lines(src, 44, 56),
  isInBounds: lines(src, 61, 65),
  clampToBounds: lines(src, 70, 82),
};

const body = `${banner("src/base/bounds.js", "src/base/bounds.h")}
//! 规则区原文行区间: ${JSON.stringify(Object.keys(regions))}

`;

writeSrc("v8-base", "bounds.rs.skel", body + Object.entries(regions).map(([k, v]) => `// ---- ${k} ----\n${v.join("\n")}`).join("\n\n"));
console.log("bounds.js: 骨架已产出 v8-base/src/bounds.rs.skel");
