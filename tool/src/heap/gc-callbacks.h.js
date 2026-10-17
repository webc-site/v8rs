#!/usr/bin/env bun
// tool/src/heap/gc-callbacks.h.js
// 机械提取 v8/src/heap/gc-callbacks.{h,cc}: GCType 位标志与安全点回调容器骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/gc-callbacks.h");

const gcType = hSrc.match(/enum GCType \{ kLocal = 1 << 0, kShared = 1 << 1, kAll = kLocal \| kShared \};/);
if (!gcType) throw new Error("GCType 定义漂移");
if (!hSrc.includes("class GCCallbacksInSafepoint")) throw new Error("GCCallbacksInSafepoint 缺失");
if (!hSrc.includes("class GCCallbacks ")) throw new Error("GCCallbacks 容器缺失");

console.log("GCType 位标志 3 + 回调容器");

let rust = banner(
  "src/heap/gc-callbacks.h.js",
  "src/heap/gc-callbacks.{h,cc}",
  "GC 回调: GCType 位标志与安全点回调容器骨架",
);

rust += `
/// GC 回调作用域 (对应 enum GCType, 位标志: kAll = kLocal | kShared)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GCType {
  /// kLocal = 1 << 0 (仅本 isolate)
  LOCAL = 1 << 0,
  /// kShared = 1 << 1 (共享堆)
  SHARED = 1 << 1,
  /// kAll = kLocal | kShared
  ALL = (1 << 0) | (1 << 1),
}

impl GCType {
  /// 对应 Invoke 内的 gc_type 按位匹配
  #[inline]
  #[must_use]
  pub const fn matches(self, requested: Self) -> bool {
    (self as u8 & requested as u8) != 0
  }
}

/// GC 阶段 (prologue/epilogue 共用, 源自 GCType)
pub type GCCallbackType = fn(*mut ());

/// 安全点内回调容器 (对应 class GCCallbacksInSafepoint, 骨架)
#[derive(Default)]
pub struct GCCallbacksInSafepoint {
  callbacks_: Vec<(usize, usize, GCType)>,
}

impl GCCallbacksInSafepoint {
  /// 对应 Add(callback, data, gc_type)
  pub fn add(&mut self, callback: usize, data: usize, gc_type: GCType) {
    self.callbacks_.push((callback, data, gc_type));
  }

  /// 对应 Remove(callback, data): 交换删除
  pub fn remove(&mut self, callback: usize, data: usize) {
    if let Some(pos) = self.callbacks_.iter().position(|(c, d, _)| *c == callback && *d == data) {
      self.callbacks_.swap_remove(pos);
    }
  }

  /// 对应 Invoke(gc_type)
  pub fn invoke(&self, gc_type: GCType) {
    for (_, _, t) in &self.callbacks_ {
      let _ = t.matches(gc_type);
      unimplemented!("骨架: 实际调用待 Developer 修订");
    }
  }

  /// 对应 IsEmpty()
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.callbacks_.is_empty()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(GCType::LOCAL as u8 == 1);
  assert!(GCType::SHARED as u8 == 2);
  assert!(GCType::ALL as u8 == 3);
  assert!(GCType::ALL.matches(GCType::LOCAL));
  assert!(GCType::LOCAL.matches(GCType::ALL));
  assert!(!GCType::LOCAL.matches(GCType::SHARED));
};
`;
;

const p = writeSrc("v8-heap-core", "gc_callbacks.rs", rust);
console.log(`written ${p}`);
