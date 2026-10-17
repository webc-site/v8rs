#!/usr/bin/env bun
// tool/src/base/timezone-cache.h.js — 覆盖 v8/src/base/timezone-cache.h
// 解析 TimezoneCache 抽象类 4 纯虚方法与 TimeZoneDetection 枚举
// → v8-base/src/timezone_cache.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/timezone-cache.h");

for (const anchor of [
  "V8_BASE_TIMEZONE_CACHE_H_",
  "class TimezoneCache",
  "virtual const char* LocalTimezone(double time_ms) = 0;",
  "virtual double DaylightSavingsOffset(double time_ms) = 0;",
  "virtual double LocalTimeOffset(double time_ms, bool is_utc) = 0;",
  "virtual void Clear(TimeZoneDetection time_zone_detection) = 0;",
  "enum class TimeZoneDetection { kSkip, kRedetect };",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 纯虚方法签名
const virtuals = [...h.matchAll(/virtual ([\w:<>*& ]+?) (\w+)\(([^)]*)\) = 0;/g)].map((m) => ({
  ret: m[1].trim(),
  name: m[2],
  args: m[3].trim(),
}));
// 枚举
const tzEnum = h.match(/enum class TimeZoneDetection \{ (\w+), (\w+) \};/);
if (!tzEnum) throw new Error("TimeZoneDetection enum not found");

const body = `${banner("src/base/timezone-cache.h.js", "src/base/timezone-cache.h", "时区缓存抽象接口")}
//! v8 侧: 纯虚方法 ${virtuals.length} 个; 枚举 1 个 (2 变体)

/// v8: src/base/timezone-cache.h enum class TimeZoneDetection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeZoneDetection {
    /// v8: ${tzEnum[1]} — 不重探测
    Skip,
    /// v8: ${tzEnum[2]} — 重探测并设默认时区
    Redetect,
}

/// v8: src/base/timezone-cache.h class TimezoneCache (虚基类)
/// Rust: trait 替代虚表; 实现体在 platform 层 (Platform::TimeZoneCache)
pub trait TimezoneCache {
    /// v8: LocalTimezone(time_ms) — 本地时区短名 (如 EST)
    fn local_timezone(&self, time_ms: f64) -> &str;
    /// v8: DaylightSavingsOffset(time_ms) — 夏令时偏移 (ECMA-262 daylight-saving-time-adjustment)
    fn daylight_savings_offset(&self, time_ms: f64) -> f64;
    /// v8: LocalTimeOffset(time_ms, is_utc) — 本地时区偏移 (local-time-zone-adjustment)
    fn local_time_offset(&self, time_ms: f64, is_utc: bool) -> f64;
    /// v8: Clear(time_zone_detection) — 时区变更通知
    fn clear(&mut self, time_zone_detection: TimeZoneDetection);
}
`;

writeSrc("v8-base", "timezone_cache.rs.skel", body);
console.log(`timezone-cache.h.js: virtuals=${virtuals.length} enum=2 variants`);
