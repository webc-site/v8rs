#!/usr/bin/env bun
// tool/src/codegen/aligned-slot-allocator.h.js
// 机械提取 v8/src/codegen/aligned-slot-allocator.{h,cc}:
// 常量 + 结构布局 + 方法签名 (算法体为机械直译, 标注待 Developer 复核)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/aligned-slot-allocator.h");
const ccSrc = readV8("src/codegen/aligned-slot-allocator.cc");

// 对数: 常量与成员必须逐个找到, 找不到即 throw
function must(re, what, src = hSrc) {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

const slotSizeM = must(/static constexpr int kSlotSize = (\w+);/, "kSlotSize");
const invalidSlotM = must(/static constexpr int kInvalidSlot = (-?\d+);/, "kInvalidSlot");
// kSystemPointerSize 来自 common/globals.h = 8 (64 位目标)
if (slotSizeM[1] !== "kSystemPointerSize") throw new Error(`kSlotSize 依赖异常: ${slotSizeM[1]}`);

// .cc 方法签名清点 (对数守卫): 原版 5 个方法
const ccMethods = [...ccSrc.matchAll(/^int AlignedSlotAllocator::(\w+)\(/gm)].map((m) => m[1]);
const EXPECT_METHODS = ["NextSlot", "Allocate", "AllocateUnaligned", "Align"];
for (const want of EXPECT_METHODS) {
  if (!ccMethods.includes(want)) throw new Error(`.cc 缺方法 ${want}`);
}
if (ccMethods.length !== 4) throw new Error(`.cc 方法数 ${ccMethods.length} != 4`);

let rust = banner(
  "src/codegen/aligned-slot-allocator.h.js",
  "src/codegen/aligned-slot-allocator.{h,cc}",
  "AlignedSlotAllocator 常量/布局/签名 (算法体机械直译)",
);

rust += `
/// 对齐槽分配器: 按 1/2/4 槽组分配, 组内首槽按组大小对齐;
/// 复用对齐 padding 片段以节槽。槽 = 指针大小单位。
/// kSlotSize (原版 kSlotSize = kSystemPointerSize, 64 位目标 = 8)
pub const SLOT_SIZE: i32 = 8;

/// 无效槽哨兵 (原版 kInvalidSlot)
pub const INVALID_SLOT: i32 = ${invalidSlotM[1]};

/// 原版 AlignedSlotAllocator (字段布局机械对应 next1_/next2_/next4_/size_)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlignedSlotAllocator {
  /// 1 槽片段起点, 无则 INVALID_SLOT
  pub next1: i32,
  /// 2 对齐 2 槽片段起点, 无则 INVALID_SLOT
  pub next2: i32,
  /// 4 对齐下一 4 槽组起点, 恒有效
  pub next4: i32,
  /// 槽区大小
  pub size: i32,
}

impl Default for AlignedSlotAllocator {
  fn default() -> Self {
    Self { next1: INVALID_SLOT, next2: INVALID_SLOT, next4: 0, size: 0 }
  }
}

impl AlignedSlotAllocator {
  /// 原版 IsValid (私有静态): slot > kInvalidSlot
  #[inline]
  #[must_use]
  pub const fn is_valid(slot: i32) -> bool {
    slot > INVALID_SLOT
  }

  /// 原版 NumSlotsForWidth: (bytes + kSlotSize - 1) / kSlotSize
  #[inline]
  #[must_use]
  pub const fn num_slots_for_width(bytes: i32) -> i32 {
    (bytes + SLOT_SIZE - 1) / SLOT_SIZE
  }

  /// 原版 NextSlot(int n) const — n 限 1/2/4
  /// 机械直译自 aligned-slot-allocator.cc NextSlot
  #[must_use]
  pub const fn next_slot(&self, n: i32) -> i32 {
    if n <= 1 && Self::is_valid(self.next1) {
      return self.next1;
    }
    if n <= 2 && Self::is_valid(self.next2) {
      return self.next2;
    }
    self.next4
  }

  /// 原版 Allocate(int n) — n 限 1/2/4, 可插 padding; 返回按 n 对齐的起点
  /// 机械直译自 aligned-slot-allocator.cc Allocate
  pub fn allocate(&mut self, n: i32) -> i32 {
    let mut result = INVALID_SLOT;
    match n {
      1 => {
        if Self::is_valid(self.next1) {
          result = self.next1;
          self.next1 = INVALID_SLOT;
        } else if Self::is_valid(self.next2) {
          result = self.next2;
          self.next1 = result + 1;
          self.next2 = INVALID_SLOT;
        } else {
          result = self.next4;
          self.next1 = result + 1;
          self.next2 = result + 2;
          self.next4 += 4;
        }
      }
      2 => {
        if Self::is_valid(self.next2) {
          result = self.next2;
          self.next2 = INVALID_SLOT;
        } else {
          result = self.next4;
          self.next2 = result + 2;
          self.next4 += 4;
        }
      }
      4 => {
        result = self.next4;
        self.next4 += 4;
      }
      _ => unreachable!("n 必须为 1/2/4"),
    }
    self.size = self.size.max(result + n);
    result
  }

  /// 原版 AllocateUnaligned(int n) — 在 size_ 尾部保留 n 槽并复位片段;
  /// n=0 可用于切分槽区 (tagged/untagged 分区)。
  /// 机械直译自 aligned-slot-allocator.cc AllocateUnaligned
  pub fn allocate_unaligned(&mut self, n: i32) -> i32 {
    let result = self.size;
    self.size += n;
    match self.size & 3 {
      0 => {
        self.next1 = INVALID_SLOT;
        self.next2 = INVALID_SLOT;
        self.next4 = self.size;
      }
      1 => {
        self.next1 = self.size;
        self.next2 = self.size + 1;
        self.next4 = self.size + 3;
      }
      2 => {
        self.next1 = INVALID_SLOT;
        self.next2 = self.size;
        self.next4 = self.size + 2;
      }
      _ => {
        self.next1 = self.size;
        self.next2 = INVALID_SLOT;
        self.next4 = self.size + 1;
      }
    }
    result
  }

  /// 原版 Align(int n) — 对齐槽区, 返回 padding 槽数 (n 为 <=4 的 2 的幂)
  /// 机械直译自 aligned-slot-allocator.cc Align
  pub fn align(&mut self, n: i32) -> i32 {
    let mask = n - 1;
    let misalignment = self.size & mask;
    let padding = (n - misalignment) & mask;
    self.allocate_unaligned(padding);
    padding
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SLOT_SIZE == 8);
  assert!(INVALID_SLOT == -1);
  assert!(AlignedSlotAllocator::num_slots_for_width(8) == 1);
  assert!(AlignedSlotAllocator::num_slots_for_width(9) == 2);
};
`;

const p = writeSrc("v8-codegen-core", "aligned_slot.rs", rust);
console.log(`aligned-slot-allocator: 常量 2 / 字段 4 / 方法 4 (原版 ${ccMethods.length}) → ${p}`);
