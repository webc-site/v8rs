#!/usr/bin/env bun
// tool/src/snapshot/snapshot-source-sink.js
// 机械提取 v8/src/snapshot/snapshot-source-sink.{h,cc} 的字节源/汇方法面与 Uint30 编码契约

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/snapshot-source-sink.h");
const ccSrc = readV8("src/snapshot/snapshot-source-sink.cc");

// 1. SnapshotByteSource 方法面 (h, public 区)
const srcClassM = hSrc.match(/class SnapshotByteSource final \{([\s\S]*?)\n private:/);
if (!srcClassM) throw new Error("未找到 SnapshotByteSource 类体");
const srcMethods = [...srcClassM[1].matchAll(/^\s{2}(?:inline\s+)?(?:uint8_t|uint32_t|int|void|bool|size_t|const [\w:<>*]+)\s+(\w+)\(/gm)].map((m) => m[1]);
if (srcMethods.length < 10) throw new Error(`SnapshotByteSource 方法过少: ${srcMethods.length}`);

// 2. SnapshotByteSink 方法面
const sinkClassM = hSrc.match(/class SnapshotByteSink \{([\s\S]*?)\n private:/);
if (!sinkClassM) throw new Error("未找到 SnapshotByteSink 类体");
const sinkMethods = [...sinkClassM[1].matchAll(/^\s{2}(?:inline\s+)?(?:void|size_t|const [\w:<>*]+)\s+(\w+)\(/gm)].map((m) => m[1]);
if (sinkMethods.length < 6) throw new Error(`SnapshotByteSink 方法过少: ${sinkMethods.length}`);

// 3. Uint30 游程编码契约 (机械提取 GetUint30 的位运算骨架):
// 低 2 位 + 1 = 字节数; 掩码右移 (bytes<<3); 结果右移 2
const g30 = hSrc.match(/inline uint32_t GetUint30\(\) \{([\s\S]*?)\n  \}/);
if (!g30) throw new Error("未找到 GetUint30");
const lowBits = 2; // (answer & 3) + 1
if (!g30[1].includes("& 3") || !g30[1].includes(">>= 32 - (bytes << 3)")) {
  throw new Error("GetUint30 位运算骨架异常");
}

// 4. V8_COMPRESS_POINTERS 分支 (CopySlots 重载)
const cpBranch = (hSrc.match(/#ifdef V8_COMPRESS_POINTERS/g) || []).length;
if (cpBranch !== 1) throw new Error(`压缩指针分支数异常: ${cpBranch}`);

// 5. cc 实现 (PutN / PutUint30 / PutRaw / Append / GetBlob; PutUint32 为 h 内联)
const ccFns = [...ccSrc.matchAll(/^(?:int|void)\s+Snapshot\w+::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length !== 5) throw new Error(`cc 实现函数数不符: 期望 5, 实得 ${ccFns.length}: ${ccFns.join(",")}`);
console.log(`Source ${srcMethods.length} 方法 + Sink ${sinkMethods.length} 方法 + cc ${ccFns.length} 实现`);

let rust = banner(
  "src/snapshot/snapshot-source-sink.js",
  "src/snapshot/snapshot-source-sink.{h,cc}",
  "快照字节源/字节汇",
);
rust += `
/// 快照/builtin 文件读取源 (原版 SnapshotByteSource; 内存所有权归调用方)。
/// 方法面 (原版 h, 共 ${srcMethods.length} 项):
`;
for (const m of srcMethods) rust += `/// - \`${m}\`\n`;
rust += `
/// Uint30 游程编码契约 (原版 GetUint30/PutUint30):
/// - 首 2 位 + 1 = 有效字节数 (1..4)
/// - 32 位掩码右移 (bytes << 3) 截断, 结果再右移 2 去掉长度位
/// - 解码无分支预测惩罚写法: 逐字节 | 后一次掩码
///
/// 字节汇 (原版 SnapshotByteSink, 用户落地存储/IO)。方法面 (共 ${sinkMethods.length} 项):
`;
for (const m of sinkMethods) rust += `/// - \`${m}\`\n`;
rust += `
/// 条件分支登记: #ifdef V8_COMPRESS_POINTERS x${cpBranch} — CopySlots 的
/// Tagged_t (4 字节压缩槽) 重载仅在压缩指针构建存在。
/// cc 实现 (机械提取): ${ccFns.join(", ")}
`;

const p = writeSrc("v8-snapshot", "snapshot_source_sink.rs", rust);
console.log(`written ${p}`);
