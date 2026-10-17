#!/usr/bin/env bun
// tool/src/base/numerics/ostream_operators.h.js — 覆盖 v8/src/base/numerics/ostream_operators.h
// 解析 StrictNumeric/ClampedNumeric 的 operator<< 流输出重载签名
// → v8-base/src/numerics/ostream_operators.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/ostream_operators.h");

for (const anchor of [
  "V8_BASE_NUMERICS_OSTREAM_OPERATORS_H_",
  "class ClampedNumeric",
  "class StrictNumeric",
  "operator<<",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 前置声明 (class X;) 与 operator<<(os, value) 重载
const fwdRe = /class (\w+);/g;
const forwards = [...h.matchAll(fwdRe)].map((m) => m[1]);
const opRe = /std::ostream& operator<<\(std::ostream& os, const (\w+)<T>& value\)/g;
const ops = [...h.matchAll(opRe)].map((m) => m[1]);
if (forwards.length !== 2 || ops.length !== 2) {
  throw new Error(`expect 2 fwd + 2 ops, got ${forwards.length}/${ops.length}`);
}

// Rust 对应: Debug/Display impl 骨架 (无 ostream, 走 trait)
const body = `${banner("src/base/numerics/ostream_operators.h.js", "src/base/numerics/ostream_operators.h", "Strict/ClampedNumeric 流输出 (Rust 映射为 Display)")}
//! v8 侧前置声明 ${forwards.length} 个, operator<< 重载 ${ops.length} 个
//! C++ ostream → Rust core::fmt::Display; 数值裸出 (os << static_cast<T>(value))

use core::fmt;

${ops
  .map(
    (op) => `/// v8: src/base/numerics/ostream_operators.h operator<<(std::ostream&, const ${op}<T>&)
/// 语义: 输出 static_cast<T>(value) 的裸数值
impl<T: fmt::Display> fmt::Display for super::${op === "StrictNumeric" ? "safe_conversions::StrictNumeric" : "clamped_math::ClampedNumeric"}<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 语义: write!(f, "{}", T::from(self.clone())) — 就地修订
        todo!()
    }
}
`,
  )
  .join("\n")}
`;

writeSrc("v8-base", "numerics/ostream_operators.rs.skel", body);
console.log(`ostream_operators.h.js: fwd=${forwards.length} ops=${ops.length} (${ops.join(",")})`);
