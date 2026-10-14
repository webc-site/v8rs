// 由 tool/src/regexp/regexp-compiler-tonode.cc.js 从 v8/src/regexp/regexp-compiler-tonode.cc 生成; 修订就地改, 复跑覆盖
// Tree→Node 转换常量基线: Unicode 五段分裂表 + UC16Range 打包 + 展开阈值; 语义在 range_split.rs, 就地修订

/// Unicode 区间五段分裂表 (AddRange, cc:224-265): BMP 双段 + 代理区两段 + 非 BMP 段
pub const K_SEGMENT_STARTS: [u32; 5] = [
  0x0,
  0xd800,
  0xdc00,
  0xe000,
  0x10000,
];
pub const K_SEGMENT_ENDS: [u32; 5] = [
  0xd7ff,
  0xdbff,
  0xdfff,
  0xffff,
  0x10ffff,
];
/// 段归属 (bmp_ / lead_surrogates_ / trail_surrogates_ / non_bmp_; 第 4 段归 bmp_)
pub const K_SEGMENT_TARGETS: [SegmentTarget; 5] = [
  SegmentTarget::Bmp,
  SegmentTarget::LeadSurrogates,
  SegmentTarget::TrailSurrogates,
  SegmentTarget::Bmp,
  SegmentTarget::NonBmp,
];

/// 分裂段归属 (对应原版 CharacterRangeVector 成员)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentTarget {
  /// BMP 可单码点表示段 (两段共用)
  Bmp,
  /// 孤立前导代理
  LeadSurrogates,
  /// 孤立尾随代理
  TrailSurrogates,
  /// 非 BMP (代理对)
  NonBmp,
}

/// 原版 10 条 static_assert 的承接 (段边界无缝衔接性)
pub const K_SEGMENT_COUNT: usize = 5;

// ── UC16Range 打包 (cc:297-306: {from,to} 压入单 u32) ──
#[must_use]
pub const fn to_uc16_range(from: u16, to: u16) -> u32 {
  (u32::from(from) << 16) | u32::from(to)
}
#[must_use]
pub const fn extract_from(r: u32) -> u16 {
  (r >> 16) as u16
}
#[must_use]
pub const fn extract_to(r: u32) -> u16 {
  r as u16
}

// ── 展开阈值 ──
/// 类编译内联区间上限 (cc:625)
pub const K_MAX_RANGES_TO_INLINE: usize = 32;
/// 类展开放大因子上限 (cc:2198)
pub const K_MAX_EXPANSION_FACTOR: i32 = 6;
/// min 展开上限 (cc:2262)
pub const K_MAX_UNROLLED_MIN_MATCHES: i32 = 3;
/// max 展开上限 (cc:2263)
pub const K_MAX_UNROLLED_MAX_MATCHES: i32 = 3;
/// CRLF 行终止符对 (cc:1292)
pub const K_CRLF: [u16; 2] = [0x0d, 0x0a];

const _: () = assert!(K_SEGMENT_COUNT == 5);
const _: () = assert!(K_SEGMENT_STARTS[0] == 0 && K_SEGMENT_ENDS[0] == 0xd7ff);
const _: () = assert!(K_SEGMENT_STARTS[1] == 0xd800 && K_SEGMENT_ENDS[1] == 0xdbff);
const _: () = assert!(K_SEGMENT_STARTS[2] == 0xdc00 && K_SEGMENT_ENDS[2] == 0xdfff);
const _: () = assert!(K_SEGMENT_STARTS[3] == 0xe000 && K_SEGMENT_ENDS[3] == 0xffff);
const _: () = assert!(K_SEGMENT_STARTS[4] == 0x10000 && K_SEGMENT_ENDS[4] == 0x10ffff);
const _: () = assert!(K_SEGMENT_STARTS[1] == K_SEGMENT_ENDS[0] + 1);
const _: () = assert!(K_SEGMENT_STARTS[2] == K_SEGMENT_ENDS[1] + 1);
const _: () = assert!(K_SEGMENT_STARTS[3] == K_SEGMENT_ENDS[2] + 1);
const _: () = assert!(K_SEGMENT_STARTS[4] == K_SEGMENT_ENDS[3] + 1);
const _: () = assert!(to_uc16_range(0x1234, 0x5678) == 0x1234_5678);
const _: () = assert!(extract_from(0x1234_5678) == 0x1234 && extract_to(0x1234_5678) == 0x5678);
