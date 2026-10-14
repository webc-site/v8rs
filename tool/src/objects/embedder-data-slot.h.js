#!/usr/bin/env bun
// tool/src/objects/embedder-data-slot.h.js — 覆盖 v8/src/objects/embedder-data-slot.{h,-inl.h}
// → v8-objects-rest/src/embedder_data_slot.rs
// 机械提取: 槽内偏移常量 + RawData 配置分支 (压缩开 → uint64 别名)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/embedder-data-slot.h");

const taggedOff = hSrc.match(/static constexpr int kTaggedPayloadOffset = (\d+);/);
const cppOff = hSrc.match(/static constexpr int kCppHeapPointerOffset = (\w+);/);
const align = hSrc.match(/static constexpr int kRequiredPtrAlignment = (\w+);/);
if (!taggedOff || taggedOff[1] !== "0") throw new Error("kTaggedPayloadOffset 失配");
if (!cppOff || cppOff[1] !== "kTaggedSize") throw new Error("kCppHeapPointerOffset 表达式失配");
if (!align || align[1] !== "kSmiTagSize") throw new Error("kRequiredPtrAlignment 表达式失配");

// RawData 配置分支: #ifdef V8_COMPRESS_POINTERS → using RawData = uint64_t;
// (目标压缩开; 非压缩分支为双 Address 结构体, 登记跳过)
const rawBranch = hSrc.match(/#ifdef V8_COMPRESS_POINTERS\n\s*using RawData = uint64_t;/);
if (!rawBranch) throw new Error("RawData 压缩分支失配");

// kTaggedSize = 4 → kCppHeapPointerOffset = 4; kSmiTagSize = 1 (kSmiTagSize = kTaggedSizeLog2? 实为 1: Smi tag 1 位)
// 机械: kSmiTagSize 来自 globals.h — 压缩/非压缩均为 1 (Smi tag 占 1 位)
const internalSrc = readV8("include/v8-internal.h");
const smiTag = internalSrc.match(/const int kSmiTagSize = (\d+);/);
if (!smiTag || smiTag[1] !== "1") throw new Error("kSmiTagSize 锚失配");

let rust = banner("src/objects/embedder-data-slot.h.js", "src/objects/embedder-data-slot.{h,-inl.h}", "EmbedderDataSlot 偏移常量与 RawData 分支");
rust += `//! EmbedderDataSlot: 嵌入器数据槽 (tagged 载荷 + raw 指针双段)。
//! 目标配置: 指针压缩开。

/// tagged 载荷段偏移 (原版 kTaggedPayloadOffset)。
pub const TAGGED_PAYLOAD_OFFSET: usize = 0;
/// raw 段偏移 (原版 kCppHeapPointerOffset = kTaggedSize, 压缩下 4)。
pub const CPP_HEAP_POINTER_OFFSET: usize = 4;
/// 指针对齐要求 (原版 kRequiredPtrAlignment = kSmiTagSize = 1)。
pub const REQUIRED_PTR_ALIGNMENT: usize = 1;

/// 原始嵌入数据类型 (压缩分支: 原版 \`using RawData = uint64_t\`。
/// 非压缩分支为双 Address 结构体 — 配置分支登记跳过)。
pub type RawData = u64;

// —— 提取不变式断言 ——
const _: () = {
  assert!(TAGGED_PAYLOAD_OFFSET == 0);
  assert!(CPP_HEAP_POINTER_OFFSET == 4);
  assert!(REQUIRED_PTR_ALIGNMENT == 1);
  assert!(size_of::<RawData>() == 8);
};
`;

const p = writeSrc("v8-objects-rest", "embedder_data_slot.rs", rust);
console.log(`embedder_data_slot.rs: 偏移 3 常量 + RawData 分支, 对数通过`);
