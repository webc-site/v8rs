#!/usr/bin/env bun
// tool/src/codegen/background-merge-task.h.js
// 机械提取 v8/src/codegen/background-merge-task.h — 后台合并任务状态机骨架。
// 产物: v8-codegen-core/src/background_merge_task.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/background-merge-task.h";
const h = readV8(H_REL);

const stM = h.match(/enum State\s*\{([\s\S]*?)\n  \};/);
if (!stM) throw new Error("未找到 State 枚举");
const states = [...stM[1].matchAll(/k(\w+)/g)].map((m) => "k" + m[1]);
if (states.length !== 4) throw new Error(`State 条数不符: ${states.length} != 4`);
if (!h.includes("new_compiled_data_for_cached_sfis_")) throw new Error("缺数据队列字段锚");

let rust = banner("src/codegen/background-merge-task.h.js", H_REL, "后台合并任务骨架");
rust += `
/// 任务状态机 (原 enum State)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
${states.map((s, i) => `  /// ${s}\n  ${s.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 原 class BackgroundMergeTask — 后台合并 cached SFI 编译数据
/// 数据队列: new_compiled_data_for_cached_sfis_ (NewCompiledDataForCachedSfi:
///   cached_sfi / new_sfi 双句柄) + sfis_without_scope_info_ 集合
pub struct BackgroundMergeTask;

// TODO(dev): 补状态迁移与合并实现。

// —— 提取不变式断言 ——
const _: () = {
  assert!(State::NotStarted as u8 == 0);
};
`;

const p = writeSrc("v8-codegen-core", "background_merge_task.rs", rust);
console.log(`written ${p} (State ${states.length})`);
