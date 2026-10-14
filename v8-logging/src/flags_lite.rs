//! v8-flags 的最小私有持有者(仅日志/统计相关的少量开关)。
//!
//! 【待回收】v8-flags 转写完成后,本模块整体删除,改用统一 flag 常量。
//! 默认值取自 `v8/src/flags/flag-definitions.h`:
//! - `histogram_interval` = 600000(ms)
//! - `log`/`log_all`/`log_deopt`/`log_timer_events`/`prof_cpp`/`rcs_cpu_time`
//!   /`verify_predictable`/`slow_histograms` = false
//! - `prof_sampling_interval` = 1000(ms)
//! - `log_function_events`/`log_code`/`predictable`/`log_source_code`
//!   /`log_maps`/`log_source_position` = false(L4056/L4048/L4246/L4046/L3355/L4047)
//! - `logfile_per_isolate` = **true**(L4036;d8 独立 shell 时经
//!   `DEFINE_WEAK_NEG_IMPLICATION` 关闭,该 implication 不在本层复刻)
//! - `log_ic` = false(L3308)/ `log_feedback_vector` = false(L4052)
//!   /`log_code_disassemble` = false(L4053)/ `prof_include_idle` = false(L4063)
//!   /`interpreted_frames_native_stack` = false(L4177)/ `log_maps_details` =
//!   **true**(L3356) —— round1 全集核验补入(原版引用点: log.cc:2041/1566/1490/
//!   1150+2012/135/2147)
//! - `logfile` = "v8.log"(字符串,L4033;消费者 V8FileLogger::Initialize
//!   log.cc:2396 推迟,备查持位)
//!
//! implication 机制不在本层复刻(同 logfile_per_isolate 决策):
//! `log_ic→log_code`(L3309)/`log_maps→log_code`(L3357)/
//! `log_code_disassemble→log_code`(L4055)/`log_code→log_source_code`、
//! `log_code→log_source_position`(L4054-4056 WEAK)。
//!
//! 注: 老 V8 的 `--log-api`(log_api)在现源码树与 flag-definitions.h 均不存在,
//! `LOG(isolate, Call)` 宏族现门控为 `v8_flags.log`(log.h:78-81), 已由本模块
//! `log` 覆盖,故不设 log_api。

use std::sync::{
  OnceLock,
  atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering},
};

use parking_lot::RwLock;

pub const DEFAULT_HISTOGRAM_INTERVAL: f64 = 600000.0;
pub const DEFAULT_PROF_SAMPLING_INTERVAL: i64 = 1000;
/// 对应 `DEFINE_STRING(logfile, "v8.log", ...)`(flag-definitions.h:4033)。
pub const DEFAULT_LOGFILE: &str = "v8.log";

static HISTOGRAM_INTERVAL: AtomicU64 = AtomicU64::new(DEFAULT_HISTOGRAM_INTERVAL.to_bits());
static INTERPRETED_FRAMES_NATIVE_STACK: AtomicBool = AtomicBool::new(false);
static LOG: AtomicBool = AtomicBool::new(false);
static LOG_ALL: AtomicBool = AtomicBool::new(false);
static LOG_CODE: AtomicBool = AtomicBool::new(false);
static LOG_CODE_DISASSEMBLE: AtomicBool = AtomicBool::new(false);
static LOG_DEOPT: AtomicBool = AtomicBool::new(false);
static LOG_FEEDBACK_VECTOR: AtomicBool = AtomicBool::new(false);
static LOG_FUNCTION_EVENTS: AtomicBool = AtomicBool::new(false);
static LOG_IC: AtomicBool = AtomicBool::new(false);
static LOG_MAPS: AtomicBool = AtomicBool::new(false);
static LOG_MAPS_DETAILS: AtomicBool = AtomicBool::new(true);
static LOG_SOURCE_CODE: AtomicBool = AtomicBool::new(false);
static LOG_SOURCE_POSITION: AtomicBool = AtomicBool::new(false);
static LOG_TIMER_EVENTS: AtomicBool = AtomicBool::new(false);
static LOGFILE_PER_ISOLATE: AtomicBool = AtomicBool::new(true);
static PREDICTABLE: AtomicBool = AtomicBool::new(false);
static PROF_CPP: AtomicBool = AtomicBool::new(false);
static PROF_INCLUDE_IDLE: AtomicBool = AtomicBool::new(false);
static PROF_SAMPLING_INTERVAL: AtomicI64 = AtomicI64::new(DEFAULT_PROF_SAMPLING_INTERVAL);
static RCS_CPU_TIME: AtomicBool = AtomicBool::new(false);
static VERIFY_PREDICTABLE: AtomicBool = AtomicBool::new(false);
static SLOW_HISTOGRAMS: AtomicBool = AtomicBool::new(false);
/// `logfile` 字符串 flag(低频写;`OnceLock` 承载初始化,`RwLock` 承载读多写少)。
static LOGFILE: OnceLock<RwLock<Box<str>>> = OnceLock::new();

#[inline]
fn logfile_cell() -> &'static RwLock<Box<str>> {
  LOGFILE.get_or_init(|| RwLock::new(DEFAULT_LOGFILE.into()))
}

#[inline]
fn f64_bits(v: f64) -> u64 {
  v.to_bits()
}

