#!/usr/bin/env bun
// tool/src/heap/heap-layout-tracer.h.js
// 机械提取 v8/src/heap/heap-layout-tracer.{h,cc}:
// GCPrologue/GCEpilogue 回调台账 + TypeToCollectorName GCType→收集器名常量表。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/heap-layout-tracer.h");
const cc = readV8("src/heap/heap-layout-tracer.cc");

for (const a of [
  "class HeapLayoutTracer : AllStatic {",
  "static void GCProloguePrintHeapLayout(v8::Isolate* isolate,",
  "static void GCEpiloguePrintHeapLayout(v8::Isolate* isolate,",
  "static void PrintMemoryChunk(std::ostream& os, const BasePage& chunk,",
  "static void PrintHeapLayout(std::ostream& os, Heap* heap);",
]) {
  if (!h.includes(a)) throw new Error(`heap-layout-tracer.h 锚点缺失: ${a}`);
}
// cc: TypeToCollectorName 4 行 switch 表逐锚
const table = cc.match(/constexpr const char\* TypeToCollectorName\(v8::GCType gc_type\) \{([\s\S]*?)\n\}/);
if (!table) throw new Error("TypeToCollectorName 定义缺失");
for (const a of [
  "case kGCTypeScavenge:",
  'return "Scavenger";',
  "case kGCTypeMarkSweepCompact:",
  'return "Mark-Compact";',
  "case kGCTypeMinorMarkSweep:",
  'return "Minor Mark-Sweep";',
  'return "Unknown collector";',
]) {
  if (!table[1].includes(a)) throw new Error(`TypeToCollectorName 表项缺失: ${a}`);
}
for (const a of [
  "void HeapLayoutTracer::GCProloguePrintHeapLayout(",
  "void HeapLayoutTracer::GCEpiloguePrintHeapLayout(",
  "void HeapLayoutTracer::PrintMemoryChunk(",
  "void HeapLayoutTracer::PrintHeapLayout(",
]) {
  if (!cc.includes(a)) throw new Error(`heap-layout-tracer.cc 锚点缺失: ${a}`);
}
console.log("HeapLayoutTracer 方法 4 + 收集器名表 4");

let rust = banner(
  "src/heap/heap-layout-tracer.h.js",
  "src/heap/heap-layout-tracer.{h,cc}",
  "堆布局打印: GCType→收集器名表与回调台账",
);

rust += `
/// 对应 TypeToCollectorName 的 GCType 枚举 (include/v8-callbacks.h, 此处按
/// switch 臂重建)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GCType {
  SCAVENGE,
  MARK_SWEEP_COMPACT,
  MINOR_MARK_SWEEP,
}

/// GCType → 收集器名 (对应 constexpr TypeToCollectorName, 未知 → "Unknown collector")。
/// v8: src/heap/heap-layout-tracer.cc TypeToCollectorName
#[must_use]
pub fn type_to_collector_name(gc_type: GCType) -> &'static str {
  match gc_type {
    GCType::SCAVENGE => "Scavenger",
    GCType::MARK_SWEEP_COMPACT => "Mark-Compact",
    GCType::MINOR_MARK_SWEEP => "Minor Mark-Sweep",
  }
}

/// 堆布局打印器 (对应 class HeapLayoutTracer : AllStatic)。
/// v8: src/heap/heap-layout-tracer.h HeapLayoutTracer
pub struct HeapLayoutTracer;

impl HeapLayoutTracer {
  /// 对应 GCProloguePrintHeapLayout (gc 回调签名)
  /// v8: src/heap/heap-layout-tracer.cc GCProloguePrintHeapLayout
  pub fn gc_prologue_print_heap_layout(_isolate: (), _gc_type: GCType, _flags: ()) {
    unimplemented!("骨架: PrintHeapLayout 待空间迭代就位");
  }

  /// 对应 GCEpiloguePrintHeapLayout
  /// v8: src/heap/heap-layout-tracer.cc GCEpiloguePrintHeapLayout
  pub fn gc_epilogue_print_heap_layout(_isolate: (), _gc_type: GCType, _flags: ()) {
    unimplemented!("骨架: PrintHeapLayout 待空间迭代就位");
  }

  /// 对应 PrintMemoryChunk(os, chunk, owner_name)
  /// v8: src/heap/heap-layout-tracer.cc PrintMemoryChunk
  pub fn print_memory_chunk(_os: &mut String, _chunk: (), _owner_name: &str) {
    unimplemented!("骨架: BasePage 遍历待页族就位");
  }

  /// 对应 PrintHeapLayout(os, heap)
  /// v8: src/heap/heap-layout-tracer.cc PrintHeapLayout
  pub fn print_heap_layout(_os: &mut String, _heap: ()) {
    unimplemented!("骨架: heap->space() 迭代待堆族就位");
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(type_to_collector_name(GCType::SCAVENGE), "Scavenger");
  assert_eq!(type_to_collector_name(GCType::MARK_SWEEP_COMPACT), "Mark-Compact");
  assert_eq!(type_to_collector_name(GCType::MINOR_MARK_SWEEP), "Minor Mark-Sweep");
};
`;

const p = writeSrc("v8-heap-core", "heap_layout_tracer.rs.skel", rust);
console.log(`written ${p}`);
