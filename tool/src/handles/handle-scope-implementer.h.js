#!/usr/bin/env bun
// tool/src/handles/handle-scope-implementer.h.js
// 机械提取 v8/src/handles/handle-scope-implementer.{h,inl.h,cc}:
// 归档块尺寸常量, 哨兵地址, 方法定义头清单

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/handles/handle-scope-implementer.h");
const inlSrc = readV8("src/handles/handle-scope-implementer-inl.h");
const ccSrc = readV8("src/handles/handle-scope-implementer.cc");
const internalsSrc = readV8("include/v8-internal.h");

// 1. HandleScopeData::kSizeInBytes = 2*ptr + 2*int32 (v8-internal.h)
const hsdM = internalsSrc.match(/kSizeInBytes =\s*\n?\s*2 \* kApiSystemPointerSize \+ 2 \* kApiInt32Size;/);
if (!hsdM) throw new Error("v8-internal.h HandleScopeData::kSizeInBytes 公式漂移");
// kApiSystemPointerSize=8, kApiInt32Size=4
const hsdSize = 2 * 8 + 2 * 4;

// 2. kSizeInBytes = 11 * kSystemPointerSize + HSD
const m = hSrc.match(/static constexpr int kSizeInBytes =\s*\n?\s*11 \* kSystemPointerSize \+ HandleScopeData::kSizeInBytes;/);
if (!m) throw new Error("kSizeInBytes 公式漂移");
const kSystemPointerSize = 8;
const sizeInBytes = 11 * kSystemPointerSize + hsdSize;

// 3. 哨兵
if (!hSrc.includes("static constexpr Address kSentinel = kNullAddress;")) throw new Error("kSentinel 漂移");
if (!hSrc.includes("kInvalidLastHandle =")) throw new Error("kInvalidLastHandle 漂移");

// 4. 方法面对数
const mustFns = ["Free", "ResetAfterArchive", "ArchiveSpace", "EnterContext", "LeaveContext", "SaveContext", "RestoreContext"];
const missing = mustFns.filter((f) => !hSrc.includes(f) && !ccSrc.includes(f));
if (missing.length) throw new Error(`implementer 缺方法: ${missing.join(",")}`);

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
const ccDefs = collectDefs(ccSrc, "HandleScopeImplementer");
if (ccDefs.length < 10) throw new Error(`implementer.cc 定义头应 ≥10, 实得 ${ccDefs.length}`);

let rust = banner("src/handles/handle-scope-implementer.h.js", "src/handles/handle-scope-implementer.{h,inl.h,cc}", "句柄域实现器: 尺寸常量与骨架");

rust += `
/// HandleScopeImplementer 布局常量 (原版 handle-scope-implementer.h)
pub mod handle_scope_implementer_const {
  /// kSystemPointerSize (src/common/globals.h)
  pub const SYSTEM_POINTER_SIZE: usize = ${kSystemPointerSize};
  /// HandleScopeData::kSizeInBytes = 2*ptr + 2*int32 (v8-internal.h: next/limit 双指针 + level/sealed_level 双 int)
  pub const HANDLE_SCOPE_DATA_SIZE_IN_BYTES: usize = ${hsdSize};
  /// 归档块尺寸 = 11*ptr + HandleScopeData (原版 kSizeInBytes)
  pub const SIZE_IN_BYTES: usize = ${sizeInBytes};
}

/// 线程局部队列骨架 (原版 HandleScopeImplementer, 每线程独立副本)
pub mod handle_scope_implementer_skel {
  /// 空指针哨兵 (原版 kSentinel = kNullAddress)
  pub const SENTINEL: usize = 0;

  /// 归档/释放协议 (原版 Free / ResetAfterArchive / ArchiveSpace)
  pub fn free() {
    unimplemented!("骨架")
  }
  pub fn reset_after_archive() {
    unimplemented!("骨架")
  }
  /// 进入/离开上下文 (原版 EnterContext/LeaveContext; 自检回绕见 EnteredContextRewindScope)
  pub fn enter_context(_context: usize) {
    unimplemented!("骨架")
  }
  pub fn leave_context() {
    unimplemented!("骨架")
  }
  /// 保存/恢复上下文链 (原版 SaveContext/RestoreContext)
  pub fn save_context(_context: usize) {
    unimplemented!("骨架")
  }
  pub fn restore_context() -> usize {
    unimplemented!("骨架")
  }
}

// handle-scope-implementer.cc 定义头清单 (对数基准, ${ccDefs.length} 项):
${ccDefs.map((s) => "// " + s).join("\n")}
`;

const p = writeSrc("v8-handles", "handle_scope_implementer_generated.rs", rust);
console.log(`written ${p}; HSD=${hsdSize}, SIZE_IN_BYTES=${sizeInBytes}, cc定义=${ccDefs.length}`);
