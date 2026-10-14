#!/usr/bin/env bun
// tool/src/heap/code-stats.h.js
// 机械提取 v8/src/heap/code-stats.{h,cc}: CodeStatistics 静态类方法台账。
// 规则: h 声明 + cc 定义逐锚断言; DEBUG 臂 (ReportCodeStatistics/ResetCodeStatistics) 单独计数。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/code-stats.h");
const cc = readV8("src/heap/code-stats.cc");

// —— 锚点断言: 声明面 (h) ——
for (const a of [
  "class CodeStatistics {",
  "static void CollectCodeStatistics(PagedSpace* space, Isolate* isolate);",
  "static void CollectCodeStatistics(OldLargeObjectSpace* space,",
  "static void ResetCodeAndMetadataStatistics(Isolate* isolate);",
  "static void ReportCodeStatistics(Isolate* isolate);",
  "static void RecordCodeAndMetadataStatistics(Tagged<HeapObject> object,",
  "static void ResetCodeStatistics(Isolate* isolate);",
]) {
  if (!h.includes(a)) throw new Error(`code-stats.h 锚点缺失: ${a}`);
}
// —— 定义面 (cc): 6 个方法全定义 ——
for (const a of [
  "void CodeStatistics::RecordCodeAndMetadataStatistics(",
  "void CodeStatistics::ResetCodeAndMetadataStatistics(",
  "void CodeStatistics::CollectCodeStatistics(PagedSpace* space,",
  "void CodeStatistics::CollectCodeStatistics(OldLargeObjectSpace* space,",
  "void CodeStatistics::ReportCodeStatistics(",
  "void CodeStatistics::ResetCodeStatistics(",
]) {
  if (!cc.includes(a)) throw new Error(`code-stats.cc 锚点缺失: ${a}`);
}
// 记账字段: cc 内 isolate 侧三计数器 (code/bytecode/external_script_source)
const counters = ["code_and_metadata_size", "bytecode_and_metadata_size", "external_script_source_size"];
for (const c of counters) {
  if ((cc.match(new RegExp(`set_${c}\\(`, "g")) || []).length < 2) {
    throw new Error(`计数器 ${c} set_ 调用数漂移`);
  }
}
console.log("CodeStatistics 方法 6 (公开 4 + DEBUG 2 + 私有 1 重复计 Collect 重载)");

let rust = banner(
  "src/heap/code-stats.h.js",
  "src/heap/code-stats.{h,cc}",
  "代码体量统计: 静态方法台账与三计数器记账骨架",
);

rust += `
/// 代码统计 (对应 class CodeStatistics, AllStatic 风格)。
/// v8: src/heap/code-stats.h CodeStatistics
pub struct CodeStatistics;

/// isolate 侧三计数器 (对应 cc 内 set_code_and_metadata_size /
/// set_bytecode_and_metadata_size / set_external_script_source_size 的宿主字段)。
/// v8: src/heap/code-stats.cc CodeStatistics::ResetCodeAndMetadataStatistics
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CodeStatsCounters {
  pub code_and_metadata_size: i32,
  pub bytecode_and_metadata_size: i32,
  pub external_script_source_size: i32,
}

impl CodeStatsCounters {
  /// 对应 ResetCodeAndMetadataStatistics: 三计数器清零
  /// v8: src/heap/code-stats.cc CodeStatistics::ResetCodeAndMetadataStatistics
  pub fn reset(&mut self) {
    self.code_and_metadata_size = 0;
    self.bytecode_and_metadata_size = 0;
    self.external_script_source_size = 0;
  }

  /// 对应 RecordCodeAndMetadataStatistics 的 Script/ExternalString 臂:
  /// external_script_source_size += 外部源串 payload 大小
  /// v8: src/heap/code-stats.cc CodeStatistics::RecordCodeAndMetadataStatistics
  pub fn add_external_script_source(&mut self, payload_size: i32) {
    self.external_script_source_size += payload_size;
  }

  /// 对应 RecordCodeAndMetadataStatistics 的 AbstractCode 臂:
  /// Code → code_and_metadata_size, 其余 (BytecodeArray) → bytecode_and_metadata_size
  /// v8: src/heap/code-stats.cc CodeStatistics::RecordCodeAndMetadataStatistics
  pub fn add_metadata_size(&mut self, is_code: bool, size_incl_metadata: i32) {
    if is_code {
      self.code_and_metadata_size += size_incl_metadata;
    } else {
      self.bytecode_and_metadata_size += size_incl_metadata;
    }
  }
}

impl CodeStatistics {
  /// 对应 static CollectCodeStatistics(PagedSpace*, Isolate*)
  /// v8: src/heap/code-stats.h CodeStatistics::CollectCodeStatistics
  pub fn collect_code_statistics_paged(_space: &mut (), _counters: &mut CodeStatsCounters) {
    unimplemented!("骨架: PagedSpace 对象迭代待空间族就位");
  }

  /// 对应 static CollectCodeStatistics(OldLargeObjectSpace*, Isolate*)
  pub fn collect_code_statistics_large(_space: &mut (), _counters: &mut CodeStatsCounters) {
    unimplemented!("骨架: OldLargeObjectSpace 对象迭代待空间族就位");
  }

  /// 对应 static ReportCodeStatistics(Isolate*) — 仅 DEBUG 构建
  pub fn report_code_statistics(_counters: &CodeStatsCounters) {
    unimplemented!("骨架: DEBUG 专用, kind/注释统计报表");
  }
}

// —— 提取不变式断言 ——
const _: () = {
  let mut c = CodeStatsCounters::default();
  c.add_metadata_size(true, 10);
  c.add_metadata_size(false, 5);
  c.add_external_script_source(3);
  assert_eq!(c.code_and_metadata_size, 10);
  assert_eq!(c.bytecode_and_metadata_size, 5);
  assert_eq!(c.external_script_source_size, 3);
  c.reset();
  assert_eq!(c, CodeStatsCounters::default());
};
`;

const p = writeSrc("v8-heap-core", "code_stats.rs.skel", rust);
console.log(`written ${p}`);
