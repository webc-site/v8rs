//! `src/counters.rs` 测试。1:1 规约来源:
//! `v8/test/unittests/logging/counters-unittest.cc` 的
//! `AggregatedMemoryHistogramTest`(14 用例全迁)。
//!
//! 跳过负登记: `SnapshotNativeCounterTest.WriteBarrier`(counters-unittest.cc:214)
//! 需 NativeContext/RunJS + `V8_SNAPSHOT_NATIVE_CODE_COUNTERS`(引擎级, M4+ 回收)。

use std::cell::RefCell;

use aok::{OK, Void};
use log::info;
use v8_logging::{
  AggregatedMemoryHistogram, DEFAULT_HISTOGRAM_INTERVAL, SampleSink, set_histogram_interval,
};

/// 对应 `MockHistogram`(counters-unittest.cc:23-30):采样收集器。
struct MockHistogram {
  samples: RefCell<Vec<i32>>,
}

impl MockHistogram {
  fn new() -> Self {
    Self {
      samples: RefCell::new(Vec::new()),
    }
  }

  fn samples(&self) -> Vec<i32> {
    self.samples.borrow().clone()
  }
}

impl SampleSink for MockHistogram {
  fn add_sample(&self, sample: i32) {
    self.samples.borrow_mut().push(sample);
  }
}

/// interval 守卫:Drop 恢复默认,防跨用例污染(gtest fixture 等价物)。
struct IntervalGuard;

impl IntervalGuard {
  /// 对应 fixture 逐用例 `v8_flags.histogram_interval = 10`。
  fn new() -> Self {
    set_histogram_interval(10.0);
    Self
  }
}

impl Drop for IntervalGuard {
  fn drop(&mut self) {
    set_histogram_interval(DEFAULT_HISTOGRAM_INTERVAL);
  }
}

/// 对应 fixture `AggregatedMemoryHistogramTest`:构造 `aggregated_(&mock_)` 并
/// 执行用例体(借用形状:Rust 无自引用结构,以闭包承接)。
fn with_histogram(
  f: impl FnOnce(&mut AggregatedMemoryHistogram<'_, MockHistogram>, &MockHistogram) -> Void,
) -> Void {
  let _guard = IntervalGuard::new();
  let mock = MockHistogram::new();
  let mut aggregated = AggregatedMemoryHistogram::new(&mock);
  f(&mut aggregated, &mock)
}

// TEST_F(AggregatedMemoryHistogramTest, OneSample1)(counters-unittest.cc:77-84)
#[test]
fn aggregated_memory_histogram_test_one_sample1() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 1000.0);
    f.add_sample(20.0, 1000.0);
    assert_eq!(mock.samples().len(), 1);
    assert_eq!(mock.samples()[0], 1000);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, OneSample2)(counters-unittest.cc:85-92)
#[test]
fn aggregated_memory_histogram_test_one_sample2() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 500.0);
    f.add_sample(20.0, 1000.0);
    assert_eq!(mock.samples().len(), 1);
    assert_eq!(mock.samples()[0], 750);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, OneSample3)(counters-unittest.cc:93-102)
#[test]
fn aggregated_memory_histogram_test_one_sample3() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 500.0);
    f.add_sample(15.0, 500.0);
    f.add_sample(15.0, 1000.0);
    f.add_sample(20.0, 1000.0);
    assert_eq!(mock.samples().len(), 1);
    assert_eq!(mock.samples()[0], 750);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, OneSample4)(counters-unittest.cc:103-111)
#[test]
fn aggregated_memory_histogram_test_one_sample4() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 500.0);
    f.add_sample(15.0, 750.0);
    f.add_sample(20.0, 1000.0);
    assert_eq!(mock.samples().len(), 1);
    assert_eq!(mock.samples()[0], 750);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, TwoSamples1)(counters-unittest.cc:112-120)
#[test]
fn aggregated_memory_histogram_test_two_samples1() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 1000.0);
    f.add_sample(30.0, 1000.0);
    assert_eq!(mock.samples().len(), 2);
    assert_eq!(mock.samples()[0], 1000);
    assert_eq!(mock.samples()[1], 1000);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, TwoSamples2)(counters-unittest.cc:121-130)
#[test]
fn aggregated_memory_histogram_test_two_samples2() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 1000.0);
    f.add_sample(20.0, 1000.0);
    f.add_sample(30.0, 1000.0);
    assert_eq!(mock.samples().len(), 2);
    assert_eq!(mock.samples()[0], 1000);
    assert_eq!(mock.samples()[1], 1000);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, TwoSamples3)(counters-unittest.cc:131-141)
#[test]
fn aggregated_memory_histogram_test_two_samples3() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 1000.0);
    f.add_sample(20.0, 1000.0);
    f.add_sample(20.0, 500.0);
    f.add_sample(30.0, 500.0);
    assert_eq!(mock.samples().len(), 2);
    assert_eq!(mock.samples()[0], 1000);
    assert_eq!(mock.samples()[1], 500);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, TwoSamples4)(counters-unittest.cc:142-150)
#[test]
fn aggregated_memory_histogram_test_two_samples4() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 1000.0);
    f.add_sample(30.0, 0.0);
    assert_eq!(mock.samples().len(), 2);
    assert_eq!(mock.samples()[0], 750);
    assert_eq!(mock.samples()[1], 250);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, TwoSamples5)(counters-unittest.cc:151-159)
