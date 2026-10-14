#!/usr/bin/env bun
// tool/src/heap/object-stats.js
// 机械提取 v8/src/heap/object-stats{.h,.cc} 族:
// VirtualInstanceType X-macro 展开 + OBJECT_STATS_COUNT 推导

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/object-stats.h");
const ccSrc = readV8("src/heap/object-stats.cc");

const anchors = [
  [hSrc, "static const size_t kNoOverAllocation = 0;"],
  [hSrc, "struct ObjectData {"],
  [hSrc, "enum class VirtualInstanceType {"],
  [hSrc, "#define DEFINE_VIRTUAL_INSTANCE_TYPE(type) type,"],
  [hSrc, "VIRTUAL_INSTANCE_TYPE_LIST(DEFINE_VIRTUAL_INSTANCE_TYPE)"],
  [hSrc, "LAST_VIRTUAL_TYPE = WEAK_NEW_SPACE_OBJECT_TO_CODE_TYPE,"],
  [hSrc, "static constexpr int FIRST_VIRTUAL_TYPE = LAST_TYPE + 1;"],
  [hSrc, "static constexpr int OBJECT_STATS_COUNT ="],
  [hSrc, "void RecordObjectStats(Tagged<HeapObject> obj, InstanceType type, size_t size,"],
  [hSrc, "void RecordVirtualObjectStats(Tagged<HeapObject> obj,"],
  [hSrc, "class ObjectStatsCollector {"],
  [ccSrc, "void ObjectStats::CheckpointObjectStats() {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// —— X-macro 展开: VIRTUAL_INSTANCE_TYPE_LIST(V) 体 ——

const listBody = hSrc.match(/#define VIRTUAL_INSTANCE_TYPE_LIST\(V\)[^\n]*\n([\s\S]*?)\n\n/);
if (!listBody) throw new Error("VIRTUAL_INSTANCE_TYPE_LIST 未匹配");
const body = listBody[1];
// 展开 CODE_KIND_LIST(V) → 读 code-kind.h 项
const ckSrc = readV8("src/objects/code-kind.h");
const ckl = ckSrc.match(/#define CODE_KIND_LIST\(V\)[^\n]*\n([\s\S]*?)\n\n/);
if (!ckl) throw new Error("CODE_KIND_LIST 未匹配");
const codeKinds = (ckl[1].match(/\bV\((\w+)\)/g) || []).map((s) => s.match(/V\((\w+)\)/)[1]);
// 直接 V(XXX) 项
const direct = (body.match(/^\s*V\((\w+)\)/gm) || []).map((s) => s.match(/V\((\w+)\)/)[1]);
const all = [...codeKinds, ...direct];
const dup = all.filter((v, i) => all.indexOf(v) !== i);
if (dup.length) throw new Error(`重复项: ${dup}`);
if (all[all.length - 1] !== "WEAK_NEW_SPACE_OBJECT_TO_CODE_TYPE") throw new Error(`末项漂移: ${all[all.length - 1]}`);
if (all.length !== codeKinds.length + direct.length) throw new Error("计数错");

console.log(`object-stats: VirtualInstanceType ${all.length} 项 (CODE_KIND_LIST ${codeKinds.length} + 直接 ${direct.length}), LAST_VIRTUAL_TYPE=${all[all.length - 1]}`);

let rust = banner(
  "src/heap/object-stats.js",
  "src/heap/object-stats.h, src/heap/object-stats.cc",
  "对象统计: 虚拟实例类型 X-macro 展开与计数数组",
);

rust += `/// v8: src/heap/object-stats.h VirtualInstanceType
/// = CODE_KIND_LIST (code-kind.h) + VIRTUAL_INSTANCE_TYPE_LIST 直接项
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum VirtualInstanceType {
`;
all.forEach((name, i) => (rust += `  /// ${i === all.length - 1 ? "LAST_VIRTUAL_TYPE" : `V(${name})`}\n  ${name} = ${i},\n`));
rust += `}

impl VirtualInstanceType {
  /// v8: VirtualInstanceType::LAST_VIRTUAL_TYPE = WEAK_NEW_SPACE_OBJECT_TO_CODE_TYPE
  pub const LAST_VIRTUAL_TYPE: Self = Self::WEAK_NEW_SPACE_OBJECT_TO_CODE_TYPE;
}

/// v8: src/heap/object-stats.h ObjectStats 常量与计数推导
pub mod object_stats_consts {
  /// v8: ObjectStats::kNoOverAllocation = 0
  pub const K_NO_OVER_ALLOCATION: usize = 0;
  /// v8: FIRST_VIRTUAL_TYPE = LAST_TYPE + 1 (InstanceType 尾后)
  pub const FIRST_VIRTUAL_TYPE: i32 = LAST_TYPE as i32 + 1;
  /// v8: OBJECT_STATS_COUNT = FIRST_VIRTUAL_TYPE + LAST_VIRTUAL_TYPE + 1
  pub const OBJECT_STATS_COUNT: i32 = FIRST_VIRTUAL_TYPE + ${all.length - 1} + 1;
}

/// v8: src/heap/object-stats.h ObjectStats
/// counts/sizes 双数组, 按组连续存放; ObjectData{address,size,type}
/// v8: ClearObjectStats / PrintJSON / Dump / CheckpointObjectStats
/// v8: RecordObject / RecordObjectStats(obj, type, size, over_allocated)
/// v8: RecordVirtualObjectStats
/// v8: ObjectStatsCollector (cc): CollectStatistics 遍历各空间
/// v8: src/heap/object-stats.cc ObjectStats::CheckpointObjectStats 等

// —— 提取不变式断言 ——
const _: () = {
  assert!(VirtualInstanceType::LAST_VIRTUAL_TYPE as i32 == ${all.length - 1});
  assert!(object_stats_consts::OBJECT_STATS_COUNT > object_stats_consts::FIRST_VIRTUAL_TYPE);
};
`;

const p = writeSrc("v8-heap-core", "object_stats.rs.skel", rust);
console.log(`written ${p}`);
