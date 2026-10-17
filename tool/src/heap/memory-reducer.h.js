#!/usr/bin/env bun
// tool/src/heap/memory-reducer.h.js
// 机械提取 v8/src/heap/memory-reducer.{h,cc}: 状态机 Id / EventType / State 枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/memory-reducer.h");
// 族内 .cc 实读对数
const mrCcSrc = readV8("src/heap/memory-reducer.cc");
if (!mrCcSrc.includes("GetCommitedSize(")) throw new Error("memory-reducer.cc 锚点缺失");

const idM = hSrc.match(/enum Id \{([^}]*)\};/);
if (!idM) throw new Error("未找到 enum Id");
const ids = idM[1].split(",").map((s) => s.trim()).filter(Boolean);
if (ids.join() !== "kUninit,kDone,kWait,kRun") throw new Error(`Id 漂移: ${ids}`);

const evM = hSrc.match(/enum EventType \{([^}]*)\};/);
if (!evM) throw new Error("未找到 enum EventType");
const events = evM[1].split(",").map((s) => s.trim()).filter(Boolean);
if (events.join() !== "kTimer,kMarkCompact,kPossibleGarbage") throw new Error(`EventType 漂移: ${events}`);

// State 结构锚点
if (!hSrc.includes("class State")) throw new Error("未找到 class State");

console.log(`Id ${ids.length}, EventType ${events.length}`);

let rust = banner(
  "src/heap/memory-reducer.h.js",
  "src/heap/memory-reducer.{h,cc}",
  "内存回收器状态机枚举",
);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

rust += `
/// 状态机状态 (对应 enum Id)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Id {
`;

ids.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

/// 触发事件 (对应 enum EventType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventType {
`;

events.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(Id::UNINIT == 0);
  assert!(Id::RUN == 3);
  assert!(EventType::TIMER == 0);
  assert!(EventType::POSSIBLE_GARBAGE == 2);
};
`;
;

const p = writeSrc("v8-heap-gc", "memory_reducer_enums.rs", rust);
console.log(`written ${p}`);
