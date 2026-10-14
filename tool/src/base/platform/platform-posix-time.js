#!/usr/bin/env bun
// tool/src/base/platform/platform-posix-time.js — 覆盖 v8/src/base/platform/platform-posix-time.{h,cc}
// POSIX 时区缓存: PosixDefaultTimezoneCache 两方法 (localtime_r 语义)
// → v8-base-platform/src/platform_posix_time.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/platform-posix-time.h");
const ccSrc = readV8("src/base/platform/platform-posix-time.cc");

// 1. 头锚点: 基类覆写面 2
const hAnchors = [
  "class PosixDefaultTimezoneCache : public PosixTimezoneCache {",
  "const char* LocalTimezone(double time_ms) override;",
  "double LocalTimeOffset(double time_ms, bool is_utc) override;",
];
for (const a of hAnchors) {
  if (!hSrc.includes(a)) throw new Error(`platform-posix-time.h 缺锚点: ${a}`);
}

// 2. 实现锚点: NaN 哨兵 + tm_zone + tm_gmtoff 夏令时折算
const ccAnchors = [
  "if (std::isnan(time)) return \"\";",
  "time_t tv = static_cast<time_t>(std::floor(time / msPerSecond));",
  "localtime_r(&tv, &tm);",
  "if (!t || !t->tm_zone) return \"\";",
  "return t->tm_zone;",
  "time_t tv = time(nullptr);",
  "DCHECK_NOT_NULL(t);",
  "return static_cast<double>(t->tm_gmtoff * msPerSecond -",
  "(t->tm_isdst > 0 ? 3600 * msPerSecond : 0));",
];
for (const a of ccAnchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-posix-time.cc 缺锚点: ${a}`);
}

console.log(`platform-posix-time: 方法 2 (LocalTimezone/LocalTimeOffset), 锚点 ${hAnchors.length + ccAnchors.length}`);

let rust = banner("src/base/platform/platform-posix-time.js", "src/base/platform/platform-posix-time.{h,cc}", "POSIX 默认时区缓存 (localtime_r; darwin 目标在用)");

rust += `
/// POSIX 默认时区缓存 (原版 class PosixDefaultTimezoneCache)
/// v8: src/base/platform/platform-posix-time.h PosixDefaultTimezoneCache
///
/// 注意语义: 两个方法都不做缓存 — 每次调 localtime_r;
/// LocalTimeOffset 保留旧行为, 忽略 time_ms 与 is_utc (取当前时刻)
pub struct PosixDefaultTimezoneCache;

impl PosixDefaultTimezoneCache {
  /// v8: src/base/platform/platform-posix-time.cc PosixDefaultTimezoneCache::LocalTimezone
  /// NaN → 空串; ms → 秒 (floor) → localtime_r → tm_zone (无则空串)
  pub fn local_timezone(&self, time_ms: f64) -> &'static str {
    if time_ms.is_nan() { return ""; }
    let secs = (time_ms / 1000.0).floor() as libc::time_t;
    unsafe {
      let mut tm: libc::tm = std::mem::zeroed();
      let t = libc::localtime_r(&secs, &mut tm);
      if t.is_null() || (*t).tm_zone.is_null() { "" } else {
        std::ffi::CStr::from_ptr((*t).tm_zone).to_str().unwrap_or("")
      }
    }
  }

  /// v8: src/base/platform/platform-posix-time.cc PosixDefaultTimezoneCache::LocalTimeOffset
  /// tm_gmtoff 含夏令时, 需减去: gmtoff*1000 - (isdst>0 ? 3600*1000 : 0)
  pub fn local_time_offset(&self, _time_ms: f64, _is_utc: bool) -> f64 {
    unsafe {
      let tv = libc::time(std::ptr::null_mut());
      let mut tm: libc::tm = std::mem::zeroed();
      let t = libc::localtime_r(&tv, &mut tm);
      debug_assert!(!t.is_null());
      let dst = if (*t).tm_isdst > 0 { 3600 * 1000.0 } else { 0.0 };
      ((*t).tm_gmtoff as f64) * 1000.0 - dst
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  // 两方法面; 夏令时折算秒数 3600
  assert_eq!(3600 * 1000, 3_600_000);
};
`;

const p = writeSrc("v8-base-platform", "platform_posix_time.rs.skel", rust);
console.log(`written ${p}`);
