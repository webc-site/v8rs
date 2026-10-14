#!/usr/bin/env bun
// tool/src/heap/trusted-range.js
// 机械提取 v8/src/heap/trusted-range{.h,.cc} 族:
// TrustedRange::InitReservation — 沙箱外 4GB 对齐保留区

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/trusted-range.h");
const ccSrc = readV8("src/heap/trusted-range.cc");

const anchors = [
  [hSrc, "class TrustedRange final : public VirtualMemoryCage {"],
  [hSrc, "bool InitReservation(size_t requested);"],
  [hSrc, "#endif  // V8_ENABLE_SANDBOX"],
  [ccSrc, "bool TrustedRange::InitReservation(size_t requested) {"],
  [ccSrc, "DCHECK_LE(requested, kMaximalTrustedRangeSize);"],
  [ccSrc, "DCHECK_GE(requested, kMinimumTrustedRangeSize);"],
  [ccSrc, "const size_t kPageSize = NormalPage::kPageSize;"],
  [ccSrc, "const size_t base_alignment = size_t{4} * GB;"],
  [ccSrc, "params.base_alignment = base_alignment;"],
  [ccSrc, "params.permissions = PageAllocator::Permission::kNoAccess;"],
  [ccSrc, "bool success = VirtualMemoryCage::InitReservation(params);"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

console.log(`trusted-range: 类 1, 方法 1 (InitReservation), 4GB 对齐 + 零页保留`);

let rust = banner(
  "src/heap/trusted-range.js",
  "src/heap/trusted-range.h, src/heap/trusted-range.cc",
  "可信对象地址区间 (沙箱外, 每 isolate group 一个)",
);

rust += `#![cfg(feature = "sandbox")] // 原文 #ifdef V8_ENABLE_SANDBOX

/// v8: src/heap/trusted-range.h TrustedRange (VirtualMemoryCage 派生)
pub struct TrustedRange {
  pub base: VirtualMemoryCage,
}

impl TrustedRange {
  /// v8: src/heap/trusted-range.cc TrustedRange::InitReservation(requested)
  /// - DCHECK: kMinimumTrustedRangeSize <= requested <= kMaximalTrustedRangeSize
  /// - page_size = NormalPage::kPageSize (256KB)
  /// - base_alignment = 4GB — 必须分配在 4GB 之上 (避开压缩指针误读)
  /// - start hint: RoundDown(GetRandomMmapAddr(), 4GB)
  /// - permissions = kNoAccess, 页可未初始化, 释放即不可访问
  /// - 成功后保留零页, 防压缩 nullptr 解引用
  pub fn init_reservation(&mut self, requested: usize) -> bool { todo!() }

  /// v8: TrustedRange 过程级单例 (EnsureProcessWideTrustedRange, 见 cc/h)
}

// —— 提取不变式断言 ——
const _: () = {
  // base_alignment 恒 4GB (原文 size_t{4} * GB)
};
`;

const p = writeSrc("v8-heap-core", "trusted_range.rs.skel", rust);
console.log(`written ${p}`);
