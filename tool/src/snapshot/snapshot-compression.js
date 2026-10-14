#!/usr/bin/env bun
// tool/src/snapshot/snapshot-compression.js
// 机械提取 v8/src/snapshot/snapshot-compression.{h,cc} 的压缩门面与头部布局

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/snapshot-compression.h");
const ccSrc = readV8("src/snapshot/snapshot-compression.cc");

// 1. 门面 (h): Compress / Decompress
const clsM = hSrc.match(/class SnapshotCompression : public AllStatic \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 SnapshotCompression 类体");
const methods = [...clsM[1].matchAll(/static SnapshotData (\w+)\(/g)].map((m) => m[1]);
if (methods.length !== 2) throw new Error(`方法数不符: 期望 2, 实得 ${methods.length}: ${methods.join(",")}`);

// 2. cc 压缩布局契约 (机械提取): 载荷前缀存未压缩长度, 之后 zlib 原始压缩流
const sizeFn = ccSrc.match(/uint32_t GetUncompressedSize\([\s\S]*?MemCopy\(&size, compressed_data, sizeof\(size\)\);/);
if (!sizeFn) throw new Error("未找到 GetUncompressedSize 布局");
const rawM = ccSrc.includes("zlib_internal::ZRAW");
const compressBoundM = ccSrc.includes("compressBound(input_size)");
if (!rawM || !compressBoundM) throw new Error("压缩契约异常 (ZRAW/compressBound)");
console.log(`方法 ${methods.length} + 压缩布局 (前缀 4B + ZRAW 流)`);

let rust = banner("src/snapshot/snapshot-compression.js", "src/snapshot/snapshot-compression.{h,cc}", "快照压缩门面");
rust += `
/// SnapshotCompression (原版 AllStatic 门面): SnapshotData 压缩/解压。
/// 方法面:
/// - \`compress\` (原版 Compress): 未压缩 SnapshotData → 压缩 blob
/// - \`decompress\` (原版 Decompress): 压缩 blob → SnapshotData
/// 压缩 blob 布局 (原版 cc 机械登记):
/// [0..4) 未压缩载荷长度 (uint32 小端前缀, 无 zlib/gzip 头需手工存长度)
/// [4..) zlib 原始压缩流 (zlib_internal::ZRAW, Z_DEFAULT_COMPRESSION,
///       上界 compressBound(input_size))
/// 解压侧 GetUncompressedSize 读前缀还原长度; 算法实现由 Developer 落。
`;

const p = writeSrc("v8-snapshot", "snapshot_compression.rs", rust);
console.log(`written ${p}`);
