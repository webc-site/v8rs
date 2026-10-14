// 由 tool/src/snapshot/snapshot-compression.js 从 v8/src/snapshot/snapshot-compression.{h,cc} 生成; 修订就地改, 复跑覆盖
// 快照压缩门面

/// v8: src/snapshot/snapshot-compression.cc GetUncompressedSize
/// 压缩 blob 首 4 字节为未压缩载荷长度 (uint32 小端前缀)。
/// 无 zlib/gzip 头, 长度手工存储, 解压侧据此还原。
/// 依赖缓冲区前 4 字节在界内 (调用方保证), 越界为契约错误。
#[inline]
#[must_use]
pub fn get_uncompressed_size(compressed_data: &[u8]) -> u32 {
  debug_assert!(
    compressed_data.len() >= PAYLOAD_LENGTH_PREFIX_SIZE,
    "GetUncompressedSize 前缀越界"
  );
  u32::from_le_bytes([
    compressed_data[0],
    compressed_data[1],
    compressed_data[2],
    compressed_data[3],
  ])
}

/// 未压缩长度前缀宽度 (原版 sizeof(payload_length) = uint32)。
pub const PAYLOAD_LENGTH_PREFIX_SIZE: usize = 4;

/// v8: src/snapshot/snapshot-compression.h SnapshotCompression
/// SnapshotData 压缩/解压 AllStatic 门面。
/// - `compress` (原版 Compress): 未压缩载荷 → 长度前缀 + zlib 原始压缩流
/// - `decompress` (原版 Decompress): 压缩 blob → 还原载荷
pub struct SnapshotCompression;

/// v8: src/snapshot/snapshot-compression.cc compressed blob length-prefix framing
/// 压缩 blob 布局 (原版 cc 机械登记):
/// [0..4) 未压缩载荷长度 (uint32 小端前缀)
/// [4..) zlib 原始压缩流 (zlib_internal::ZRAW, Z_DEFAULT_COMPRESSION,
///       上界 compressBound(input_size))
///
/// `framing` 子模块提供纯布局面 (长度前缀读写), 可脱离压缩算法单测;
/// 压缩/解压算法体依赖 zlib (ZRAW 无损 DEFLATE 裸流), 语义绑定推迟 (见 readme/zh.md):
/// zlib 不在第十章 v8-snapshot 白名单, 且 ZRAW 字节流需与原版逐字节差分后方可接入。
pub mod framing {
  use super::PAYLOAD_LENGTH_PREFIX_SIZE;

  /// 原版 Compress 头部写入: 前置 uint32 小端长度, 返回带前缀缓冲 (载荷由外层填压缩流)。
  #[must_use]
  pub fn write_length_prefix(payload_length: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(PAYLOAD_LENGTH_PREFIX_SIZE);
    out.extend_from_slice(&payload_length.to_le_bytes());
    out
  }

  /// 剥离长度前缀, 返回 (未压缩长度, 压缩流切片)。
  #[must_use]
  pub fn split(compressed: &[u8]) -> Option<(u32, &[u8])> {
    if compressed.len() < PAYLOAD_LENGTH_PREFIX_SIZE {
      return None;
    }
    let len = u32::from_le_bytes([compressed[0], compressed[1], compressed[2], compressed[3]]);
    Some((len, &compressed[PAYLOAD_LENGTH_PREFIX_SIZE..]))
  }
}
