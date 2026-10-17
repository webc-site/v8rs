#!/usr/bin/env bun
// tool/src/base/platform/platform.cc.js — 覆盖 v8/src/base/platform/platform.cc
// 平台公共实现: thread_local 栈界缓存 + OS::GetCurrentThreadId 惰性线程号
// → v8-base-platform/src/platform_cc.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform.cc");

// 1. 匿名域锚点: 两个 thread_local 栈界缓存
const tlDecls = [...ccSrc.matchAll(/^thread_local void\* (\w+) = nullptr;/gm)].map((m) => m[1]);
if (tlDecls.join(",") !== "thread_stack_start,thread_reserved_stack_limit") {
  throw new Error(`thread_local 声明不符: ${tlDecls}`);
}

// 2. Stack 三静态面
const stackFns = ["GetStackStartUnchecked", "GetReservedStackLimit", "GetStackStart"];
for (const f of stackFns) {
  if (!ccSrc.includes(`Stack::StackSlot Stack::${f}()`)) throw new Error(`platform.cc 缺 Stack::${f}`);
}
// 缓存判空 + 填充模式 (各一次 Obtain* 调用)
if ((ccSrc.match(/ObtainCurrentThreadStackStart\(\)/g) || []).length !== 1) throw new Error("ObtainCurrentThreadStackStart 调用数不符");
if ((ccSrc.match(/ObtainCurrentThreadStackReservedLimit\(\)/g) || []).length !== 1) throw new Error("ObtainCurrentThreadStackReservedLimit 调用数不符");

// 3. OS::GetCurrentThreadId: static thread_local 局部缓存内部函数结果
if (!ccSrc.includes("int OS::GetCurrentThreadId() {")) throw new Error("缺 OS::GetCurrentThreadId");
if (!ccSrc.includes("static thread_local int id = GetCurrentThreadIdInternal();")) {
  throw new Error("GetCurrentThreadId 惰性缓存锚点缺失");
}
// GetStackStart 直通 GetStackStartUnchecked
if (!ccSrc.includes("Stack::StackSlot Stack::GetStackStart() { return GetStackStartUnchecked(); }")) {
  throw new Error("GetStackStart 直通锚点缺失");
}

// 4. 文件总面: 命名空间级定义 5 个 (Stack×3 + OS×1 + 匿名 2 声明)
const osDefs = (ccSrc.match(/^int OS::GetCurrentThreadId\(\)/gm) || []).length;
if (osDefs !== 1) throw new Error(`OS:: 定义数不符: ${osDefs}`);

console.log(`platform.cc: thread_local 缓存 ${tlDecls.length}, Stack 静态 ${stackFns.length}, OS:: 静态 ${osDefs}`);

let rust = banner("src/base/platform/platform.cc.js", "src/base/platform/platform.cc", "线程栈界缓存与线程号 (平台无关公共实现)");

rust += `
use std::cell::Cell;

thread_local! {
  /// 当前线程栈起始 (原版 thread_local void* thread_stack_start)
  /// v8: src/base/platform/platform.cc thread_stack_start
  static THREAD_STACK_START: Cell<usize> = const { Cell::new(0) };

  /// 当前线程保留栈下限 (原版 thread_local void* thread_reserved_stack_limit)
  /// v8: src/base/platform/platform.cc thread_reserved_stack_limit
  static THREAD_RESERVED_STACK_LIMIT: Cell<usize> = const { Cell::new(0) };
}

/// 栈操作面 (原版 class Stack 的平台无关缓存层)
/// v8: src/base/platform/platform.cc Stack::GetStackStartUnchecked
/// 首次调用经 ObtainCurrentThreadStackStart (darwin 实现:
/// pthread_get_stackaddr_np(pthread_self()), 见 platform-darwin 族) 填充
pub fn get_stack_start_unchecked() -> *mut std::ffi::c_void {
  THREAD_STACK_START.with(|c| {
    let mut p = c.get();
    if p == 0 {
      p = obtain_current_thread_stack_start() as usize;
      c.set(p);
    }
    p as *mut std::ffi::c_void
  })
}

/// v8: src/base/platform/platform.cc Stack::GetReservedStackLimit
/// 保留栈下限: 首次经 ObtainCurrentThreadStackReservedLimit 填充
/// (darwin: stackaddr - stacksize, 见 platform-darwin 族)
pub fn get_reserved_stack_limit() -> *mut std::ffi::c_void {
  THREAD_RESERVED_STACK_LIMIT.with(|c| {
    let mut p = c.get();
    if p == 0 {
      p = obtain_current_thread_stack_reserved_limit() as usize;
      c.set(p);
    }
    p as *mut std::ffi::c_void
  })
}

/// v8: src/base/platform/platform.cc Stack::GetStackStart
/// 直通 GetStackStartUnchecked (0 判空即 "Unchecked" 语义差异点)
pub fn get_stack_start() -> *mut std::ffi::c_void { get_stack_start_unchecked() }

// —— 平台钩子 (实现在各 platform-<os> 族, 此处登记签名) ——
/// v8: src/base/platform/platform-darwin.cc Stack::ObtainCurrentThreadStackStart
fn obtain_current_thread_stack_start() -> *mut std::ffi::c_void {
  unsafe { libc::pthread_get_stackaddr_np(libc::pthread_self()) as *mut std::ffi::c_void }
}
/// v8: src/base/platform/platform-darwin.cc Stack::ObtainCurrentThreadStackReservedLimit
fn obtain_current_thread_stack_reserved_limit() -> *mut std::ffi::c_void {
  unsafe {
    let addr = libc::pthread_get_stackaddr_np(libc::pthread_self()) as usize;
    let size = libc::pthread_get_stacksize_np(libc::pthread_self());
    (addr - size) as *mut std::ffi::c_void
  }
}

thread_local! {
  /// 线程号缓存 (原版 static thread_local int id = GetCurrentThreadIdInternal())
  /// v8: src/base/platform/platform.cc OS::GetCurrentThreadId
  static THREAD_ID: std::cell::OnceCell<i32> = const { std::cell::OnceCell::new() };
}

/// v8: src/base/platform/platform.cc OS::GetCurrentThreadId
/// 惰性求值一次后线程内复用; 内部函数 darwin: pthread_mach_thread_np
pub fn get_current_thread_id() -> i32 {
  THREAD_ID.with(|c| *c.get_or_init(|| unsafe {
    libc::pthread_mach_thread_np(libc::pthread_self()) as i32
  }))
}

// —— 提取不变式断言 ——
const _: () = {
  // 2 个 thread_local 栈界 + 3 个 Stack 静态 + 1 个 OS 静态
  const STACK_STATICS: usize = 3;
  const OS_STATICS: usize = 1;
  assert!(STACK_STATICS == 3 && OS_STATICS == 1);
};
`;

const p = writeSrc("v8-base-platform", "platform_cc.rs.skel", rust);
console.log(`written ${p}`);
