#!/usr/bin/env bun
// tool/src/codegen/tick-counter.h.js
// 机械提取 v8/src/codegen/tick-counter.{h,cc} → v8-codegen-core/src/tick_counter.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/tick-counter.h");
const ccSrc = readV8("src/codegen/tick-counter.cc");

const m = (re, what, src = hSrc) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

m(/class TickCounter \{/, "TickCounter");
m(/void TickAndMaybeEnterSafepoint\(\) \{/, "TickAndMaybeEnterSafepoint");
m(/size_t CurrentTicks\(\) const \{ return ticks_; \}/, "CurrentTicks");
const kMaxTicks = parseInt(m(/constexpr size_t kMaxTicks = (\d+);/, "kMaxTicks")[1], 10);
const ccMethods = [...ccSrc.matchAll(/void TickCounter::(\w+)\(/gm)].map((x) => x[1]);
for (const want of ["AttachLocalHeap", "DetachLocalHeap"]) {
  if (!ccMethods.includes(want)) throw new Error(`.cc 缺 ${want}`);
}

let rust = banner(
  "src/codegen/tick-counter.h.js",
  "src/codegen/tick-counter.{h,cc}",
  "TickCounter 常量 + 签名骨架",
);

rust += `
/// 编译 tick 计数上限哨兵 (原版 kMaxTicks; 性能退化/编译器发散探测,
/// 取值约为实际需求的 10 倍)
pub const K_MAX_TICKS: usize = ${kMaxTicks};

/// 原版 TickCounter: 以确定性 tick 近似时间, 驱动 safepoint。
#[derive(Debug, Clone, Copy, Default)]
pub struct TickCounter {
  /// 原版 ticks_
  ticks_: usize,
  /// 原版 local_heap_: LocalHeap* (占位, 待堆侧接线)
  local_heap_: *mut (),
}

impl TickCounter {
  /// 原版 TickAndMaybeEnterSafepoint: ++ticks_; 需要 local heap 则进 safepoint
  pub fn tick_and_maybe_enter_safepoint(&mut self) {
    self.ticks_ += 1;
    debug_assert!(self.ticks_ < K_MAX_TICKS, "tick 溢出哨兵");
    // local_heap_->Safepoint() 待堆侧接线
  }

  /// 原版 AttachLocalHeap — 骨架
  pub fn attach_local_heap(&mut self, local_heap: *mut ()) {
    let _ = local_heap;
    todo!("待 Developer 修订: 对齐 tick-counter.cc AttachLocalHeap")
  }

  /// 原版 DetachLocalHeap — 骨架
  pub fn detach_local_heap(&mut self) {
    todo!("待 Developer 修订: 对齐 tick-counter.cc DetachLocalHeap")
  }

  /// 原版 CurrentTicks
  #[inline]
  #[must_use]
  pub const fn current_ticks(&self) -> usize {
    self.ticks_
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_TICKS == ${kMaxTicks});
};
`;

const p = writeSrc("v8-codegen-core", "tick_counter.rs", rust);
console.log(`tick-counter: 常量 1 / 方法 4 (cc ${ccMethods.length}) → ${p}`);
