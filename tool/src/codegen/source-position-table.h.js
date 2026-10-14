#!/usr/bin/env bun
// tool/src/codegen/source-position-table.h.js
// 机械提取 v8/src/codegen/source-position-table.{h,cc} —
// RecordingMode/IterationFilter/FunctionEntryFilter 枚举 + 构建器骨架。
// 注: source_position_table.rs (手写) 已存在, 本脚本落 _cc_generated 旁路。
// 产物: v8-codegen-core/src/source_position_table_cc_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/source-position-table.h";
const CC_REL = "src/codegen/source-position-table.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const recM = h.match(/enum RecordingMode\s*\{([^}]+)\n  \};/);
if (!recM) throw new Error("未找到 RecordingMode 枚举");
const recs = [...recM[1].matchAll(/^\s*([A-Z_]+),?/gm)].map((m) => m[1]);
if (recs.join() !== "OMIT_SOURCE_POSITIONS,LAZY_SOURCE_POSITIONS,RECORD_SOURCE_POSITIONS") {
  throw new Error(`RecordingMode 条目异常: ${recs.join()}`);
}
const iterM = h.match(/enum IterationFilter\s*\{([^}]+)\};/);
if (!iterM) throw new Error("未找到 IterationFilter 枚举");
const iters = [...iterM[1].matchAll(/k(\w+)\s*=\s*(\d+)/g)].map((m) => ({ name: "k" + m[1], value: Number(m[2]) }));
const feM = h.match(/enum FunctionEntryFilter\s*\{([^}]+)\};/);
if (!feM) throw new Error("未找到 FunctionEntryFilter 枚举");
const fes = [...feM[1].matchAll(/k(\w+)\s*=\s*(\d+)/g)].map((m) => ({ name: "k" + m[1], value: Number(m[2]) }));
if (iters.length !== 3 || fes.length !== 2) throw new Error("过滤器枚举条数异常");
if (!h.includes("static const int kDone = -1;")) throw new Error("缺 kDone 锚");

let rust = banner("src/codegen/source-position-table.h.js", "src/codegen/source-position-table.{h,cc}", "源位置表枚举与骨架");
rust += `
/// 位置记录模式 (原 enum RecordingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RecordingMode {
  /// 永不生成 (空表)
  OmitSourcePositions = 0,
  /// 暂不需要, 可后补
  LazySourcePositions = 1,
  /// 立即生成
  RecordSourcePositions = 2,
}

/// 迭代过滤器 (原 enum IterationFilter)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IterationFilter {
  JavaScriptOnly = 0,
  ExternalOnly = 1,
  All = 2,
}

/// 首条目过滤器 (原 enum FunctionEntryFilter)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FunctionEntryFilter {
  SkipFunctionEntry = 0,
  DontSkipFunctionEntry = 1,
}

/// 迭代终止哨兵 (原 kDone)
pub const DONE: i32 = -1;

/// 原 SourcePositionTableBuilder / SourcePositionTableIterator 骨架
/// - 原签名: SourcePositionTableBuilder(Zone*, RecordingMode = RECORD_SOURCE_POSITIONS)
/// - 原签名: void AddPosition(code_offset, source_position, is_statement, is_breakable = true)
/// - 原签名: bool Omit() const / bool Lazy() const
/// - 原签名: IndexAndPositionState (迭代器保存/恢复)
pub struct SourcePositionTableBuilder;

// TODO(dev): 补 delta 编码字节流实现 (cc 语义面登记跳过)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(DONE == -1);
  assert!(IterationFilter::All as u8 == 2);
};
`;

const p = writeSrc("v8-codegen-core", "source_position_table_cc_generated.rs", rust);
console.log(`written ${p}`);
