#!/usr/bin/env bun
// tool/src/handles/global-handles.h.js
// 机械提取 v8/src/handles/global-handles.{h,inl.h,cc}:
// EternalHandles 分块寻址常量, 弱回调协议枚举, .cc 方法定义头对数清单

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/handles/global-handles.h");
const inlSrc = readV8("src/handles/global-handles-inl.h");
const ccSrc = readV8("src/handles/global-handles.cc");

// 1. EternalHandles 常量
function intConst(name) {
  const m = hSrc.match(new RegExp(`static const int ${name} = (-?\\d+);`));
  if (!m) throw new Error(`global-handles.h 未找到 ${name}`);
  return parseInt(m[1], 10);
}
const invalidIndex = intConst("kInvalidIndex");
const shift = intConst("kShift");
const sizeM = hSrc.match(/static const int kSize = 1 << kShift;/);
if (!sizeM) throw new Error("global-handles.h 未找到 kSize = 1 << kShift");
const size = 1 << shift;
const maskHex = hSrc.match(/static const int kMask = (0x[0-9a-fA-F]+);/);
if (!maskHex) throw new Error("global-handles.h 未找到 kMask");
const mask = parseInt(maskHex[1], 16);
if (invalidIndex !== -1 || shift !== 8 || size !== 256 || mask !== 0xff) {
  throw new Error(`EternalHandles 常量漂移: ${invalidIndex}/${shift}/${size}/${mask}`);
}
// 寻址协议: index >> kShift 块号, index & kMask 块内偏移
if (!hSrc.includes("blocks_[index >> kShift][index & kMask]")) throw new Error("分块寻址协议漂移");

// 2. 弱回调协议
const invM = hSrc.match(/enum InvocationType \{ ([^}]+) \};/);
if (!invM) throw new Error("未找到 InvocationType 枚举");
const invocations = invM[1].split(",").map((s) => s.trim());
if (invocations.join() !== "kFirstPass,kSecondPass") throw new Error("InvocationType 漂移");

const embedM = (readV8("include/v8-weak-callback-info.h")).match(/kEmbedderFieldsInWeakCallback = (\d+);/);
if (!embedM) throw new Error("未找到 kEmbedderFieldsInWeakCallback");
const embedderFields = parseInt(embedM[1], 10);

// 3. inl/cc 方法定义头对数
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
const ccDefs = collectDefs(ccSrc, "(?:GlobalHandles|EternalHandles|Node|NodeBlock|NodeSpace)");
if (ccDefs.length !== 35) throw new Error(`global-handles.cc 定义头应 35, 实得 ${ccDefs.length}`);
const inlDefs = collectDefs(inlSrc, "(?:GlobalHandles|EternalHandles)");
if (inlDefs.length !== 1) throw new Error(`global-handles-inl.h 定义头应 1, 实得 ${inlDefs.length}`);

let rust = banner("src/handles/global-handles.h.js", "src/handles/global-handles.{h,inl.h,cc}", "全局句柄: 永生句柄寻址/弱回调协议/定义头清单");

rust += `
/// EternalHandles 分块寻址常量 (原版 global-handles.h)
pub mod eternal_handles_const {
  /// 非法下标哨兵 (原版 kInvalidIndex)
  pub const INVALID_INDEX: i32 = ${invalidIndex};
  /// 块内偏移位宽 (原版 kShift)
  pub const SHIFT: i32 = ${shift};
  /// 每块槽数 = 1 << kShift (原版 kSize)
  pub const SIZE: i32 = ${size};
  /// 块内偏移掩码 (原版 kMask)
  pub const MASK: u32 = 0x${mask.toString(16)};

  /// 分块寻址协议 (原版 GetLocation): 块号 = index >> SHIFT, 槽 = index & MASK
  pub fn block_of(index: i32) -> usize {
    (index >> SHIFT) as usize
  }
  pub fn slot_of(index: i32) -> usize {
    (index & (MASK as i32)) as usize
  }
}

/// 弱回调调度协议 (原版 PendingPhantomCallback::InvocationType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InvocationType {
  /// 第一遍: 节点释放前的幻象回调
  FirstPass = 0,
  /// 第二遍: 独立任务里补发
  SecondPass = 1,
}

/// 弱回调内嵌字段槽位数 (原版 v8::kEmbedderFieldsInWeakCallback)
pub const EMBEDDER_FIELDS_IN_WEAK_CALLBACK: usize = ${embedderFields};

/// EternalHandles 签名骨架 (语义见 crate 内手写模块)
pub mod eternal_handles_skel {
  /// 建句柄并回写下标 (原版 Create)
  pub fn create(_isolate_key: usize, _object: usize, _index: &mut i32) {
    unimplemented!("骨架")
  }
  /// 取既有句柄 (原版 Get)
  pub fn get(_index: i32) -> usize {
    unimplemented!("骨架")
  }
  /// 全根遍历 (原版 IterateAllRoots)
  pub fn iterate_all_roots(_visitor: &mut dyn FnMut()) {
    unimplemented!("骨架")
  }
  /// 年轻代遍历 (原版 IterateYoungRoots)
  pub fn iterate_young_roots(_visitor: &mut dyn FnMut()) {
    unimplemented!("骨架")
  }
  /// GC 后新建代列表重建 (原版 PostGarbageCollectionProcessing)
  pub fn post_garbage_collection_processing() {
    unimplemented!("骨架")
  }
}

// global-handles.cc 方法定义头清单 (对数基准, ${ccDefs.length} 项):
${ccDefs.map((s) => "// " + s).join("\n")}

// global-handles-inl.h 定义头清单 (${inlDefs.length} 项):
${inlDefs.map((s) => "// " + s).join("\n")}
`;

const p = writeSrc("v8-handles", "global_handles_generated.rs", rust);
console.log(`written ${p}; Eternal常量=4, 弱回调=${invocations.length}, cc定义=${ccDefs.length}, inl定义=${inlDefs.length}`);
