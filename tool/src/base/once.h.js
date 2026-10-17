#!/usr/bin/env bun
// tool/src/base/once.h.js — 覆盖 v8/src/base/once.{h,cc}
// 解析 OnceType/状态枚举 (UNINITIALIZED/EXECUTING/DONE)、CallOnce 两重载与 CAS+yield 等待实现
// → v8-base/src/once.rs.skel

import { readV8, writeSrc, banner, lines } from "../../lib.js";

const h = readV8("src/base/once.h");
const cc = readV8("src/base/once.cc");

for (const anchor of [
  [h, "V8_BASE_ONCE_H_"],
  [h, "using OnceType = std::atomic<uint8_t>;"],
  [h, "ONCE_STATE_UNINITIALIZED = 0"],
  [h, "ONCE_STATE_EXECUTING_FUNCTION = 1"],
  [h, "ONCE_STATE_DONE = 2"],
  [cc, "void CallOnceImpl(OnceType* once, std::function<void()> init_func) {"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: 状态枚举
const states = [...h.matchAll(/^\s+(ONCE_STATE_\w+) = (\d+)/gm)].map((m) => ({ name: m[1], value: m[2] }));
if (states.length !== 3) throw new Error(`expect 3 states, got ${states.length}`);
// 真解析: CallOnce 重载
const overloads = [...h.matchAll(/^inline void CallOnce\(|^V8_BASE_EXPORT void CallOnce(?:Impl)?\(/gm)].length;
const implBody = lines(cc, 16, 53);

const body = `${banner("src/base/once.h.js", "src/base/once.h + .cc", "POD 单次初始化原语")}
//! v8 侧: 状态枚举 ${states.length}; CallOnce 重载 ${overloads} + Impl 1
//! Rust 策略: OnceType = AtomicU8; 整体可替换 std::sync::Once, 保留状态机供差分

/// v8: src/base/once.h OnceType = std::atomic<uint8_t> (POD, 无静态初始化器)
pub type OnceType = core::sync::atomic::AtomicU8;

/// v8: src/base/once.h enum : uint8_t
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OnceState {
    /// v8: ONCE_STATE_UNINITIALIZED = 0
    Uninitialized = ${states[0].value},
    /// v8: ONCE_STATE_EXECUTING_FUNCTION = 1
    ExecutingFunction = ${states[1].value},
    /// v8: ONCE_STATE_DONE = 2
    Done = ${states[2].value},
}

/// v8: src/base/once.h CallOnce (acquire 快路径 + Impl)
/// 语义: load(acquire) != DONE 才进 Impl
#[inline]
pub fn call_once<F: FnOnce()>(once: &OnceType, init_func: F) {
    use core::sync::atomic::Ordering;
    if once.load(Ordering::Acquire) != OnceState::Done as u8 {
        call_once_impl(once, init_func);
    }
}

/// v8: src/base/once.cc CallOnceImpl — CAS(UNINIT→EXECUTING, acq_rel) 成功者执行;
/// 失败者自旋 yield 等 DONE
pub fn call_once_impl<F: FnOnce()>(once: &OnceType, init_func: F) {
    use core::sync::atomic::Ordering;
    /// v8: 三态机 — 原文行区间 cc 16-53 见文件尾
    todo!()
}

// ---- v8 CallOnceImpl 原文 (CAS/自旋证据) ----
${implBody.map((l) => `// ${l}`).join("\n")}
`;

writeSrc("v8-base", "once.rs.skel", body);
console.log(`once.h.js: states=${states.length} overloads=${overloads}`);
