#!/usr/bin/env bun
// tool/src/snapshot/read-only-serializer.js
// 机械提取 v8/src/snapshot/read-only-serializer.{h,cc} 的方法面与缓存契约

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/read-only-serializer.h");
const ccSrc = readV8("src/snapshot/read-only-serializer.cc");

// 1. ReadOnlySerializer 方法面 (h)
const clsM = hSrc.match(/class V8_EXPORT_PRIVATE ReadOnlySerializer : public RootsSerializer \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 ReadOnlySerializer 类体");
const methods = [...clsM[1].matchAll(/^\s{2,4}(?:void|bool)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 2) {
  throw new Error(`ReadOnlySerializer 方法数不符: 期望 2, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. cc 规模登记
const ccLines = ccSrc.split("\n").length;
// RO 对象缓存重建契约 (cc 内 ReconstructReadOnlyObjectCache / SerializeReadOnlyRoot 等)
const ccFns = [...ccSrc.matchAll(/^void ReadOnlySerializer::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length !== 1) throw new Error(`cc 实现函数不符: 期望 1 (Serialize, 其余为类内定义), 实得 ${ccFns.length}`);
console.log(`方法 ${methods.length} + cc ${ccLines} 行 (${ccFns.length} 实现)`);

let rust = banner("src/snapshot/read-only-serializer.js", "src/snapshot/read-only-serializer.{h,cc}", "只读空间序列化器");
rust += `
/// ReadOnlySerializer (原版, 继承 RootsSerializer)。
/// 方法面 (原版 h, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// cc 实现 (机械提取, ${ccLines} 行): ${ccFns.join(", ")}
/// 契约: RO 快照一次性生成; object_cache 可按
/// kReconstructReadOnlyAndSharedObjectCachesForTesting 从现有 RO heap 重建,
/// 保证同索引映射; 反序列化端配合 ro::Bytecode (kAllocatePage/kSegment/...)。
`;

const p = writeSrc("v8-snapshot", "read_only_serializer.rs", rust);
console.log(`written ${p}`);
