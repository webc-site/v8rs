#!/usr/bin/env bun
// tool/src/heap/mutable-page.h.js
// 机械提取 v8/src/heap/mutable-page.h: RememberedSetType / MarkingMode /
// ConcurrentSweepingState 枚举。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/mutable-page.h");
// 族内 .cc/-inl 实读对数
const mpInlSrc = readV8("src/heap/mutable-page-inl.h");
const mpCcSrc = readV8("src/heap/mutable-page.cc");
if (!mpInlSrc.includes("MutablePage::ClearLiveness(")) throw new Error("mutable-page-inl.h 锚点缺失");
if (!mpCcSrc.includes("MutablePage::OldGenerationPageFlags(")) throw new Error("mutable-page.cc 锚点缺失");

function enumBody(name) {
  const m = hSrc.match(new RegExp(`enum(?: class)? ${name}[^\\{]*\\{([^}]*)\\};`));
  if (!m) throw new Error(`未找到枚举 ${name}`);
  return m[1]
    .replace(/\/\/[^\n]*/g, "")
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
}

const rst = enumBody("RememberedSetType");
// 末位为 NUMBER_OF_REMEMBERED_SET_TYPES 哨兵
if (rst[rst.length - 1] !== "NUMBER_OF_REMEMBERED_SET_TYPES") throw new Error("RememberedSetType 哨兵漂移");
const rstTypes = rst.slice(0, -1);
if (rstTypes.length !== 8) throw new Error(`RememberedSetType ${rstTypes.length} 项, 期望 8`);
if (rstTypes[0] !== "OLD_TO_NEW" || rstTypes[rstTypes.length - 1] !== "SURVIVOR_TO_EXTERNAL_POINTER") {
  throw new Error("RememberedSetType 锚点漂移");
}

const markingMode = enumBody("MarkingMode");
if (markingMode.join() !== "kNoMarking,kMinorMarking,kMajorMarking") throw new Error("MarkingMode 漂移");

const sweepState = enumBody("ConcurrentSweepingState");
if (sweepState.length < 2) throw new Error(`ConcurrentSweepingState 异常: ${sweepState}`);

console.log(`RememberedSetType ${rstTypes.length}+哨兵, MarkingMode ${markingMode.length}, ConcurrentSweepingState ${sweepState.length}`);

let rust = banner(
  "src/heap/mutable-page.h.js",
  "src/heap/mutable-page.h",
  "页级枚举: 记忆集种类 / 标记模式 / 并发清扫状态",
);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

rust += `
/// 记忆集种类 (对应 enum RememberedSetType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RememberedSetType {
`;

rstTypes.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

/// 种类总数 (对应 NUMBER_OF_REMEMBERED_SET_TYPES)
pub const NUMBER_OF_REMEMBERED_SET_TYPES: u8 = ${rstTypes.length};

/// 页标记模式 (对应 enum class MarkingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MarkingMode {
`;

markingMode.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

/// 并发清扫状态 (对应 enum class ConcurrentSweepingState : intptr_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum ConcurrentSweepingState {
`;

sweepState.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(RememberedSetType::OLD_TO_NEW == 0);
  assert!(RememberedSetType::OLD_TO_NEW_BACKGROUND == 1);
  assert!(RememberedSetType::OLD_TO_OLD == 2);
  assert!(RememberedSetType::OLD_TO_SHARED == 3);
  assert!(RememberedSetType::SURVIVOR_TO_EXTERNAL_POINTER == 7);
  assert!(NUMBER_OF_REMEMBERED_SET_TYPES == 8);
  assert!(MarkingMode::NO_MARKING == 0);
  assert!(MarkingMode::MAJOR_MARKING == 2);
};
`;
;

const p = writeSrc("v8-heap-core", "mutable_page_enums.rs", rust);
console.log(`written ${p}`);
