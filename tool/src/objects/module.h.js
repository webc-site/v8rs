#!/usr/bin/env bun
// tool/src/objects/module.h.js — 覆盖 v8/src/objects/module.{h,tq}
// → v8-objects-rest/src/module.rs
// 机械提取: Module::Status 枚举 + JSModuleNamespace in-object 字段索引枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/module.h");

// Status 枚举 (顺序敏感, 原版注释 "Order matters!")
const statusRe = hSrc.match(/enum Status\s*\{([\s\S]*?)\n\s*\};/);
if (!statusRe) throw new Error("未找到 Module::Status 枚举");
const status = statusRe[1]
  .split(",")
  .map((s) => s.replace(/\/\/.*$/gm, "").trim())
  .filter(Boolean);
if (status.length !== 8) throw new Error(`Module::Status 条数 ${status.length} != 8`);

// JSModuleNamespace in-object 字段索引枚举
const idxRe = hSrc.match(/enum\s*\{\s*\n\s*(kToStringTagIndex),\s*\n\s*(kInObjectFieldCount),\s*\n\s*\};/);
if (!idxRe) throw new Error("未找到 JSModuleNamespace in-object 索引枚举");
if (idxRe[1] !== "kToStringTagIndex" || idxRe[2] !== "kInObjectFieldCount") {
  throw new Error("JSModuleNamespace 索引枚举条目失配");
}

// kSize 公式对数: kSize = kHeaderSize + (kTaggedSize * kInObjectFieldCount)
const sizeRe = hSrc.match(
  /inline constexpr int JSModuleNamespace::kSize =\s*\n\s*JSModuleNamespace::kHeaderSize \+ \(kTaggedSize \* kInObjectFieldCount\);/,
);
if (!sizeRe) throw new Error("未找到 JSModuleNamespace::kSize 公式");

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

let rust = banner("src/objects/module.h.js", "src/objects/module.{h,tq}", "Module 状态机枚举与命名空间布局索引");
rust += `//! Module: 模块生命周期状态与命名空间对象布局索引。

/// 模块生命周期状态 (原版 Module::Status, 顺序敏感; 共 ${status.length} 态)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Status {
`;
status.forEach((s, i) => {
  rust += `  /// ${s}\n  ${pascal(s)} = ${i},\n`;
});
rust += `}

impl Status {
  /// 状态总数
  pub const COUNT: usize = ${status.length};

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
`;
status.forEach((s, i) => {
  rust += `      ${i} => Some(Self::${pascal(s)}),\n`;
});
rust += `      _ => None,
    }
  }
}

/// JSModuleNamespace in-object 字段索引 (同时是描述符索引; 原版匿名 enum)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum NamespaceField {
  /// \`toStringTag\` 字段。
  ToStringTag = 0,
}
/// in-object 字段总数 (原版 kInObjectFieldCount)。
pub const NAMESPACE_IN_OBJECT_FIELD_COUNT: usize = 1;

/// JSModuleNamespace::kSize 公式 (原版: kHeaderSize + kTaggedSize * kInObjectFieldCount)。
/// kHeaderSize = sizeof(JSModuleNamespace), 由 Developer 就地落值。
#[inline]
#[must_use]
pub const fn namespace_size(header_size: usize, tagged_size: usize) -> usize {
  header_size + tagged_size * NAMESPACE_IN_OBJECT_FIELD_COUNT
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(Status::COUNT == 8);
  assert!(Status::Unlinked as u8 == 0);
  assert!(Status::Errored as u8 == 7);
  assert!(NAMESPACE_IN_OBJECT_FIELD_COUNT == 1);
};
`;

const p = writeSrc("v8-objects-rest", "module.rs", rust);
console.log(`module.rs: Status 8 态 + 命名空间索引 2 条, 对数通过`);
