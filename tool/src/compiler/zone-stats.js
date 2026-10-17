#!/usr/bin/env bun
// tool/src/compiler/zone-stats.js
// v8/src/compiler/zone-stats.h + zone-stats.cc
// → v8-compiler-passes/src/zone_stats.rs.skel
// 编译用 zone 记账: 解析 Scope/StatsScope RAII 面、kMaxUnusedSize 与
// cc 的 ZoneReturned 回调对数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/zone-stats.h");
const cc = readV8("src/compiler/zone-stats.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`zone-stats: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE ZoneStats final {", "ZoneStats 定义"],
  ["class V8_NODISCARD Scope final {", "Scope"],
  ["class V8_EXPORT_PRIVATE V8_NODISCARD StatsScope final {", "StatsScope"],
  ["static const size_t kMaxUnusedSize = 3;", "kMaxUnusedSize"],
  ["void ZoneStats::StatsScope::ZoneReturned(Zone* zone)", "cc ZoneReturned"],
  ["Zone* ZoneStats::NewEmptyZone(const char* zone_name)", "cc NewEmptyZone"],
  ["void ZoneStats::ReturnZone(Zone* zone)", "cc ReturnZone"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// kMaxUnusedSize 真解析
const m = h.match(/kMaxUnusedSize = (\d+)/);
need(m && m[1] === "3", "kMaxUnusedSize != 3");

// StatsScope 公有方法 (头文件)
const statsGetters = ["GetMaxAllocatedBytes", "GetCurrentAllocatedBytes", "GetTotalAllocatedBytes"];
for (const g of statsGetters) need(h.includes(`size_t ${g}();`), `StatsScope 缺 ${g}`);

// ZoneStats 公有 getter
const zoneGetters = ["GetMaxAllocatedBytes() const", "GetTotalAllocatedBytes() const", "GetCurrentAllocatedBytes() const"];
for (const g of zoneGetters) need(h.includes(`size_t ${g};`) || h.includes(`size_t ${g} {`), `ZoneStats 缺 ${g}`);

// cc 定义清单
const ccDefs = [
  "ZoneStats::StatsScope::StatsScope(ZoneStats* zone_stats)",
  "ZoneStats::StatsScope::~StatsScope()",
  "size_t ZoneStats::StatsScope::GetMaxAllocatedBytes()",
  "size_t ZoneStats::StatsScope::GetCurrentAllocatedBytes()",
  "size_t ZoneStats::StatsScope::GetTotalAllocatedBytes()",
  "void ZoneStats::StatsScope::ZoneReturned(Zone* zone)",
  "ZoneStats::ZoneStats(AccountingAllocator* allocator)",
  "ZoneStats::~ZoneStats()",
  "size_t ZoneStats::GetMaxAllocatedBytes() const",
  "size_t ZoneStats::GetTotalAllocatedBytes() const",
  "size_t ZoneStats::GetCurrentAllocatedBytes() const",
  "Zone* ZoneStats::NewEmptyZone(const char* zone_name)",
  "void ZoneStats::ReturnZone(Zone* zone)",
];
for (const d of ccDefs) need(cc.includes(d), `cc 缺 ${d}`);

const body = `${banner("src/compiler/zone-stats.js", "src/compiler/zone-stats.h + zone-stats.cc", "编译期多 zone 分配记账 (Scope 借还 / StatsScope 统计)")}
//! zone-stats — zone 生命周期与内存统计中枢
//!
//! Scope: 惰性 NewEmptyZone, 析构 ReturnZone (zone 复用: unused 余量
//! <= kMaxUnusedSize 时保留); StatsScope: 记 max/current/total 三口径。

/// 原版 kMaxUnusedSize — zone 复用的剩余容量阈值
pub const K_MAX_UNUSED_SIZE: usize = ${m[1]};

/// 原版 ZoneStats::Scope — 借出/归还 RAII
pub struct ZoneScope<'s> {
  /// zone_name_
  pub zone_name: &'static str,
  /// zone_stats_
  pub zone_stats: &'s mut ZoneStats,
  /// zone_
  pub zone: Option<&'s mut Zone<'s>>,
  /// support_zone_compression_
  pub support_zone_compression: bool,
}

impl ZoneScope<'_> {
  /// 原版 zone() — 惰性建空 zone
  /// v8: src/compiler/zone-stats.h ZoneStats::Scope::zone
  pub fn zone(&mut self) -> &mut Zone<'_> {
    todo!("v8: src/compiler/zone-stats.h ZoneStats::Scope::zone")
  }

  /// 原版 Destroy — 立即归还 (析构语义)
  /// v8: src/compiler/zone-stats.h ZoneStats::Scope::Destroy
  pub fn destroy(&mut self) {
    todo!("v8: src/compiler/zone-stats.cc ReturnZone 路径")
  }
}

/// 原版 ZoneStats::StatsScope — 统计区间
pub struct StatsScope<'s> {
  /// zone_stats_
  pub zone_stats: &'s mut ZoneStats,
  /// initial_values_ — 各 zone 起始分配量
  pub initial_values: HashMap<*mut Zone, usize>,
  /// total_allocated_bytes_at_start_
  pub total_allocated_bytes_at_start: usize,
  /// max_allocated_bytes_
  pub max_allocated_bytes: usize,
}

impl StatsScope<'_> {
  /// 原版 GetMaxAllocatedBytes
  /// v8: src/compiler/zone-stats.cc ZoneStats::StatsScope::GetMaxAllocatedBytes
  pub fn get_max_allocated_bytes(&self) -> usize {
    self.max_allocated_bytes
  }

  /// 原版 GetCurrentAllocatedBytes — 各活 zone 当前量减起始量
  /// v8: src/compiler/zone-stats.cc ZoneStats::StatsScope::GetCurrentAllocatedBytes
  pub fn get_current_allocated_bytes(&self) -> usize {
    todo!("v8: src/compiler/zone-stats.cc ZoneStats::StatsScope::GetCurrentAllocatedBytes")
  }

  /// 原版 GetTotalAllocatedBytes
  /// v8: src/compiler/zone-stats.cc ZoneStats::StatsScope::GetTotalAllocatedBytes
  pub fn get_total_allocated_bytes(&self) -> usize {
    todo!("v8: src/compiler/zone-stats.cc ZoneStats::StatsScope::GetTotalAllocatedBytes")
  }

  /// 原版 ZoneReturned — zone 归还时把增量并入统计
  /// v8: src/compiler/zone-stats.cc ZoneStats::StatsScope::ZoneReturned
  pub fn zone_returned(&mut self, zone: &Zone<'_>) {
    todo!("v8: src/compiler/zone-stats.cc ZoneStats::StatsScope::ZoneReturned")
  }
}

/// 原版 ZoneStats — zone 池 + 双计数器
pub struct ZoneStats {
  /// zones_
  pub zones: Vec<*mut Zone<'static>>,
  /// stats_
  pub stats: Vec<*mut StatsScope<'static>>,
  /// max_allocated_bytes_
  pub max_allocated_bytes: usize,
  /// total_deleted_bytes_
  pub total_deleted_bytes: usize,
  /// allocator_
  pub allocator: *mut AccountingAllocator,
}

impl ZoneStats {
  /// 原版 GetMaxAllocatedBytes / GetTotalAllocatedBytes / GetCurrentAllocatedBytes
  /// v8: src/compiler/zone-stats.cc ZoneStats::GetMaxAllocatedBytes
  pub fn get_max_allocated_bytes(&self) -> usize {
    self.max_allocated_bytes
  }

  /// v8: src/compiler/zone-stats.cc ZoneStats::GetTotalAllocatedBytes
  pub fn get_total_allocated_bytes(&self) -> usize {
    todo!("v8: src/compiler/zone-stats.cc ZoneStats::GetTotalAllocatedBytes")
  }

  /// v8: src/compiler/zone-stats.cc ZoneStats::GetCurrentAllocatedBytes
  pub fn get_current_allocated_bytes(&self) -> usize {
    todo!("v8: src/compiler/zone-stats.cc ZoneStats::GetCurrentAllocatedBytes")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "zone_stats.rs.skel", body);
console.log(`written ${p}\nkMaxUnusedSize=${m[1]} stats_getters=${statsGetters.length} cc_defs=${ccDefs.length}`);