#[test]
fn aggregated_memory_histogram_test_two_samples5() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 0.0);
    f.add_sample(30.0, 1000.0);
    assert_eq!(mock.samples().len(), 2);
    assert_eq!(mock.samples()[0], 250);
    assert_eq!(mock.samples()[1], 750);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, TwoSamples6)(counters-unittest.cc:160-169)
#[test]
fn aggregated_memory_histogram_test_two_samples6() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 0.0);
    f.add_sample(15.0, 1000.0);
    f.add_sample(30.0, 1000.0);
    assert_eq!(mock.samples().len(), 2);
    assert_eq!(mock.samples()[0], (500 + 1000) / 2);
    assert_eq!(mock.samples()[1], 1000);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, TwoSamples7)(counters-unittest.cc:170-180)
#[test]
fn aggregated_memory_histogram_test_two_samples7() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 0.0);
    f.add_sample(15.0, 1000.0);
    f.add_sample(25.0, 0.0);
    f.add_sample(30.0, 1000.0);
    assert_eq!(mock.samples().len(), 2);
    assert_eq!(mock.samples()[0], (500 + 750) / 2);
    assert_eq!(mock.samples()[1], (250 + 500) / 2);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, TwoSamples8)(counters-unittest.cc:181-191)
#[test]
fn aggregated_memory_histogram_test_two_samples8() -> Void {
  with_histogram(|f, mock| {
    f.add_sample(10.0, 1000.0);
    f.add_sample(15.0, 0.0);
    f.add_sample(25.0, 1000.0);
    f.add_sample(30.0, 0.0);
    assert_eq!(mock.samples().len(), 2);
    assert_eq!(mock.samples()[0], (500 + 250) / 2);
    assert_eq!(mock.samples()[1], (750 + 500) / 2);
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, ManySamples1)(counters-unittest.cc:192-202)
#[test]
fn aggregated_memory_histogram_test_many_samples1() -> Void {
  with_histogram(|f, mock| {
    const K_MAX_SAMPLES: usize = 1000;
    f.add_sample(0.0, 0.0);
    f.add_sample(10.0 * K_MAX_SAMPLES as f64, 10.0 * K_MAX_SAMPLES as f64);
    let samples = mock.samples();
    assert_eq!(samples.len(), K_MAX_SAMPLES);
    for (i, s) in samples.iter().enumerate() {
      assert_eq!(*s, (i * 10 + 5) as i32, "sample {i}");
    }
    info!("many samples1 ok");
    OK
  })
}

// TEST_F(AggregatedMemoryHistogramTest, ManySamples2)(counters-unittest.cc:203-213)
#[test]
fn aggregated_memory_histogram_test_many_samples2() -> Void {
  with_histogram(|f, mock| {
    const K_MAX_SAMPLES: usize = 1000;
    f.add_sample(0.0, 0.0);
    f.add_sample(
      10.0 * (2 * K_MAX_SAMPLES) as f64,
      10.0 * (2 * K_MAX_SAMPLES) as f64,
    );
    let samples = mock.samples();
    assert_eq!(samples.len(), K_MAX_SAMPLES);
    for (i, s) in samples.iter().enumerate() {
      assert_eq!(*s, (i * 10 + 5) as i32, "sample {i}");
    }
    OK
  })
}

// 回归(round3): `TimedHistogram::RecordAbandon` 的 MILLISECOND 分支样本为 -1。
// 原版链路: time.cc:234-240 `TimeDelta::Max().InMilliseconds()` 保持 i64::MAX →
// counters.cc:79-82 `static_cast<int>` 低 32 位截断 = -1;
// 若 `in_milliseconds` 改为整除截断(i64::MAX/1000), 样本会错成 2147482615。
#[test]
fn timed_histogram_record_abandon_max_sample() -> Void {
  use std::sync::Arc;

  use parking_lot::Mutex;
  use v8_logging::{
    Counters, ElapsedTimer, HistogramHandle, StatsTable, TimeDelta, TimedHistogramResolution,
  };

  static SAMPLES: Mutex<Vec<i32>> = Mutex::new(Vec::new());

  fn create_cb(_n: &str, _min: i32, _max: i32, _b: usize) -> Option<HistogramHandle> {
    Some(0x1234)
  }
  fn add_cb(_h: HistogramHandle, sample: i32) {
    SAMPLES.lock().push(sample);
  }

  let table = Arc::new(StatsTable::new());
  table.set_create_histogram_function(create_cb);
  table.set_add_histogram_sample_function(add_cb);
  let counters = Counters::new(table);
  // MILLISECOND 分辨率(counters-definitions.h:245)。
  let hist = counters.measure_memory_delay_ms();
  assert!(hist.enabled());
  assert_eq!(hist.resolution(), TimedHistogramResolution::Millisecond);
  // TimeDelta::Max() 的直查规约(等价 C++ `Max().InMilliseconds()`)。
  assert_eq!(TimeDelta::max().in_milliseconds(), i64::MAX);
  let mut timer = ElapsedTimer::new();
  timer.start();
  hist.record_abandon(&mut timer);
  assert_eq!(SAMPLES.lock().last(), Some(&-1));
  OK
}

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}
