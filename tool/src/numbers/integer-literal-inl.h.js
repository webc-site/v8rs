#!/usr/bin/env bun
// tool/src/numbers/integer-literal-inl.h.js
// 机械提取 v8/src/numbers/integer-literal-inl.h: IntegerLiteral 运算符签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const inlSrc = readV8("src/numbers/integer-literal-inl.h");
const hSrc = readV8("src/numbers/integer-literal.h");

// 对数守卫: -inl 三算符 + ToString
const mustInl = ["IntegerLiteral::ToString", "operator<<(", "operator+("];
for (const k of mustInl) {
  if (!inlSrc.includes(k)) throw new Error(`integer-literal-inl.h 未找到 ${k}`);
}
// 头侧字段与另三算符
if (!/bool negative_;/.test(hSrc) || !/uint64_t absolute_value_;/.test(hSrc)) {
  throw new Error("IntegerLiteral 字段漂移");
}
for (const k of ["operator==", "operator!=", "operator|("]) {
  if (!hSrc.includes(k)) throw new Error(`integer-literal.h 未找到 ${k}`);
}

let rust = banner("src/numbers/integer-literal-inl.h.js", "src/numbers/integer-literal-inl.h", "IntegerLiteral 运算骨架");

rust += `
/// IntegerLiteral 内联运算签名骨架 (原版 integer-literal-inl.h)。
/// 布局: { bool negative_; uint64_t absolute_value_; } 语义实现见手写 integer_literal.rs
pub mod integer_literal_inl_skel {
  use super::IntegerLiteralSkel;

  /// 十进制串, 负值带 '-' (原版 ToString)
  pub fn to_string(_x: &IntegerLiteralSkel) -> String {
    unimplemented!("骨架")
  }

  /// 左移: y 须非负且 < 64 (原版 operator<<, DCHECK 两个前置)
  pub fn shl(_x: &IntegerLiteralSkel, _y: &IntegerLiteralSkel) -> IntegerLiteralSkel {
    unimplemented!("骨架")
  }

  /// 加法: 同号相加 / 异号大减小取大者号 (原版 operator+)
  pub fn add(_x: &IntegerLiteralSkel, _y: &IntegerLiteralSkel) -> IntegerLiteralSkel {
    unimplemented!("骨架")
  }

  /// 相等: 逐字段 (原版 operator==, integer-literal.h)
  pub fn eq(_x: &IntegerLiteralSkel, _y: &IntegerLiteralSkel) -> bool {
    unimplemented!("骨架")
  }

  /// 按位或: 异号时按绝对值大者 (原版 operator|, integer-literal.h)
  pub fn bit_or(_x: &IntegerLiteralSkel, _y: &IntegerLiteralSkel) -> IntegerLiteralSkel {
    unimplemented!("骨架")
  }
}

/// 骨架占位结构 (字段 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerLiteralSkel {
  pub negative: bool,
  pub absolute_value: u64,
}
`;

const p = writeSrc("v8-numbers", "integer_literal_inl_generated.rs", rust);
console.log(`written ${p}; 算符=${mustInl.length + 3}`);
