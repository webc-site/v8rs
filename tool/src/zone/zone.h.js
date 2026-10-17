#!/usr/bin/env bun
// tool/src/zone/zone.h.js
// 机械提取 v8/src/zone/zone.{h,cc}: Zone 分配常量/协议与定义头清单
// (zone.h 主体已由 crate 内手写 zone/core.rs 承接, 此处为生成基线)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/zone/zone.h");
const ccSrc = readV8("src/zone/zone.cc");

// 1. 常量机械提取
const alignM = hSrc.match(/static const size_t kAlignmentInBytes = (\d+);/);
const minM = hSrc.match(/static const size_t kMinimumSegmentSize = (\d+) \* KB;/);
const maxM = hSrc.match(/static const size_t kMaximumSegmentSize = (\d+) \* KB;/);
if (!alignM || alignM[1] !== "8") throw new Error("kAlignmentInBytes 漂移");
if (!minM || minM[1] !== "8") throw new Error("kMinimumSegmentSize 漂移");
if (!maxM || maxM[1] !== "32") throw new Error("kMaximumSegmentSize 漂移");
const KB = 1024;
const minSeg = parseInt(minM[1], 10) * KB;
const maxSeg = parseInt(maxM[1], 10) * KB;

// 2. 协议锚点
if (!hSrc.includes("position_ % kAlignmentInBytes")) throw new Error("对齐断言漂移");
if (!hSrc.includes("half-open interval [position, limit)")) throw new Error("半开区间协议漂移");
for (const k of ["Expand(", "ExpandAndAllocate(", "AllocateUnchecked(", "ReleaseSegment("]) {
  if (!hSrc.includes(k)) throw new Error(`zone.h 缺 ${k}`);
}

// 3. cc 定义头对数
function collectDefs(src, prefix) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    const l = ls[i];
    if (!new RegExp(`^[A-Za-z_][\\w:<>,*&\\s]*\\b${prefix}::~?\\w+\\(`).test(l)) continue;
    let sig = l;
    let j = i;
    while (!/\{\s*$|;\s*$/.test(sig) && j + 1 < ls.length) sig += " " + ls[++j].trim();
    sig = sig.replace(/\{\s*$/, "").trim();
    if (sig.endsWith(";")) continue;
    out.push(sig);
  }
  return out;
}
const ccDefs = collectDefs(ccSrc, "Zone");
if (ccDefs.length < 5) throw new Error(`zone.cc 定义头应 ≥5, 实得 ${ccDefs.length}`);

let rust = banner("src/zone/zone.h.js", "src/zone/zone.{h,cc}", "Zone 分配器基线");

rust += `
/// Zone 分配常量 (原版 zone.h)
pub mod zone_const {
  /// New() 返回指针按 8 字节对齐 (ASan/MIPS 均要求; 原版 kAlignmentInBytes)
  pub const ALIGNMENT_IN_BYTES: usize = 8;
  /// 段最小 8KB (原版 kMinimumSegmentSize = 8 * KB)
  pub const MINIMUM_SEGMENT_SIZE: usize = ${minSeg};
  /// 段最大 32KB (原版 kMaximumSegmentSize = 32 * KB)
  pub const MAXIMUM_SEGMENT_SIZE: usize = ${maxSeg};
}

/// Zone 协议骨架 (语义见 crate 内手写 zone/core.rs)
pub mod zone_skel {
  /// 当前段空闲区为半开区间 [position, limit) (原版注释)
  pub fn allocate(_size: usize) -> usize {
    unimplemented!("骨架")
  }
  /// 扩容: 当前段不足时开新段 (原版 Expand / ExpandAndAllocate, NOINLINE)
  pub fn expand(_size: usize) {
    unimplemented!("骨架")
  }
  /// 释放段回分配器 (原版 ReleaseSegment)
  pub fn release_segment(_segment: usize) {
    unimplemented!("骨架")
  }
  /// 统计: allocation_size_ / segment_bytes_allocated_ 双原子计数
  pub const ATOMIC_COUNTERS: [&str; 2] = ["allocation_size", "segment_bytes_allocated"];
}

// zone.cc 定义头清单 (对数基准, ${ccDefs.length} 项):
${ccDefs.map((s) => "// " + s).join("\n")}
`;

const p = writeSrc("v8-zone", "zone/zone_generated.rs", rust);
console.log(`written ${p}; 常量=3, cc定义=${ccDefs.length}`);
