#!/usr/bin/env bun
// tool/src/snapshot/shared-heap-serializer.js
// 机械提取 v8/src/snapshot/shared-heap-serializer.{h,cc} 的方法面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/shared-heap-serializer.h");
const ccSrc = readV8("src/snapshot/shared-heap-serializer.cc");

// 1. 方法面 (h)
const clsM = hSrc.match(/class V8_EXPORT_PRIVATE SharedHeapSerializer : public RootsSerializer \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 SharedHeapSerializer 类体");
const methods = [...clsM[1].matchAll(/^\s{2,4}(?:static\s+)?(?:void|bool)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 8) {
  throw new Error(`SharedHeapSerializer 方法数不符: 期望 8, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. cc 实现清点
const ccFns = [...ccSrc.matchAll(/^(?:void|bool) SharedHeapSerializer::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length < 5) throw new Error(`cc 实现过少: ${ccFns.length}`);
const ccLines = ccSrc.split("\n").length;
console.log(`方法 ${methods.length} + cc ${ccLines} 行 (${ccFns.length} 实现)`);

let rust = banner("src/snapshot/shared-heap-serializer.js", "src/snapshot/shared-heap-serializer.{h,cc}", "共享堆序列化器");
rust += `
/// SharedHeapSerializer (原版, 继承 RootsSerializer): 共享空间对象序列化。
/// 方法面 (原版 h, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// cc 实现 (机械清点, ${ccLines} 行): ${ccFns.join(", ")}
/// 契约: 对象可共享性判定 (字符串表/ETC) 与共享对象缓存重建
/// (配合 kReconstructReadOnlyAndSharedObjectCachesForTesting)。
`;

const p = writeSrc("v8-snapshot", "shared_heap_serializer.rs", rust);
console.log(`written ${p}`);
