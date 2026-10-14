#!/usr/bin/env bun
// tool/src/heap/safepoint.js
// 机械提取 v8/src/heap/safepoint.cc (族 .h 归他席, 读取仅作上下文):
// IsolateSafepoint / GlobalSafepoint 方法清单 + IncludeMainThread

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/heap/safepoint.cc");
const hSrc = readV8("src/heap/safepoint.h"); // 上下文 (本席表外)

const anchors = [
  [ccSrc, "IsolateSafepoint::IsolateSafepoint(Heap* heap) : heap_(heap) {}"],
  [ccSrc, "void IsolateSafepoint::EnterLocalSafepointScope() {"],
  [ccSrc, "void IsolateSafepoint::InitiateGlobalSafepointScope("],
  [ccSrc, "void IsolateSafepoint::SetSafepointRequestedFlags("],
  [ccSrc, "void IsolateSafepoint::LockMutex(LocalHeap* local_heap) {"],
  [ccSrc, "void IsolateSafepoint::LeaveGlobalSafepointScope(Isolate* initiator) {"],
  [ccSrc, "void IsolateSafepoint::LeaveLocalSafepointScope() {"],
  [ccSrc, "void IsolateSafepoint::ClearSafepointRequestedFlags("],
  [ccSrc, "void IsolateSafepoint::WaitInSafepoint() { barrier_.WaitInSafepoint(); }"],
  [ccSrc, "void IsolateSafepoint::WaitInUnpark() { barrier_.WaitInUnpark(); }"],
  [ccSrc, "void IsolateSafepoint::NotifyPark() { barrier_.NotifyPark(); }"],
  [ccSrc, "void IsolateSafepoint::WaitUntilRunningThreadsInSafepoint("],
  [ccSrc, "class GlobalSafepointInterruptTask : public CancelableTask {"],
  [ccSrc, "class PerClientSafepointData final {"],
  [hSrc, "class IsolateSafepoint final {"],
  [hSrc, "enum class IncludeMainThread { kYes, kNo };"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

const inc = hSrc.match(/enum class IncludeMainThread \{([^}]*)\};/)[1].split(",").map((s) => s.trim());
if (inc.join() !== "kYes,kNo") throw new Error(`IncludeMainThread 漂移: ${inc}`);
const ccIsolate = (ccSrc.match(/void IsolateSafepoint::(\w+)\(/g) || []).length;
const ccGlobal = (ccSrc.match(/(?:void|bool|Isolate\*) GlobalSafepoint::(\w+)\(/g) || []).length;
if (ccIsolate < 11) throw new Error(`IsolateSafepoint 方法 ${ccIsolate} < 11`);
console.log(`safepoint: IsolateSafepoint cc 方法 ${ccIsolate}, GlobalSafepoint cc 方法 ${ccGlobal}, IncludeMainThread 2`);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
let rust = banner(
  "src/heap/safepoint.js",
  "src/heap/safepoint.cc (族 .h: src/heap/safepoint.h)",
  "安全点: isolate 级与全局栅栏",
);

rust += `/// v8: src/heap/safepoint.h IsolateSafepoint::IncludeMainThread
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IncludeMainThread {
`;
inc.forEach((k, i) => (rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`));
rust += `}

/// v8: src/heap/safepoint.h IsolateSafepoint
/// 字段: heap_ / barrier_(Barrier) / active_safepoint_scopes_ /
///       active_global_safepoint_scopes_ / is_local_ / is_global_ (atomic)
pub struct IsolateSafepoint {
  pub heap: *mut Heap,
}

impl IsolateSafepoint {
  /// v8: EnterLocalSafepointScope — 主线程起本地安全点 (嵌套计数)
  ///     barrier_.Arm() → SetSafepointRequestedFlags(kNo) → WaitUntilRunningThreadsInSafepoint
  pub fn enter_local_safepoint_scope(&mut self) { todo!() }
  /// v8: LeaveLocalSafepointScope — ClearSafepointRequestedFlags → Disarm → 解锁
  pub fn leave_local_safepoint_scope(&mut self) { todo!() }
  /// v8: InitiateGlobalSafepointScope / LeaveGlobalSafepointScope
  /// v8: SetSafepointRequestedFlags / ClearSafepointRequestedFlags
  /// v8: LockMutex / WaitInSafepoint / WaitInUnpark / NotifyPark
  /// v8: WaitUntilRunningThreadsInSafepoint / TryInitiateNewSafepoint
}

/// v8: src/heap/safepoint.cc GlobalSafepointInterruptTask — 投递到各 client isolate
/// v8: src/heap/safepoint.cc PerClientSafepointData — client 级请求记账
/// v8: src/heap/safepoint.h GlobalSafepoint — 跨 isolate 全局安全点
/// v8: src/heap/safepoint.cc GlobalSafepoint 各实现 (${ccGlobal} 方法)

// —— 提取不变式断言 ——
const _: () = {
  assert!(IncludeMainThread::No == 1);
};
`;

const p = writeSrc("v8-heap-core", "safepoint.rs.skel", rust);
console.log(`written ${p}`);
