#!/usr/bin/env bun
// tool/src/execution/local-isolate.js — LocalIsolate + ThreadLocalTop 族
// 覆盖: src/execution/local-isolate.h + local-isolate.cc + local-isolate-inl.h +
//       thread-local-top.h + thread-local-top.cc
// 产出: v8-execution-isolate/src/local_isolate.rs.skel
// 破环登记: LocalIsolate 是 Isolate 的受限视图 (后台线程用) — 经 IsolateHeapEnv 窄 trait
//   取堆, 不直持 Isolate 裸指针; 禁 dyn, 泛型单态化。
// 复跑幂等: 纯文本扫描 + 确定性输出, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/execution/local-isolate.js";
const FILES = [
  "src/execution/local-isolate.h",
  "src/execution/local-isolate.cc",
  "src/execution/local-isolate-inl.h",
  "src/execution/thread-local-top.h",
  "src/execution/thread-local-top.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[local-isolate.js] ${m}`); };

const h = txt["src/execution/local-isolate.h"];
for (const needle of ["class V8_EXPORT_PRIVATE LocalIsolate final", "LocalHeap* heap()", "roots_table()", "explicit LocalIsolate(Isolate* isolate, ThreadKind kind)"]) {
  if (!h.includes(needle)) die(`local-isolate.h 缺针 ${needle}`);
}
const tlt = txt["src/execution/thread-local-top.h"];
for (const needle of ["class ThreadLocalTop", "kSizeInBytes = 28 * kSystemPointerSize", "thread_id_", "IncrementCallDepth"]) {
  if (!tlt.includes(needle)) die(`thread-local-top.h 缺针 ${needle}`);
}

const out = [];
out.push(banner(GEN, FILES.join(", "), "LocalIsolate 受限视图; 回调经 IsolateHeapEnv 窄 trait"));
out.push(`//! 破环登记: LocalIsolate 不直持 Isolate 裸指针 — 后台任务经 IsolateHeapEnv 泛型参数取堆面。`);
out.push(``);
out.push(`// ── LocalIsolate 类面 (local-isolate.h:47; Isolate 受限视图, 后台线程用) ──`);
out.push(`//   explicit LocalIsolate(Isolate*, ThreadKind) / heap() / roots_table() / root()`);
out.push(`//   修订阶段转窄 trait 泛型 (LocalIsolate 不直持 Isolate 裸指针语义)`);
out.push(``);
out.push(`// ── ThreadLocalTop (thread-local-top.h; kSizeInBytes = 28*kSystemPointerSize; static_assert==sizeof) ──`);
out.push(``);
out.push(`// 对数: local-isolate 类面针 4 + ThreadLocalTop 针 4 全过`);

writeSrc("v8-execution-isolate", "local_isolate.rs.skel", out.join("\n") + "\n");
console.log(`OK local-isolate.js → v8-execution-isolate/src/local_isolate.rs.skel`);
