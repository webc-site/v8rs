#!/usr/bin/env bun
// tool/src/objects/map-updater.h.js — 覆盖 v8/src/objects/map-updater.{h,cc}
// → v8-objects-maps/src/map_updater_generated.rs
// 注意: maps crate 已有手写 map_updater.rs (禁改) → 产物改落 <name>_generated.rs
// 机械提取: MapUpdater::State 状态机 + InstanceTypeChange 枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/map-updater.h");

// State 枚举 (private 段)
const stateRe = hSrc.match(/enum State\s*\{([\s\S]*?)\n\s*\};/);
if (!stateRe) throw new Error("未找到 MapUpdater::State");
const states = stateRe[1].split(",").map((s) => s.trim()).filter(Boolean);
if (states.join(",") !== "kInitialized,kAtRootMap,kAtTargetMap,kAtIntegrityLevelSource,kEnd") {
  throw new Error(`State 条目失配: ${states.join(",")}`);
}

// InstanceTypeChange
const itc = hSrc.match(/enum class InstanceTypeChange\s*\{([^}]+)\}/);
if (!itc) throw new Error("未找到 InstanceTypeChange");
const itcEntries = itc[1].split(",").map((s) => s.trim()).filter(Boolean);
if (itcEntries.join(",") !== "kTypedArrayDetaching") throw new Error(`InstanceTypeChange 条目失配: ${itcEntries.join(",")}`);

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

let rust = banner("src/objects/map-updater.h.js", "src/objects/map-updater.{h,cc}", "MapUpdater 状态机 (maps 已有手写同名模块 → _generated 落点)");
rust += `//! MapUpdater 内部状态机。
//! 本文件为生成基线; 语义修订归 \`crate::map_updater\` (手写模块, 本文件禁手改混淆)。

/// 更新流程状态 (原版 enum State, private; 共 ${states.length} 态)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum State {
`;
states.forEach((s, i) => {
  rust += `  /// ${s}\n  ${pascal(s)} = ${i},\n`;
});
rust += `}

/// 实例类型变更场景 (原版 enum class InstanceTypeChange)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InstanceTypeChange {
  /// kTypedArrayDetaching
  TypedArrayDetaching = 0,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(State::Initialized as u8 == 0);
  assert!(State::End as u8 == 4);
};
#[allow(dead_code)]
const STATE_COUNT: usize = ${states.length};
const _: () = { assert!(STATE_COUNT == 5); };
`;

const p = writeSrc("v8-objects-maps", "map_updater_generated.rs", rust);
console.log(`map_updater_generated.rs: State 5 态 + InstanceTypeChange 1, 对数通过 (落点避开手写 map_updater.rs)`);
