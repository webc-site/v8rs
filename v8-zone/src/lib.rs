//! v8-zone: V8 `src/zone` 的 Rust 转写 (L0, arena 分配器)
//!
//! 对外 API 集中于本文件 `pub use`, 内部模块不向外泄漏。
//! 生命周期模型: `Zone<'a>` 借用 `AccountingAllocator`, 容器与分配结果
//! 经 `&'z Zone` 引用参数化获得 `'z` 借用, 取代原版裸 `Zone*`/`ZonePointer`。

#![cfg_attr(docsrs, feature(doc_cfg))]

mod containers;
mod error;
mod internal;
mod kernel;
mod zone;

/// 原版 using IntVector = ZoneVector<int>
pub use containers::IntVector;
/// 原版 zone-type-traits.h ZonePtrList (ZoneList<T*> 别名)
pub use containers::ZonePtrList;
pub use containers::{
  allocator::{RecyclingZoneAllocator, ZoneAllocator, ZoneBoolAllocator, ZoneIntAllocator},
  compact_set::{ZoneCompactSet, ZoneCompactSetIter, ZoneCompactSetTraits},
  deque::{ZoneDeque, ZoneQueue, ZoneStack},
  linked::{ZoneForwardList, ZoneForwardListIter, ZoneLinkedList, ZoneLinkedListIter},
  list::ZoneList,
  vector::ZoneVector,
};
pub use error::{Error, Result};
/// 原版 AccountingAllocator + TracingFlags::IsZoneStatsEnabled 入口
pub use zone::accounting::{AccountingAllocator, set_zone_stats_enabled, zone_stats_enabled};
/// 原版 v8_flags.trace_zone_type_stats 入口 (仅精确统计构建)
#[cfg(feature = "precise-zone-stats")]
pub use zone::accounting::{set_trace_zone_type_stats_enabled, trace_zone_type_stats_enabled};
/// 原版 Zone/ZoneSnapshot/ZoneScope/ZoneAllocationPolicy
pub use zone::core::{Zone, ZoneAllocationPolicy, ZoneScope, ZoneSnapshot};
/// 原版 Segment (供诊断/遍历段链使用)
pub use zone::segment::Segment;
/// 原版 type-stats.h TypeStats/StatsEntry (仅精确统计构建)
#[cfg(feature = "precise-zone-stats")]
pub use zone::type_stats::{StatsEntry, TypeStats};
