#!/usr/bin/env bun
// tool/src/heap/zapping.cc.js
// 机械提取 v8/src/heap/zapping.{h,cc}: zap 块纯算术规则面
// (DEBUG 门控 / 对齐断言 / 步进计数); 指针写本体留签名骨架归 Developer

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/zapping.h");
const ccSrc = readV8("src/heap/zapping.cc");
// 目标配置数值锚 (指针压缩开)
const gSrc = readV8("src/common/globals.h");

// —— 锚点核对 (禁吞账) ——
if (!ccSrc.includes("void ZapCodeBlock(Address start, int size_in_bytes)")) {
  throw new Error("ZapCodeBlock 签名锚点缺失");
}
if (!ccSrc.includes("void ZapBlock(Address start, size_t size, uintptr_t zap_value)")) {
  throw new Error("ZapBlock 签名锚点缺失");
}

// ZapCodeBlock 函数体: DEBUG 门控 + kIntSize 对齐 + 逐 int 写 kCodeZapValue
const zcbStart = ccSrc.indexOf("void ZapCodeBlock");
const zcb = ccSrc.slice(zcbStart, ccSrc.indexOf("void ZapBlock"));
if (!zcb.includes("#ifdef DEBUG")) throw new Error("ZapCodeBlock 非 DEBUG 门控");
if (!zcb.includes("IsAligned(start, kIntSize)")) throw new Error("ZapCodeBlock 对齐断言缺失");
if (!/i < size_in_bytes \/ kIntSize/.test(zcb)) throw new Error("ZapCodeBlock 步进式缺失");
if (!zcb.includes("kCodeZapValue")) throw new Error("ZapCodeBlock zap 值缺失");

// ZapBlock 函数体: 双对齐 + MemsetTagged 长度 size >> kTaggedSizeLog2
const zb = ccSrc.slice(ccSrc.indexOf("void ZapBlock"));
if (!zb.includes("ShouldZapGarbage()")) throw new Error("ZapBlock ShouldZapGarbage 缺失");
if (!zb.includes("IsAligned(start, kTaggedSize)") || !zb.includes("IsAligned(size, kTaggedSize)")) {
  throw new Error("ZapBlock 双对齐断言缺失");
}
if (!/size >> kTaggedSizeLog2/.test(zb)) throw new Error("ZapBlock 步进式缺失");

// ShouldZapGarbage 规则面 (zapping.h:18, DEBUG 真 / VERIFY_HEAP 看 flag / 否则假)
if (!hSrc.includes("inline bool ShouldZapGarbage()")) throw new Error("ShouldZapGarbage 缺失");
if (!hSrc.includes("#ifdef DEBUG") || !hSrc.includes("#ifdef VERIFY_HEAP")) {
  throw new Error("ShouldZapGarbage 门控链缺失");
}
// ZapValue() 选择规则 (zapping.h:23): clear_free_memory ? kClearedFreeMemoryValue : kZapValue
if (!hSrc.includes("v8_flags.clear_free_memory ? kClearedFreeMemoryValue : kZapValue")) {
  throw new Error("ZapValue 选择规则缺失");
}

// globals.h 目标配置支 (指针压缩 → kTaggedSize = kInt32Size = 4)
if (!gSrc.includes("constexpr int kTaggedSize = kInt32Size;")) {
  throw new Error("kTaggedSize=4 支缺失 (指针压缩目标配置)");
}
if (!gSrc.includes("constexpr int kTaggedSizeLog2 = 2;")) {
  throw new Error("kTaggedSizeLog2=2 支缺失");
}
if (!gSrc.includes("constexpr int kIntSize = sizeof(int);")) {
  throw new Error("kIntSize 定义缺失");
}

console.log("zapping.cc: 2 函数规则面 + 3 门控锚全中");

let rust = banner(
  "src/heap/zapping.cc.js",
  "src/heap/zapping.{h,cc} + src/common/globals.h (配置支)",
  "zap 块步进算术与门控规则; zap 值常量在 zapping.rs",
);

rust += `
//! ZapCodeBlock / ZapBlock 的机械规则面。
//!
//! - ZapCodeBlock 仅 DEBUG 构建有实体 (cc:#ifdef DEBUG), 对齐 kIntSize, 逐 int 写 kCodeZapValue
//! - ZapBlock 双对齐 kTaggedSize (start 与 size), MemsetTagged 长度 size >> kTaggedSizeLog2
//! - zap 值常量 (kZapValue/kCodeZapValue/kClearedFreeMemoryValue) 已在 zapping.rs, 不重复

/// 目标配置: 指针压缩开 → kTaggedSize = kInt32Size (globals.h:580)
pub const K_TAGGED_SIZE: usize = 4;
/// kTaggedSizeLog2 (globals.h:581)
pub const K_TAGGED_SIZE_LOG2: u32 = 2;
/// sizeof(int) (globals.h:450)
pub const K_INT_SIZE: i32 = 4;

/// ZapCodeBlock 写入字数 — 原版循环上界 size_in_bytes / kIntSize
/// v8: src/heap/zapping.cc:20
#[must_use]
pub const fn zap_code_block_word_count(size_in_bytes: i32) -> i32 {
  size_in_bytes / K_INT_SIZE
}

/// ZapBlock 写入槽数 — 原版 MemsetTagged 长度 size >> kTaggedSizeLog2
/// v8: src/heap/zapping.cc:32
#[must_use]
pub const fn zap_block_tagged_count(size: usize) -> usize {
  size >> K_TAGGED_SIZE_LOG2
}

/// 签名骨架: 代码块 zap (DEBUG 门控 + CodePageMemoryModificationScopeForDebugging + 逐 int 写)
/// 原版签名: void ZapCodeBlock(Address start, int size_in_bytes)
/// v8: src/heap/zapping.cc:14
pub fn zap_code_block(_start: u64, _size_in_bytes: i32) {
  todo!("就地修订: DEBUG 门控 + 代码页写权限域 + 逐 int 写 K_CODE_ZAP_VALUE")
}

/// 签名骨架: 常规块 zap (MemsetTagged 填 zap_value)
/// 原版签名: void ZapBlock(Address start, size_t size, uintptr_t zap_value)
/// v8: src/heap/zapping.cc:26
pub fn zap_block(_start: u64, _size: usize, _zap_value: u64) {
  todo!("就地修订: MemsetTagged 填充")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_TAGGED_SIZE == 4);
  assert!(K_TAGGED_SIZE_LOG2 == 2);
  assert!(K_INT_SIZE == 4);
  assert!(zap_code_block_word_count(64) == 16);
  assert!(zap_code_block_word_count(0) == 0);
  assert!(zap_block_tagged_count(96) == 24);
  assert!(zap_block_tagged_count(4) == 1);
};
`;

const p = writeSrc("v8-heap-core", "zapping_cc.skel", rust);
console.log(`written ${p}`);