/// 恢复所有 flag 的编译期默认值(对应 C++ `FlagList::SetDefault`/重置)。
pub fn reset_defaults() {
  HISTOGRAM_INTERVAL.store(f64_bits(DEFAULT_HISTOGRAM_INTERVAL), Ordering::Relaxed);
  INTERPRETED_FRAMES_NATIVE_STACK.store(false, Ordering::Relaxed);
  LOG.store(false, Ordering::Relaxed);
  LOG_ALL.store(false, Ordering::Relaxed);
  LOG_CODE.store(false, Ordering::Relaxed);
  LOG_CODE_DISASSEMBLE.store(false, Ordering::Relaxed);
  LOG_DEOPT.store(false, Ordering::Relaxed);
  LOG_FEEDBACK_VECTOR.store(false, Ordering::Relaxed);
  LOG_FUNCTION_EVENTS.store(false, Ordering::Relaxed);
  LOG_IC.store(false, Ordering::Relaxed);
  LOG_MAPS.store(false, Ordering::Relaxed);
  LOG_MAPS_DETAILS.store(true, Ordering::Relaxed);
  LOG_SOURCE_CODE.store(false, Ordering::Relaxed);
  LOG_SOURCE_POSITION.store(false, Ordering::Relaxed);
  LOG_TIMER_EVENTS.store(false, Ordering::Relaxed);
  LOGFILE_PER_ISOLATE.store(true, Ordering::Relaxed);
  PREDICTABLE.store(false, Ordering::Relaxed);
  PROF_CPP.store(false, Ordering::Relaxed);
  PROF_INCLUDE_IDLE.store(false, Ordering::Relaxed);
  PROF_SAMPLING_INTERVAL.store(DEFAULT_PROF_SAMPLING_INTERVAL, Ordering::Relaxed);
  RCS_CPU_TIME.store(false, Ordering::Relaxed);
  VERIFY_PREDICTABLE.store(false, Ordering::Relaxed);
  SLOW_HISTOGRAMS.store(false, Ordering::Relaxed);
  *logfile_cell().write() = DEFAULT_LOGFILE.into();
}

macro_rules! bool_flag {
  ($get:ident, $set:ident, $cell:ident) => {
    #[inline]
    pub fn $get() -> bool {
      $cell.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn $set(v: bool) {
      $cell.store(v, Ordering::Relaxed);
    }
  };
}

bool_flag!(
  interpreted_frames_native_stack,
  set_interpreted_frames_native_stack,
  INTERPRETED_FRAMES_NATIVE_STACK
);
bool_flag!(log, set_log, LOG);
bool_flag!(log_all, set_log_all, LOG_ALL);
bool_flag!(log_code, set_log_code, LOG_CODE);
bool_flag!(
  log_code_disassemble,
  set_log_code_disassemble,
  LOG_CODE_DISASSEMBLE
);
bool_flag!(log_deopt, set_log_deopt, LOG_DEOPT);
bool_flag!(
  log_feedback_vector,
  set_log_feedback_vector,
  LOG_FEEDBACK_VECTOR
);
bool_flag!(
  log_function_events,
  set_log_function_events,
  LOG_FUNCTION_EVENTS
);
bool_flag!(log_ic, set_log_ic, LOG_IC);
bool_flag!(log_maps, set_log_maps, LOG_MAPS);
bool_flag!(log_maps_details, set_log_maps_details, LOG_MAPS_DETAILS);
bool_flag!(log_source_code, set_log_source_code, LOG_SOURCE_CODE);
bool_flag!(
  log_source_position,
  set_log_source_position,
  LOG_SOURCE_POSITION
);
bool_flag!(log_timer_events, set_log_timer_events, LOG_TIMER_EVENTS);
bool_flag!(
  logfile_per_isolate,
  set_logfile_per_isolate,
  LOGFILE_PER_ISOLATE
);
bool_flag!(predictable, set_predictable, PREDICTABLE);
bool_flag!(prof_cpp, set_prof_cpp, PROF_CPP);
bool_flag!(prof_include_idle, set_prof_include_idle, PROF_INCLUDE_IDLE);
bool_flag!(rcs_cpu_time, set_rcs_cpu_time, RCS_CPU_TIME);
bool_flag!(
  verify_predictable,
  set_verify_predictable,
  VERIFY_PREDICTABLE
);
bool_flag!(slow_histograms, set_slow_histograms, SLOW_HISTOGRAMS);

/// 对应 `v8_flags.logfile`(字符串,默认 "v8.log")。
#[inline]
pub fn logfile() -> String {
  logfile_cell().read().clone().into_string()
}

/// 设置 `v8_flags.logfile`。
#[inline]
pub fn set_logfile(v: &str) {
  *logfile_cell().write() = v.into();
}

#[inline]
pub fn histogram_interval() -> f64 {
  f64::from_bits(HISTOGRAM_INTERVAL.load(Ordering::Relaxed))
}

#[inline]
pub fn set_histogram_interval(v: f64) {
  HISTOGRAM_INTERVAL.store(f64_bits(v), Ordering::Relaxed);
}

#[inline]
pub fn prof_sampling_interval() -> i64 {
  PROF_SAMPLING_INTERVAL.load(Ordering::Relaxed)
}

#[inline]
pub fn set_prof_sampling_interval(v: i64) {
  PROF_SAMPLING_INTERVAL.store(v, Ordering::Relaxed);
}
