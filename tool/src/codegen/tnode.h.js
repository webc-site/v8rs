#!/usr/bin/env bun
// tool/src/codegen/tnode.h.js
// 机械提取 v8/src/codegen/tnode.{h,cc}: TNode 类型层级表 (类型 → 基类 → 机器表示)
// → v8-codegen-core/src/tnode.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/tnode.h");
const ccSrc = readV8("src/codegen/tnode.cc");

// 提取 struct X : Base { static constexpr MachineType kMachineType = MachineType::Y(); }
const entries = [];
const re =
  /struct (\w+)\s*(?:<[^>]*>)?\s*:\s*([\w:]+(?:\s*,\s*[\w:]+)*)\s*\{([^}]*)\}/g;
let m;
while ((m = re.exec(hSrc)) !== null) {
  const [, name, bases, body] = m;
  if (name === "UntaggedT" || name === "IntegralT") {
    entries.push({ name, bases: bases.trim(), machine: null });
    continue;
  }
  const mt = body.match(/static constexpr MachineType kMachineType = MachineType::(\w+)\(\);/);
  entries.push({ name, bases: bases.trim(), machine: mt ? mt[1] : null });
}
// UntaggedT/IntegralT 无基类体, 单独补
if (!entries.some((e) => e.name === "UntaggedT"))
  entries.unshift({ name: "UntaggedT", bases: "", machine: null });
if (!entries.some((e) => e.name === "IntegralT")) {
  const i = entries.findIndex((e) => e.name === "UntaggedT");
  entries.splice(i + 1, 0, { name: "IntegralT", bases: "UntaggedT", machine: null });
}

// 对数守卫: 关键类型必须在场
for (const want of ["WordT", "RawPtrT", "SandboxedPtrT", "Word32T", "Int32T", "Uint32T",
  "Int64T", "Uint64T", "IntPtrT", "Float64T", "ExternalPointerT"]) {
  if (!entries.some((e) => e.name === want)) throw new Error(`缺 TNode 类型 ${want}`);
}
if (entries.length < 30) throw new Error(`类型层级过少: ${entries.length}`);
// 每个带机器表示的必须有 MachineType
for (const e of entries) {
  if (e.machine === undefined) throw new Error(`类型 ${e.name} MachineType 解析失败`);
}

// using 别名 (TaggedT 按压缩指针/沙箱分支二选一)
const usingAliases = [...hSrc.matchAll(/using (\w+) = (\w+);/g)].map(
  ([, name, target]) => ({ name, target }),
);
const taggedAliases = usingAliases.filter((a) => a.name === "TaggedT");
if (taggedAliases.length !== 2) throw new Error(`TaggedT 别名分支 ${taggedAliases.length} != 2`);

if (!ccSrc.includes("MachineTypeOf<ExternalReference>::value")) throw new Error("tnode.cc 缺 MachineTypeOf 特化定义");

function rustRows() {
  return entries
    .map((e) => {
      const base = e.bases || "";
      const mach = e.machine ? e.machine : "";
      return `  (${JSON.stringify(e.name)}, ${JSON.stringify(base)}, ${JSON.stringify(mach)}),`;
    })
    .join("\n");
}

let rust = banner(
  "src/codegen/tnode.h.js",
  "src/codegen/tnode.{h,cc}",
  "TNode 类型层级表 (phantom 类型 → 机器表示映射)",
);

rust += `
// 三元组: (类型名, 直接基类, MachineType 构造名; 空串 = 无对应项)。
// 原版以 C++ 继承 + 幻影类型承载静态机器表示; Rust 侧由 Developer 定型为
// 枚举或 marker trait, 本表保证层级与表示映射 1:1。

/// TNode 类型层级表 (按原版声明顺序)
pub const TNODE_TYPE_HIERARCHY: &[(&str, &str, &str)] = &[
${rustRows()}
];

/// 查类型机器表示 (空返回 None)
#[inline]
#[must_use]
pub fn machine_type_of(name: &str) -> Option<&'static str> {
  TNODE_TYPE_HIERARCHY
    .iter()
    .find(|(n, _, m)| *n == name && !m.is_empty())
    .map(|(_, _, m)| *m)
}

/// 查类型直接基类 (根返回 None)
#[inline]
#[must_use]
pub fn base_of(name: &str) -> Option<&'static str> {
  TNODE_TYPE_HIERARCHY
    .iter()
    .find(|(n, b, _)| *n == name && !b.is_empty())
    .map(|(_, b, _)| *b)
}

/// using 别名 (原版 TaggedT 按条件分支二选一; TrustedPointerT 派生)
pub const TNODE_USING_ALIASES: &[(&str, &str)] = &[
${usingAliases.map((a) => `  (${JSON.stringify(a.name)}, ${JSON.stringify(a.target)}),`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () => {
  assert!(TNODE_TYPE_HIERARCHY.len() == ${entries.length});
  assert!(TNODE_USING_ALIASES.len() == ${usingAliases.length});
  assert!(TNODE_TYPE_HIERARCHY[0].0 == "UntaggedT");
  assert!(machine_type_of("Int32T") == Some("Int32"));
  assert!(machine_type_of("Float64T") == Some("Float64"));
  assert!(base_of("IntegralT") == Some("UntaggedT"));
  assert!(machine_type_of("UntaggedT").is_none());
};
`;

const p = writeSrc("v8-codegen-core", "tnode.rs", rust);
console.log(`tnode: 类型 ${entries.length} 项 (带机器表示 ${entries.filter((e) => e.machine).length}) → ${p}`);
