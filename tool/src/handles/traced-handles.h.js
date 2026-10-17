#!/usr/bin/env bun
// tool/src/handles/traced-handles.h.js
// 机械提取 v8/src/handles/traced-handles.{h,inl.h,cc}:
// TracedNodeBlock 容量分支 (ASAN/64位/32位), MarkMode 枚举, 定义头清单

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/handles/traced-handles.h");
const inlSrc = readV8("src/handles/traced-handles-inl.h");
const ccSrc = readV8("src/handles/traced-handles.cc");

// 1. MarkMode 枚举
const markM = hSrc.match(/enum class MarkMode : uint8_t \{ ([^}]+) \};/);
if (!markM) throw new Error("未找到 MarkMode 枚举");
const markModes = markM[1].split(",").map((s) => s.trim());
if (markModes.join() !== "kOnlyYoung,kAll") throw new Error("MarkMode 漂移");

// 2. 容量分支 (机械提取三组)
const asanMin = hSrc.match(/V8_USE_ADDRESS_SANITIZER\)\s*\n\s*static constexpr size_t kMinCapacity = (\d+);/);
if (!asanMin || asanMin[1] !== "1") throw new Error("ASAN 容量漂移");
const host64Min = hSrc.match(/V8_HOST_ARCH_64_BIT\s*\n\s*static constexpr size_t kMinCapacity = (\d+);/);
if (!host64Min || host64Min[1] !== "256") throw new Error("64 位容量漂移");
const host32Min = hSrc.match(/#else\s*\/\/ !V8_HOST_ARCH_64_BIT\s*\n\s*static constexpr size_t kMinCapacity = (\d+);/);
if (!host32Min || host32Min[1] !== "128") throw new Error("32 位容量漂移");

// 3. 自由链表哨兵与静态断言
if (!hSrc.includes("kInvalidFreeListNodeIndex = -1;")) throw new Error("自由链表哨兵漂移");
const asserts = (hSrc.match(/static_assert\(k(MinCapacity|InvalidFreeListNodeIndex)/g) || []).length;
if (asserts !== 2) throw new Error(`静态断言应 2, 实得 ${asserts}`);

// 4. cc/inl 定义头对数
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
const ccDefs = collectDefs(ccSrc, "(?:TracedHandles|TracedNode|TracedNodeBlock)");
if (ccDefs.length < 30) throw new Error(`traced-handles.cc 定义头应 ≥30, 实得 ${ccDefs.length}`);
const inlDefs = collectDefs(inlSrc, "(?:TracedHandles|TracedNode|TracedNodeBlock)");

// 5. 块列表体系 (Overall/Usable/Young 三链)
for (const k of ["OverallList", "UsableList", "YoungList"]) {
  if (!hSrc.includes(k)) throw new Error(`traced-handles.h 缺 ${k} 链`);
}

let rust = banner("src/handles/traced-handles.h.js", "src/handles/traced-handles.{h,inl.h,cc}", "跟踪句柄: 块容量分支/标记模式/定义头清单");

rust += `
/// TracedNode 块容量分支 (原版 traced-handles.h #if 展开; x64/arm64 取 64 位非 ASAN 路径)
pub mod traced_node_block_capacity {
  /// ASAN 构建: 块容量固定 1
  pub const ASAN_MIN_CAPACITY: usize = 1;
  pub const ASAN_MAX_CAPACITY: usize = 1;
  /// 64 位主机 (当前目标): 最小容量 256
  pub const HOST_64BIT_MIN_CAPACITY: usize = 256;
  /// 32 位主机: 最小容量 128 (登记备查, 当前目标跳过)
  pub const HOST_32BIT_MIN_CAPACITY: usize = 128;
  /// 最大容量 = IndexType::max() - 1 (u32)
  pub const MAX_CAPACITY: u32 = u32::MAX - 1;
  /// 自由链表哨兵下标 (原版 kInvalidFreeListNodeIndex = -1)
  pub const INVALID_FREE_LIST_NODE_INDEX: i64 = -1;

  /// 静态断言登记 (原版 static_assert ×2):
  /// kMinCapacity <= kMaxCapacity; kInvalidFreeListNodeIndex > kMaxCapacity
  pub const STATIC_ASSERTS: usize = ${asserts};
}

/// 标记范围 (原版 enum class MarkMode : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MarkMode {
  /// 只标年轻代
  OnlyYoung = 0,
  /// 全量标记
  All = 1,
}

/// 块三链体系 (原版 OverallList/UsableList/YoungList, 双向线程表)
pub mod traced_block_lists {
  pub const LISTS: [&str; 3] = ["overall", "usable", "young"];
}

/// TracedHandles 签名骨架 (语义见 crate 内手写模块)
pub mod traced_handles_skel {
  /// 分配/释放节点 (原版 Allocate/Destroy)
  pub fn allocate() -> usize {
    unimplemented!("骨架")
  }
  /// GC 标记入口 (原版 Mark; 按 MarkMode 选范围)
  pub fn mark(_mode: MarkMode) {
    unimplemented!("骨架")
  }
  /// 年轻代处理 (原版 PostGarbageCollectionProcessing)
  pub fn post_garbage_collection_processing() {
    unimplemented!("骨架")
  }
  /// 快照导出 (原版 Iterate; 快照场景)
  pub fn iterate(_cb: &mut dyn FnMut()) {
    unimplemented!("骨架")
  }
}

// traced-handles.cc 定义头清单 (对数基准, ${ccDefs.length} 项):
${ccDefs.map((s) => "// " + s).join("\n")}
${inlDefs.length ? `\n// traced-handles-inl.h 定义头清单 (${inlDefs.length} 项):\n${inlDefs.map((s) => "// " + s).join("\n")}` : ""}
`;

const p = writeSrc("v8-handles", "traced_handles_generated.rs", rust);
console.log(`written ${p}; MarkMode=${markModes.length}, 容量分支=3, cc定义=${ccDefs.length}, inl定义=${inlDefs.length}`);
