//! 原版 src/zone 核心模块目录 (zone.h/cc, zone-segment.h/cc, accounting-allocator.h/cc)

pub(crate) mod accounting;
pub(crate) mod core;
pub(crate) mod segment;
/// 原版 type-stats.h/cc (仅 V8_ENABLE_PRECISE_ZONE_STATS 构建存在)
#[cfg(feature = "precise-zone-stats")]
pub(crate) mod type_stats;
