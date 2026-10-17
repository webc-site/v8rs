#!/usr/bin/env bun
// tool/src/snapshot/snapshot-data.js
// 机械提取 v8/src/snapshot/snapshot-data.{h,cc} 的 SerializedData 位域、
// 魔数与 SnapshotData 头布局

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/snapshot-data.h");
const ccSrc = readV8("src/snapshot/snapshot-data.cc");

const K_UINT32_SIZE = 4; // 原版 kUInt32Size, 通用机械展开

// 1. 位域: ChunkSizeBits = BitField<uint32_t,0,31>, IsLastChunkBits = BitField<bool,31,1>
const chunkM = hSrc.match(/using ChunkSizeBits = base::BitField<uint32_t, (\d+), (\d+)>;/);
const lastM = hSrc.match(/using IsLastChunkBits = base::BitField<bool, (\d+), (\d+)>;/);
if (!chunkM || !lastM) throw new Error("未找到 ChunkSizeBits/IsLastChunkBits 位域");
const chunkOff = parseInt(chunkM[1], 10);
const chunkW = parseInt(chunkM[2], 10);
const lastOff = parseInt(lastM[1], 10);
const lastW = parseInt(lastM[2], 10);
if (chunkOff + chunkW !== lastOff || lastOff + lastW !== 32) {
  throw new Error(`位域布局异常: chunk ${chunkOff}+${chunkW}, last ${lastOff}+${lastW}`);
}

// 2. 魔数
const magicOffM = hSrc.match(/static constexpr uint32_t kMagicNumberOffset = (\d+);/);
const magicM = hSrc.match(/static constexpr uint32_t kMagicNumber =\s*([^;]+);/);
if (!magicOffM || !magicM) throw new Error("未找到魔数定义");
const magicOff = parseInt(magicOffM[1], 10);
const magicExpr = magicM[1].replace(/\s+/g, " ").trim();
if (!magicExpr.startsWith("0xC0DE0000")) throw new Error(`魔数表达式异常: ${magicExpr}`);

// 3. SnapshotData 头布局 (kPayloadLengthOffset / kHeaderSize)
const payloadM = hSrc.match(
  /static const uint32_t kPayloadLengthOffset = kMagicNumberOffset \+ kUInt32Size;/,
);
const headerM = hSrc.match(
  /static const uint32_t kHeaderSize = kPayloadLengthOffset \+ kUInt32Size;/,
);
if (!payloadM || !headerM) throw new Error("未找到 SnapshotData 头布局常量");

// 4. cc 侧实现函数清点 (机械登记)
const ccFns = [...ccSrc.matchAll(/^(?:void|uint32_t|SnapshotData::\w+)\s+(?:SnapshotData::)?(\w+)\(/gm)].map((m) => m[1]);
console.log(`位域 (${chunkW}+${lastW}) + 魔数 + 头布局 2 常量 + cc 函数 ${ccFns.length}`);

let rust = banner("src/snapshot/snapshot-data.js", "src/snapshot/snapshot-data.{h,cc}", "快照数据 blob 头布局");
rust += `
/// SerializedData: 快照 blob 基类。头部署 uint32 槽, 小端读写
/// (原版 base::Read/WriteLittleEndianValue)。
pub mod serialized_data {
  /// ChunkSizeBits = BitField<uint32_t, ${chunkOff}, ${chunkW}>: 压缩块大小
  pub const CHUNK_SIZE_OFFSET: u32 = ${chunkOff};
  pub const CHUNK_SIZE_BITS: u32 = ${chunkW};
  pub const CHUNK_SIZE_MASK: u32 = ((1u32 << CHUNK_SIZE_BITS) - 1) << CHUNK_SIZE_OFFSET;

  /// IsLastChunkBits = BitField<bool, ${lastOff}, ${lastW}>: 末块标记
  pub const IS_LAST_CHUNK_OFFSET: u32 = ${lastOff};
  pub const IS_LAST_CHUNK_BITS: u32 = ${lastW};
  pub const IS_LAST_CHUNK_MASK: u32 = 1u32 << IS_LAST_CHUNK_OFFSET;

  /// 魔数槽偏移 (原版 kMagicNumberOffset)
  pub const MAGIC_NUMBER_OFFSET: u32 = ${magicOff};
  /// 魔数 = 0xC0DE0000 ^ ExternalReferenceTable::kSize
  /// (原版表达式: ${magicExpr}; ExternalReferenceTable::kSize 跨模块常量链,
  /// 由 Developer 对齐 v8-common/external-reference-table 产物后落值)
  pub const MAGIC_NUMBER_EXPR: &str = "${magicExpr}";

  /// 原版 ChunkSizeBits::encode
  #[inline]
  #[must_use]
  pub const fn encode_chunk_size(v: u32) -> u32 {
    (v << CHUNK_SIZE_OFFSET) & CHUNK_SIZE_MASK
  }
  /// 原版 IsLastChunkBits::encode
  #[inline]
  #[must_use]
  pub const fn encode_is_last_chunk(v: bool) -> u32 {
    if v { IS_LAST_CHUNK_MASK } else { 0 }
  }
}

/// SnapshotData: 预留区大小 + 序列化载荷包装。
/// 头布局 (uint32 槽):
/// [0] 魔数 + (内部) 外部引用数
/// [1] 载荷长度
/// 之后为序列化载荷
pub mod snapshot_data {
  /// kUInt32Size 机械展开
  pub const UINT32_SIZE: u32 = ${K_UINT32_SIZE};
  /// 载荷长度槽偏移 = kMagicNumberOffset + kUInt32Size
  pub const PAYLOAD_LENGTH_OFFSET: u32 = ${magicOff} + UINT32_SIZE;
  /// 头大小 = kPayloadLengthOffset + kUInt32Size
  pub const HEADER_SIZE: u32 = PAYLOAD_LENGTH_OFFSET + UINT32_SIZE;
}

/// cc 实现 (机械提取函数名): ${ccFns.join(", ")}
`;

const p = writeSrc("v8-snapshot", "snapshot_data.rs", rust);
console.log(`written ${p}`);
