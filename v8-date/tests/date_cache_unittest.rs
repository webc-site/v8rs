// 1:1 迁移自 v8/test/unittests/date/date-cache-unittest.cc
use std::{
  sync::{Arc, Mutex},
  thread,
};

use aok::{OK, Void};
use v8_date::date_cache::{
  cache::DateCache,
  provider::{SystemTimezoneProvider, TimeZoneDetection},
};

// 2019-05-08T04:16:04.845Z
const START_TIME: i64 = 1557288964845;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_adopt_default_first() -> Void {
  // 顺序执行各线程任务验证逻辑正确性
  let t1 = thread::spawn(|| {
    let mut dc = DateCache::default();
    dc.reset_date_cache(TimeZoneDetection::Redetect);
  });
  t1.join().unwrap();

  let t2 = thread::spawn(|| {
    let mut dc = DateCache::default();
    let _ = dc.get_local_offset_from_os(START_TIME, true);
  });
  let t3 = thread::spawn(|| {
    let mut dc = DateCache::default();
    let _ = dc.get_local_offset_from_os(START_TIME, false);
  });
  let t4 = thread::spawn(|| {
    let mut dc = DateCache::default();
    let _ = dc.local_timezone(START_TIME);
  });

  t2.join().unwrap();
  t3.join().unwrap();
  t4.join().unwrap();

  OK
}

#[test]
fn test_adopt_default_mixed() -> Void {
  // 并发混合执行
  let shared_cache = Arc::new(Mutex::new(DateCache::<SystemTimezoneProvider>::default()));

  let mut handles = Vec::new();

  // 重置时区线程
  let c1 = Arc::clone(&shared_cache);
  handles.push(thread::spawn(move || {
    let mut dc = c1.lock().unwrap();
    dc.reset_date_cache(TimeZoneDetection::Redetect);
  }));

  // UTC 偏移查询线程
  let c2 = Arc::clone(&shared_cache);
  handles.push(thread::spawn(move || {
    let mut dc = c2.lock().unwrap();
    let _ = dc.get_local_offset_from_os(START_TIME, true);
  }));

  // 本地偏移查询线程
  let c3 = Arc::clone(&shared_cache);
  handles.push(thread::spawn(move || {
    let mut dc = c3.lock().unwrap();
    let _ = dc.get_local_offset_from_os(START_TIME, false);
  }));

  // 时区名称查询线程
  let c4 = Arc::clone(&shared_cache);
  handles.push(thread::spawn(move || {
    let mut dc = c4.lock().unwrap();
    let _ = dc.local_timezone(START_TIME);
  }));

  for handle in handles {
    handle.join().unwrap();
  }

  OK
}
