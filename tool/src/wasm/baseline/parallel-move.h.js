#!/usr/bin/env bun
// tool/src/wasm/baseline/parallel-move.h.js
// 机械提取 v8/src/wasm/baseline/parallel-move.h 的寄存器搬移装载类别

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/baseline/parallel-move.h");

const lkMatch = hSrc.match(/enum LoadKind : uint8_t \{([\s\S]*?)\n    \};/);
if (!lkMatch) throw new Error("未找到 LoadKind");
const kinds = [];
for (const rawLine of lkMatch[1].split("\n")) {
  let line = rawLine.trim();
  if (!line) continue;
  let tail = "";
  const idx = line.indexOf("//");
  if (idx >= 0) {
    tail = line.slice(idx + 2).trim();
    line = line.slice(0, idx).trim();
  }
  if (!line || line.startsWith("}")) continue;
  const m = line.match(/^(\w+),?$/);
  if (!m) throw new Error(`无法解析 LoadKind 行: "${rawLine}"`);
  kinds.push({ name: m[1], doc: tail });
}
const EXPECTED = ["kNop", "kConstant", "kStack", "kLowHalfStack", "kHighHalfStack"];
if (kinds.map((k) => k.name).join(",") !== EXPECTED.join(",")) {
  throw new Error(`LoadKind 内容不符: ${kinds.map((k) => k.name)}`);
}

// RegisterMove / Cycle 存在性
if (!/struct RegisterMove/.test(hSrc) && !/class RegisterMove/.test(hSrc)) {
  throw new Error("未找到 RegisterMove");
}

console.log(`parallel-move.h: LoadKind=${kinds.length}`);

let rust = banner(
  "src/wasm/baseline/parallel-move.h.js",
  "src/wasm/baseline/parallel-move.h",
  "并行寄存器搬移的装载类别",
);

rust += `
/// 寄存器装载类别 (对应原版 RegisterLoad::LoadKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LoadKind {
`;
for (const [i, k] of kinds.entries()) {
  rust += `  /// ${k.name}${k.doc ? ` — ${k.doc}` : ""}\n  ${k.name.replace(/^k/, "")} = ${i},\n`;
}
rust += `}

/// 寄存器搬移请求 (对应原版 RegisterMove; 语义面待 Developer 修订)
pub struct RegisterMove;

// —— 提取不变式断言 ——
const _: () = {
  assert!(LoadKind::HighHalfStack as u8 == 4);
};
`;

const p = writeSrc("v8-wasm-liftoff", "parallel_move.rs", rust);
console.log(`written ${p}`);
