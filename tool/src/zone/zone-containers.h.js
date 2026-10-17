#!/usr/bin/env bun
// tool/src/zone/zone-containers.h.js
// 机械提取 v8/src/zone/ 容器七件: zone-containers.h + zone-allocator.h +
// zone-compact-set.h + zone-hashmap.h + zone-list-inl.h + zone-type-traits.h + zone-utils.h

import { readV8, writeSrc, banner } from "../../lib.js";

const containers = readV8("src/zone/zone-containers.h");
const allocator = readV8("src/zone/zone-allocator.h");
const compactSet = readV8("src/zone/zone-compact-set.h");
const hashmap = readV8("src/zone/zone-hashmap.h");
const listInl = readV8("src/zone/zone-list-inl.h");
const typeTraits = readV8("src/zone/zone-type-traits.h");
const utils = readV8("src/zone/zone-utils.h");

// 1. compact-set 标签枚举 (机械提取)
const tagM = compactSet.match(/enum Tag \{ ([^}]+) \};/);
if (!tagM) throw new Error("zone-compact-set.h 未找到 Tag 枚举");
const tags = tagM[1].split(",").map((s) => s.trim());
if (tagM[1] !== "kSingletonTag = 0, kEmptyTag = 1, kListTag = 2") throw new Error("Tag 漂移");

// 2. allocator 关键面 (ZoneAllocator + RecyclingZoneAllocator 复用变体)
for (const k of ["class ZoneAllocator", "using value_type", "class RecyclingZoneAllocator"]) {
  if (!allocator.includes(k)) throw new Error(`zone-allocator.h 缺 ${k}`);
}

// 3. containers 关键面 (ZoneVector/ZoneDeque/ZoneIntrusiveSet 等类族)
const classes = [...containers.matchAll(/\nclass (Zone\w+)/g)].map((m) => m[1]);
if (classes.length < 8) throw new Error(`Zone 容器类应 ≥8, 实得 ${classes.length}: ${classes.join(",")}`);

// 4. hashmap 薄壳
if (!hashmap.includes("ZoneHashmap") && !hashmap.includes("using")) throw new Error("zone-hashmap.h 面缺失");

// 5. list-inl: ZoneList 内联段 (Add/ResizeAdd/DropAndClear)
for (const k of ["Add(", "ResizeAdd(", "DropAndClear()"]) {
  if (!listInl.includes(k)) throw new Error(`zone-list-inl.h 缺 ${k}`);
}

// 6. type-traits / utils
if (!typeTraits.includes("is_zone_container") && !typeTraits.includes("ZoneTypeTraits") && !/template/.test(typeTraits)) {
  throw new Error("zone-type-traits.h 面缺失");
}
if (!utils.includes("CloneVector")) throw new Error("zone-utils.h 缺 CloneVector");
// CloneVector 分支: trivially copyable → MemCopy, 否则 std::copy
if (!utils.includes("is_trivially_copyable_v<T>")) throw new Error("CloneVector 分支漂移");

let rust = banner(
  "src/zone/zone-containers.h.js",
  "src/zone/zone-{containers,allocator,compact-set,hashmap,list-inl,type-traits,utils}.h",
  "Zone 容器七件骨架",
);

rust += `
/// ZoneCompactSet 标签枚举 (原版 zone-compact-set.h enum Tag; 压缩于指针载荷位)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ZoneCompactSetTag {
  /// 单例: 载荷即句柄
  Singleton = 0,
  /// 空
  Empty = 1,
  /// 列表: 载荷指向 ZoneList
  List = 2,
}

/// Zone 容器类族 (原版 zone-containers.h class 清单, ${classes.length} 项)
pub const ZONE_CONTAINER_ALIASES: [&str; ${classes.length}] = [${classes.map((a) => `"${a}"`).join(", ")}];

/// 容器骨架
pub mod zone_containers_skel {
  /// ZoneAllocator: 分配器适配 (原版 value_type 接口 + RecyclingZoneAllocator 复用变体)
  pub fn allocator_rebind() {
    unimplemented!("骨架")
  }

  /// ZoneHashmap: Zone 支撑的映射薄壳 (原版 zone-hashmap.h)
  pub fn hashmap_skel() {
    unimplemented!("骨架")
  }

  /// ZoneList 内联段: Add/clear 热路径 (原版 zone-list-inl.h)
  pub fn zone_list_add(_value: usize) {
    unimplemented!("骨架")
  }

  /// CloneVector 分支协议 (原版 zone-utils.h):
  /// 平凡可拷 → MemCopy; 否则逐元素 std::copy
  pub fn clone_vector_trivially_copyable_branch() -> bool {
    unimplemented!("骨架")
  }

  /// ZoneCompactSet 构造协议: 单例持 HandleToPointer, 列表挂 ZoneList (原版实现)
  pub fn compact_set_from_singleton() {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-zone", "containers/extra_generated.rs", rust);
console.log(`written ${p}; 类族=${classes.length}, Tag=${tags.length}`);
