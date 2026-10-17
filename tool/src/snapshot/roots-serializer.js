#!/usr/bin/env bun
// tool/src/snapshot/roots-serializer.js
// 机械提取 v8/src/snapshot/roots-serializer.{h,cc} 的根序列化基类方法面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/roots-serializer.h");
const ccSrc = readV8("src/snapshot/roots-serializer.cc");

// 1. RootsSerializer 方法面 (h, public + protected)
const clsM = hSrc.match(/class RootsSerializer : public Serializer \{([\s\S]*?)\n private:/);
if (!clsM) throw new Error("未找到 RootsSerializer 类体");
const methods = [...clsM[1].matchAll(/^\s{2,4}(?:bool|int|void)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 6) {
  throw new Error(`RootsSerializer 方法数不符: 期望 6, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. 字段骨架: root_has_been_serialized_ 位集容量 = RootsTable::kEntriesCount
const bitsM = hSrc.match(/std::bitset<RootsTable::kEntriesCount> root_has_been_serialized_;/);
if (!bitsM) throw new Error("未找到 root_has_been_serialized_ 位集");

// 3. cc 实现清点
const ccFns = [...ccSrc.matchAll(/^(?:void|int|bool) RootsSerializer::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length < 3) throw new Error(`cc 实现过少: ${ccFns.length}`);
console.log(`方法 ${methods.length} + cc ${ccFns.length} 实现`);

let rust = banner("src/snapshot/roots-serializer.js", "src/snapshot/roots-serializer.{h,cc}", "根遍历序列化基类");
rust += `
/// RootsSerializer (原版, 继承 Serializer): 遍历根的序列化器基类,
/// 维护可与非序列化器共享的对象缓存。方法面 (原版 h, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// 字段骨架:
/// - first_root_to_be_serialized_: 构造时约定此前所有根已序列化
/// - root_has_been_serialized_: bitset<RootsTable::kEntriesCount> 根标记位集
/// - object_cache_index_map_: 对象缓存索引 (对齐 serializer.rs ObjectCacheIndexMap)
/// - can_be_rehashed_: 仅序列化可重散列哈希表时为真
/// cc 实现 (机械清点): ${ccFns.join(", ")}
`;

const p = writeSrc("v8-snapshot", "roots_serializer.rs", rust);
console.log(`written ${p}`);
