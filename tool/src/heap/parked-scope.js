#!/usr/bin/env bun
// tool/src/heap/parked-scope.js
// 机械提取 v8/src/heap/parked-scope{.h,-inl.h} 族:
// ParkedScope 家族 RAII 与 parking 同步原语签名

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/parked-scope.h");
const inlSrc = readV8("src/heap/parked-scope-inl.h");

const anchors = [
  [hSrc, "class V8_NODISCARD ParkedScope {"],
  [hSrc, "class V8_NODISCARD UnparkedScope {"],
  [hSrc, "class V8_NODISCARD UnparkedScopeIfOnBackground {"],
  [hSrc, "class V8_NODISCARD ParkedMutexGuard {"],
  [hSrc, "class V8_NODISCARD ParkedRecursiveMutexGuard {"],
  [hSrc, "class V8_NODISCARD ParkedMutexGuardIf final {"],
  [hSrc, "class V8_NODISCARD ParkingConditionVariable final"],
  [hSrc, "class V8_NODISCARD ParkingSemaphore final : public base::Semaphore {"],
  [hSrc, "using base::ConditionVariable::Wait;"],
  [hSrc, "using base::Semaphore::Wait;"],
  [inlSrc, "V8_INLINE ParkedMutexGuard::ParkedMutexGuard(LocalHeap* local_heap,"],
  [inlSrc, "local_heap->ExecuteWhileParked([this]() { mutex_->Lock(); });"],
  [inlSrc, "if (!enable_mutex) return;"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

const hClasses = hSrc.match(/class V8_NODISCARD (\w+)/g) || [];
if (hClasses.length !== 8) throw new Error(`RAII 类计数 ${hClasses.length} != 8`);
console.log(`parked-scope: RAII/parking 类 ${hClasses.length}, inl 构造/析构转发见锚`);

let rust = banner(
  "src/heap/parked-scope.js",
  "src/heap/parked-scope.h, src/heap/parked-scope-inl.h",
  "驻留(parked)线程作用域与 parking 同步原语",
);

rust += `//! LocalHeap 驻留状态切换的 RAII 家族; 等待时驻留以让出执行权。
//! 语义面: ParkedScope/UnparkedScope 调 LocalHeap::Park()/Unpark(),
//! IfOnBackground 变体先判 IsBackgroundThread (inl)。

/// v8: src/heap/parked-scope.h ParkedScope — 构造 Park, 析构 Unpark
pub struct ParkedScope<'a> { pub local_heap: &'a mut LocalHeap }
/// v8: src/heap/parked-scope.h UnparkedScope — 构造 Unpark, 析构 Park
pub struct UnparkedScope<'a> { pub local_heap: &'a mut LocalHeap }
/// v8: src/heap/parked-scope.h UnparkedScopeIfOnBackground — 后台线程才解驻留
pub struct UnparkedScopeIfOnBackground<'a> { pub local_heap: &'a mut LocalHeap }
/// v8: src/heap/parked-scope.h ParkedMutexGuard — Lock 前先 Park
pub struct ParkedMutexGuard<'a> { pub mutex: &'a mut Mutex }
/// v8: src/heap/parked-scope.h ParkedRecursiveMutexGuard
pub struct ParkedRecursiveMutexGuard<'a> { pub mutex: &'a mut RecursiveMutex }
/// v8: src/heap/parked-scope.h ParkedMutexGuardIf — 条件驻留锁
pub struct ParkedMutexGuardIf<'a> { pub mutex: &'a mut Mutex }
/// v8: src/heap/parked-scope.h ParkingConditionVariable — 等待前 Park, 唤醒后 Unpark
pub struct ParkingConditionVariable { inner: ConditionVariable }
/// v8: src/heap/parked-scope.h ParkingSemaphore — 同上语义的信号量
pub struct ParkingSemaphore { inner: Semaphore }

/// v8: src/heap/parked-scope-inl.h ParkedScope::ParkedScope / ~ParkedScope
/// v8: src/heap/parked-scope-inl.h UnparkedScope::UnparkedScope / ~UnparkedScope
/// v8: src/heap/parked-scope-inl.h UnparkedScopeIfOnBackground (is_background 判断)
/// v8: src/heap/parked-scope-inl.h ParkedMutexGuard / ParkedRecursiveMutexGuard
/// v8: src/heap/parked-scope-inl.h ParkedMutexGuardIf::{Lock,Unlock}
/// v8: src/heap/parked-scope-inl.h ParkingConditionVariable::{Wait,WaitFor,ParkAndWait}
/// v8: src/heap/parked-scope-inl.h ParkingSemaphore::{Wait,WaitFor,ParkedWait,ParkedWaitFor}
`;

const p = writeSrc("v8-heap-core", "parked_scope.rs.skel", rust);
console.log(`written ${p}`);
