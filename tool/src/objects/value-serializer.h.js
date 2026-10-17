#!/usr/bin/env bun
// tool/src/objects/value-serializer.h.js — 覆盖 v8/src/objects/value-serializer.{h,cc}
// → v8-snapshot/src/value_serializer.rs (W9SNAP 席位独占; SerializationTag 线上标签表)
// 机械提取: SerializationTag 全量枚举 (定义在 .cc, 字节值 = ASCII 字符字面量)
// 巨型语义实现面 (3013 行序列化器) 登记跳过: 仅产出线上格式 tag 表。

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/objects/value-serializer.cc");

// 去注释但保留条目注释提炼 (tag 含义注释保留为 doc)
function stripComments(src) {
  return src.replace(/\/\*[\s\S]*?\*\//g, "").replace(/^[ \t]*\/\/.*$/gm, "");
}

const enumRe = ccSrc.match(/enum class SerializationTag : uint8_t\s*\{([\s\S]*?)\n\};/);
if (!enumRe) throw new Error("未找到 SerializationTag 枚举");
const enumBody = enumRe[1];

// 逐条目解析 (保留原注释作为文档)
const entries = [];
let curDoc = [];
for (const rawLine of enumBody.split("\n")) {
  const line = rawLine.trim();
  if (!line) continue;
  if (line.startsWith("//")) {
    const doc = line.replace(/^\/\/\s?/, "");
    if (doc) curDoc.push(doc);
    continue;
  }
  const m = line.match(/^(k\w+)\s*=\s*('(?:[^'\\]|\\.)*'|-?(?:0x[0-9A-Fa-f]+|\d+))(?:\s*,)?$/) || line.match(/^(k\w+)(?:\s*,)?$/);
  if (!m) {
    if (line === "") continue;
    throw new Error(`SerializationTag 条目不可解析: ${line}`);
  }
  const name = m[1];
  let value = null;
  if (m[2] !== undefined) {
    const v = m[2];
    if (v.startsWith("'")) {
      // 字符字面量 → 字节值
      const inner = v.slice(1, -1);
      if (inner === "\\0") value = 0;
      else if (inner === "\\\\") value = 0x5c; // '\\' 字面量 = 反斜杠
      else if (inner === "\\'") value = 0x27; // '\'' 字面量 = 单引号
      else if (inner.startsWith("\\")) throw new Error(`未处理的字符转义: ${v}`);
      else value = inner.charCodeAt(0);
    } else {
      value = Number(v);
    }
  }
  entries.push({ name, value, doc: curDoc.join(" ") });
  curDoc = [];
}
if (curDoc.length) throw new Error("枚举尾部悬挂注释, 解析不完整");

// 对数: 原版枚举条目数 (逐行扫描无未解析行; 59 = 45 常规 + 14 legacy 保留)
const KNOWN_COUNT = 59;
if (entries.length !== KNOWN_COUNT) throw new Error(`SerializationTag 条数 ${entries.length} != ${KNOWN_COUNT}`);

// 命名唯一性
const names = entries.map((e) => e.name);
if (new Set(names).size !== names.length) throw new Error("SerializationTag 名字重复");

// 字节值唯一性 (别名除外 — 本枚举无别名条目)
const withValue = entries.filter((e) => e.value !== null);
const vals = withValue.map((e) => e.value);
if (new Set(vals).size !== vals.length) throw new Error("SerializationTag 字节值重复");

// 语义锚 (线上格式, 不可变)
const anchors = {
  kVersion: 0xff,
  kPadding: 0,
  kTrue: "T".charCodeAt(0),
  kFalse: "F".charCodeAt(0),
  kBeginJSObject: "o".charCodeAt(0),
  kEndJSObject: "{".charCodeAt(0),
  kHostObject: 0x5c,
};
for (const [k, v] of Object.entries(anchors)) {
  const e = entries.find((x) => x.name === k);
  if (!e || e.value !== v) throw new Error(`锚失配 ${k}: 期望 ${v}`);
}

const pascal = (s) => {
  const c = s.replace(/^k/, "");
  return c.charAt(0).toUpperCase() + c.slice(1);
};

let rust = banner("src/objects/value-serializer.h.js", "src/objects/value-serializer.{h,cc}", "结构化克隆线上格式 SerializationTag 全量表");
rust += `//! SerializationTag: 结构化序列化 (structured clone) 线上格式标签。
//!
//! 数值即 ASCII 字节 (kVersion=0xFF 除外)。语义实现面 (序列化器/反序列化器,
//! 约 3000 行) 不在本产物范围, 由 Developer 就地实现。

/// v8: src/objects/value-serializer.cc SerializationTag
/// 原版 enum class SerializationTag : uint8_t, 共 ${entries.length} 标签。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SerializationTag {
`;
for (const e of entries) {
  const v = e.value;
  const ch = v !== null && v >= 0x21 && v < 0x7f && v !== 0x5c ? ` '${String.fromCharCode(v)}'` : "";
  rust += `  /// ${e.name}${ch}${e.doc ? ` — ${e.doc}` : ""}\n  ${pascal(e.name)} = ${v === null ? "0" : v},\n`;
}
rust += `}

impl SerializationTag {
  /// 标签总数
  pub const COUNT: usize = ${entries.length};

  /// 字节反查
  #[inline]
  #[must_use]
  pub const fn from_byte(b: u8) -> Option<Self> {
    match b {
`;
for (const e of entries) {
  rust += `      ${e.value} => Some(Self::${pascal(e.name)}),\n`;
}
rust += `      _ => None,
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SerializationTag::COUNT == ${entries.length});
  assert!(SerializationTag::Version as u8 == 0xFF);
  assert!(SerializationTag::Padding as u8 == 0);
  assert!(SerializationTag::True as u8 == b'T');
  assert!(SerializationTag::False as u8 == b'F');
  assert!(SerializationTag::BeginJSObject as u8 == b'o');
  assert!(SerializationTag::EndJSObject as u8 == b'{');
  assert!(SerializationTag::HostObject as u8 == b'\\\\');
};
`;

const p = writeSrc("v8-snapshot", "value_serializer.rs", rust);
console.log(`value_serializer.rs: SerializationTag ${entries.length} 标签全量展开, 锚点全中; 序列化器语义面登记跳过`);
