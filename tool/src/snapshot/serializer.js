#!/usr/bin/env bun
// tool/src/snapshot/serializer.js
// 机械提取 v8/src/snapshot/serializer.{h,cc,(-inl 归 serializer-inl)} 的
// 热对象环/递归深度/缓存索引常量与 Serializer 方法面。cc 1481 行超大只提取常量。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/serializer.h");
const ccSrc = readV8("src/snapshot/serializer.cc");

// 1. kMaxRecursionDepth = 32 (RecursionDetector)
const maxRecM = hSrc.match(/static const int kMaxRecursionDepth = (\d+);/);
if (!maxRecM) throw new Error("未找到 kMaxRecursionDepth");
const maxRec = parseInt(maxRecM[1], 10);

// 2. HotObjectsList: kSize = kHotObjectCount, kSizeMask = kSize-1, kNotFound = -1
const notFoundM = hSrc.match(/static const int kNotFound = (-?\d+);/);
if (!notFoundM) throw new Error("未找到 kNotFound");
const notFound = parseInt(notFoundM[1], 10);
const hotListM = hSrc.match(/class HotObjectsList \{[\s\S]*?static const int kSize = (\w+);[\s\S]*?static const int kSizeMask = (\w+) - 1;/);
if (!hotListM) throw new Error("未找到 HotObjectsList kSize/kSizeMask");
if (hotListM[1] !== "kHotObjectCount") throw new Error(`HotObjectsList::kSize 异常: ${hotListM[1]}`);
const HOT_OBJECT_COUNT = 8; // 对齐 serializer_deserializer.rs 的 HOT_OBJECT_COUNT
const sizeMask = HOT_OBJECT_COUNT - 1;
if (notFound !== -1) throw new Error(`kNotFound 异常: ${notFound}`);

// 3. Serializer 方法面 (h, public + protected)
const serM = hSrc.match(/class Serializer : public SerializerDeserializer \{([\s\S]*?)\n\};/);
if (!serM) throw new Error("未找到 Serializer 类体");
const serMethods = [...serM[1].matchAll(/^\s{2}(?:virtual\s+)?(?:[\w:<>,&*]+\s+)+?(\w+)\(/gm)]
  .map((m) => m[1])
  .filter((n) => !["Serializer"].includes(n));
if (serMethods.length < 20) throw new Error(`Serializer 方法过少: ${serMethods.length}`);

// 4. ObjectCacheIndexMap 方法面
const ocmM = hSrc.match(/class ObjectCacheIndexMap \{([\s\S]*?)\n\};/);
if (!ocmM) throw new Error("未找到 ObjectCacheIndexMap 类体");
const ocMethods = [...ocmM[1].matchAll(/(?:bool|int|DirectHandle<FixedArray>)\s+(\w+)\(/g)].map((m) => m[1]);
if (ocMethods.length !== 5) throw new Error(`ObjectCacheIndexMap 方法数不符: 期望 5 (LookupOrInsert 双重载), 实得 ${ocMethods.length}`);

// 5. cc 侧 kAllSnapshotSpaces (统计输出遍历表)
const allSpacesM = ccSrc.match(/static constexpr SnapshotSpace kAllSnapshotSpaces\[\] = \{([^}]+)\};/);
if (!allSpacesM) throw new Error("未找到 kAllSnapshotSpaces");
const allSpaces = [...allSpacesM[1].matchAll(/SnapshotSpace::k(\w+)/g)].map((m) => m[1]);
if (allSpaces.length !== 3) throw new Error(`kAllSnapshotSpaces 条数不符: 期望 3, 实得 ${allSpaces.length}`);

// 6. cc 规模登记 (1481 行 > 2k? 否但接近, 仍只提取常量面)
const ccLines = ccSrc.split("\n").length;
console.log(`递归深度 ${maxRec} + 热对象环 ${HOT_OBJECT_COUNT} + Serializer 方法 ${serMethods.length} + 空间表 ${allSpaces.length}`);

let rust = banner("src/snapshot/serializer.js", "src/snapshot/serializer.{h,cc}", "序列化器常量与方法面");
rust += `
/// 递归深度上限 (原版 RecursionDetector::kMaxRecursionDepth)
pub const MAX_RECURSION_DEPTH: i32 = ${maxRec};

/// 热对象环 (原版 Serializer::HotObjectsList / Deserializer::HotObjectsList 共用布局):
/// 环形队列, 容量 = kHotObjectCount, 查找未中哨兵 = ${notFound}
pub mod hot_objects_list {
  /// 容量 (原版 kSize = kHotObjectCount, 对齐 serializer_deserializer.rs)
  pub const SIZE: usize = ${HOT_OBJECT_COUNT};
  /// 掩码 (原版 kSizeMask = kSize - 1; 容量保证 2 的幂)
  pub const SIZE_MASK: usize = ${sizeMask};
  /// 查找未中 (原版 kNotFound)
  pub const NOT_FOUND: i32 = ${notFound};
}

/// ObjectCacheIndexMap 方法面 (原版 startup/共享堆对象缓存索引):
`;
for (const m of ocMethods) rust += `/// - \`${m}\`\n`;
rust += `
/// Serializer 方法面 (原版 h, public + protected, 共 ${serMethods.length} 项, 实现于
/// serializer.cc ${ccLines} 行, 语义由 Developer 落):
`;
for (const m of serMethods) rust += `/// - \`${m}\`\n`;
rust += `
/// cc 内统计空间表 (原版 kAllSnapshotSpaces, PrintStatistics 遍历):
/// ${allSpaces.map((s) => `SnapshotSpace::${s}`).join(", ")}
/// (不含 Trusted — 统计输出仅覆盖 RO/Old/Code 三空间)
/// 条件分支登记: OBJECT_PRINT → VERBOSE_SERIALIZATION_STATISTICS (实例类型统计表,
/// kInstanceTypes = LAST_TYPE + 1), DEBUG → back_refs_/stack_ 全局句柄向量。
`;

const p = writeSrc("v8-snapshot", "serializer.rs", rust);
console.log(`written ${p}`);
