#!/usr/bin/env bun
// tool/src/zone/accounting-allocator.h.js
// 机械提取 v8/src/zone/accounting-allocator.{h,cc}: 段分配器/追踪虚面/压缩分支

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/zone/accounting-allocator.h");
const ccSrc = readV8("src/zone/accounting-allocator.cc");

// 1. 追踪虚面对数
const traceFns = [...hSrc.matchAll(/virtual void (Trace\w+Impl)\(/g)].map((m) => m[1]);
if (traceFns.join() !== "TraceZoneCreationImpl,TraceZoneDestructionImpl,TraceAllocateSegmentImpl") {
  throw new Error(`追踪虚面漂移: ${traceFns.join()}`);
}

// 2. 分配/回收入口
for (const k of ["AllocateSegment(size_t bytes)", "ReturnSegment(Segment* memory)"]) {
  if (!hSrc.includes(k)) throw new Error(`accounting-allocator.h 缺 ${k}`);
}

// 3. cc 侧平台分支 (32 位保留区间收缩 + VirtualMemory 保留路径)
const has32BitNote = ccSrc.includes("For 32-bit platforms, use smaller reservation size");
if (!has32BitNote) throw new Error("缺 32 位保留收缩分支登记");
const hasVm = ccSrc.includes("VirtualMemory");
if (!hasVm) throw new Error("缺 VirtualMemory 保留路径");

// 4. cc 定义头对数
function collectDefs(src, prefix) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    const l = ls[i];
    if (!new RegExp(`^[\\w:<>,*&\\s]*\\b${prefix}::~?\\w+\\(`).test(l)) continue;
    if (/^\s*\/\//.test(l)) continue;
    let sig = l;
    let j = i;
    while (!/\{\}?\s*$|;\s*$/.test(sig) && j + 1 < ls.length) sig += " " + ls[++j].trim();
    sig = sig.replace(/\{\s*$/, "").replace(/\{\}\s*$/, "").trim();
    if (sig.endsWith(";")) continue;
    out.push(sig);
  }
  return out;
}
const ccDefs = collectDefs(ccSrc, "AccountingAllocator");
if (ccDefs.length !== 4) throw new Error(`accounting-allocator.cc 定义头应 4, 实得 ${ccDefs.length}`);

let rust = banner("src/zone/accounting-allocator.h.js", "src/zone/accounting-allocator.{h,cc}", "段记账分配器骨架");

rust += `
/// AccountingAllocator 签名骨架 (语义见 crate 内手写 zone/accounting.rs)
pub mod accounting_allocator_skel {
  /// 分配段 (原版 AllocateSegment; 尺寸夹在 [kMinimumSegmentSize, kMaximumSegmentSize])
  pub fn allocate_segment(_bytes: usize) -> usize {
    unimplemented!("骨架")
  }
  /// 回收段 (原版 ReturnSegment; DEBUG 下先 zap)
  pub fn return_segment(_segment: usize) {
    unimplemented!("骨架")
  }
  /// 追踪虚面 (原版 Trace* 三件套, 默认空实现, 精确统计构建覆写)
  pub const TRACE_IMPLS: [&str; 3] = ["TraceZoneCreationImpl", "TraceZoneDestructionImpl", "TraceAllocateSegmentImpl"];
  /// 平台分支登记: 32 位宿主缩小保留区间; VirtualMemory 保留 + 重试
  pub const PLATFORM_BRANCHES: [&str; 3] = ["32bit-reservation", "VirtualMemory", "retry"];
}

// accounting-allocator.cc 定义头清单 (对数基准, ${ccDefs.length} 项):
${ccDefs.map((s) => "// " + s).join("\n")}
`;

const p = writeSrc("v8-zone", "zone/accounting_generated.rs", rust);
console.log(`written ${p}; 追踪=${traceFns.length}, cc定义=${ccDefs.length}`);
