#!/usr/bin/env bun
// tool/src/heap/memory-measurement.js
// 机械提取 v8/src/heap/memory-measurement{.h,.cc,-inl.h} 族:
// MemoryMeasurement / NativeContextInferrer / NativeContextStats

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/memory-measurement.h");
const ccSrc = readV8("src/heap/memory-measurement.cc");
const inlSrc = readV8("src/heap/memory-measurement-inl.h");

const anchors = [
  [hSrc, "class MemoryMeasurement {"],
  [hSrc, "static const int kGCTaskDelayInSeconds = 10;"],
  [hSrc, "bool EnqueueRequest(std::unique_ptr<v8::MeasureMemoryDelegate> delegate,"],
  [hSrc, "std::vector<Address> StartProcessing();"],
  [hSrc, "void FinishProcessing(const NativeContextStats& stats);"],
  [hSrc, "void ScheduleReportingTask();"],
  [hSrc, "struct Request {"],
  [hSrc, "class V8_EXPORT_PRIVATE NativeContextInferrer {"],
  [hSrc, "class V8_EXPORT_PRIVATE NativeContextStats {"],
  [hSrc, "size_t Get(Address context) const {"],
  [hSrc, "bool Empty() const { return size_by_context_.empty(); }"],
  [ccSrc, "bool MemoryMeasurement::EnqueueRequest("],
  [ccSrc, "MemoryMeasurement::DefaultDelegate("],
  [inlSrc, "NativeContextInferrer::Infer("],
  [inlSrc, "NativeContextStats::IncrementSize("],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

const delay = parseInt(hSrc.match(/kGCTaskDelayInSeconds = (\d+);/)[1], 10);
if (delay !== 10) throw new Error("kGCTaskDelayInSeconds 漂移");
console.log(`memory-measurement: 类 3, kGCTaskDelayInSeconds=${delay}, Request 字段 7`);

let rust = banner(
  "src/heap/memory-measurement.js",
  "src/heap/memory-measurement.h, src/heap/memory-measurement.cc, src/heap/memory-measurement-inl.h",
  "measureMemory() 实现: 请求队列与原生上下文计量",
);

rust += `/// v8: src/heap/memory-measurement.h MemoryMeasurement::Request
pub struct Request {
  /// v8: delegate / contexts(WeakFixedArray) / sizes / shared / wasm_code / wasm_metadata / timer
  pub fields_todo: (),
}

/// v8: src/heap/memory-measurement.h MemoryMeasurement
pub struct MemoryMeasurement {
  /// v8: received_ / processing_ / done_ (三个 list<Request>)
  /// v8: isolate_ / task_runner_
  /// v8: reporting_task_pending_ / delayed_gc_task_pending_ / eager_gc_task_pending_
  pub flags: u8,
}

impl MemoryMeasurement {
  /// v8: kGCTaskDelayInSeconds = 10
  pub const K_GC_TASK_DELAY_IN_SECONDS: i32 = ${delay};

  /// v8: EnqueueRequest — 入队 + ScheduleGCTask(execution)
  pub fn enqueue_request(&mut self, delegate: MeasureMemoryDelegate, execution: MeasureMemoryExecution, contexts: &[Handle<NativeContext>]) -> bool { todo!() }
  /// v8: StartProcessing — processing_ = move(received_), 返回 context 地址集
  pub fn start_processing(&mut self) -> Vec<Address> { todo!() }
  /// v8: FinishProcessing(stats) — 汇总 sizes 并可能触发上报
  pub fn finish_processing(&mut self, stats: &NativeContextStats) { todo!() }
  /// v8: DefaultDelegate — Promise 解析为 {bytes} 的默认委托
  pub fn default_delegate(isolate: &v8::Isolate, context: v8::Local<v8::Context>, resolver: v8::Local<v8::Promise::Resolver>, mode: v8::MeasureMemoryMode) -> Box<MeasureMemoryDelegate> { todo!() }
  /// v8: ScheduleReportingTask / ReportResults / ScheduleGCTask /
  ///     IsGCTaskPending / SetGCTaskPending / SetGCTaskDone / NextGCTaskDelayInSeconds
}

/// v8: src/heap/memory-measurement.h NativeContextInferrer
/// Infer(map, object, &native_context): 由 map/对象类型推断归属 context;
/// shared context 时做更多工作。
pub struct NativeContextInferrer;

/// v8: src/heap/memory-measurement.h NativeContextStats
pub struct NativeContextStats {
  /// v8: size_by_context_ — context → 累计字节
  pub size_by_context: std::collections::HashMap<Address, usize>,
}

impl NativeContextStats {
  /// v8: Get — 无记录返回 0
  pub fn get(&self, context: Address) -> usize {
    self.size_by_context.get(&context).copied().unwrap_or(0)
  }
  /// v8: Empty
  pub fn empty(&self) -> bool { self.size_by_context.is_empty() }
  /// v8: Clear / Merge
  /// v8: IncrementSize / IncrementExternalSize / HasExternalBytes (-inl.h)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(MemoryMeasurement::K_GC_TASK_DELAY_IN_SECONDS == 10);
};
`;

const p = writeSrc("v8-heap-core", "memory_measurement.rs.skel", rust);
console.log(`written ${p}`);
