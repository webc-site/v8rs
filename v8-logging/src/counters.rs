//! 1:1 转写自 `v8/src/logging/counters.h/.cc` + `counters-definitions.h`
//! + `counters-scopes.h`。
//!
//! 转写要点:
//! - `counters-definitions.h` 的 X-macro 清单机械展开为宏输入数据
//!   (登记于本文件 `define_counters!` 调用,禁止"理解"宏语义)。
//! - C++ `StatsTable` 的函数指针回调(`int*(*)(const char*)` 等)以
//!   `fn` 指针 + `Mutex` 承载;`void*` histogram 句柄 → `HistogramHandle(u64, 0=未创建)`。
//! - `Histogram::EnsureCreated` 的双检锁(atomic acquire + Mutex)原样保留。
//! - `NestedTimedHistogramScope` 的裸指针链改为同一 `Mutex<Vec<ElapsedTimer>>`
//!   栈实现(语义一致,登记差异)。
//! - `TimedHistogram::ToggleRunningState`(仅 DEBUG 构建的嵌套计时检查)未移植。
//! - `CountersVisitor`/`CountersInitializer`/`StatsCounterResetter`/`HistogramResetter`
//!   的遍历初始化在本移植中由 `define_counters!` 生成的构造函数与
//!   `reset_counter_function`/`reset_create_histogram_function` 直接完成。
//! - `Histogram::AddSample` 的 `V8.SandboxedExternalPointersCount` 等
//!   上限常量取 **桌面默认配置**(V8_ENABLE_SANDBOX 默认,64 位,
//!   COMPRESS_POINTERS):512MB/8 与 64MB/8 等;【待回收】v8-common globals。
//! - `StatsCounter::Initialize` 的 `DCHECK(name 以 "c:V8." 开头)` 保留。
//!
//! 【推迟登记】`DelayedCounterUpdates::PublishImpl(Isolate*)` 改为
//! `publish(&Counters)`;`Counters(Isolate*)` 构造改为 `Counters::new(Arc<StatsTable>)`。

use std::{
  cell::Cell,
  mem::{size_of, swap},
  sync::{
    Arc,
    atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering},
  },
};

use parking_lot::Mutex;

use crate::{
  flags_lite,
  runtime_call_stats::{RuntimeCallStats, ThreadType, WorkerThreadRuntimeCallStats},
  time_base::{ElapsedTimer, TimeDelta, TimeTicks},
};

/// 对应 `include/v8-internal.h` 的 `KB/MB/GB`(此处按直方图参数用 i32 截取,
/// 数值均在 i32 范围内)。【待回收】v8-common/v8-internal 常量。
pub const KB: i32 = 1024;
pub const MB: i32 = 1024 * KB;
pub const GB: i32 = 1024 * MB;
/// 对应 `kGarbageCollectionReasonMaxValue`(include/v8-internal.h)。
pub const K_GARBAGE_COLLECTION_REASON_MAX_VALUE: i32 = 30;
/// 桌面默认(V8_ENABLE_SANDBOX,非 Android/iOS):external = 512MB/8。
pub const K_MAX_EXTERNAL_POINTERS: i32 = 512 * MB / 8;
/// trusted = 64MB/8。
pub const K_MAX_TRUSTED_POINTERS: i32 = 64 * MB / 8;
/// cppheap = 512MB/8。
pub const K_MAX_CPP_HEAP_POINTERS: i32 = 512 * MB / 8;
/// js dispatch = 16 * entry * MB / entry = 16MB 条目数。
pub const K_MAX_JS_DISPATCH_ENTRIES: i32 = 16 * MB;

/// 对应 `DefaultTimedHistogramNumBuckets`(counters.cc)。
pub const DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS: i32 = 50;

/// 对应 `enum class TimedHistogramResolution { MILLISECOND, MICROSECOND }`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimedHistogramResolution {
  Millisecond,
  Microsecond,
}

/// X-macro 数据表中 `MICROSECOND` 单元。
pub const MICROSECOND: TimedHistogramResolution = TimedHistogramResolution::Microsecond;
/// X-macro 数据表中 `MILLISECOND` 单元。
pub const MILLISECOND: TimedHistogramResolution = TimedHistogramResolution::Millisecond;

/// 对应 C++ `void*` histogram 句柄;0 即 `nullptr`(未创建)。
pub type HistogramHandle = u64;

/// 对应 `CounterLookupCallback`:`int* (*)(const char*)`。
pub type CounterLookupCallback = fn(&str) -> Option<Arc<AtomicI32>>;
/// 对应 `CreateHistogramCallback`:`void* (*)(const char*, int, int, size_t)`。
pub type CreateHistogramCallback = fn(&str, i32, i32, usize) -> Option<HistogramHandle>;
/// 对应 `AddHistogramSampleCallback`:`void (*)(void*, int)`。
pub type AddHistogramSampleCallback = fn(HistogramHandle, i32);

/// 对应 `class StatsTable`。
pub struct StatsTable {
  lookup_function: Mutex<Option<CounterLookupCallback>>,
  create_histogram_function: Mutex<Option<CreateHistogramCallback>>,
  add_histogram_sample_function: Mutex<Option<AddHistogramSampleCallback>>,
}

impl StatsTable {
  pub fn new() -> Self {
    Self {
      lookup_function: Mutex::new(None),
      create_histogram_function: Mutex::new(None),
      add_histogram_sample_function: Mutex::new(None),
    }
  }

  /// 对应 `SetCounterFunction`。
  pub fn set_counter_function(&self, f: CounterLookupCallback) {
    *self.lookup_function.lock() = Some(f);
  }

  /// 对应 `SetCreateHistogramFunction`。
  pub fn set_create_histogram_function(&self, f: CreateHistogramCallback) {
    *self.create_histogram_function.lock() = Some(f);
  }

  /// 对应 `SetAddHistogramSampleFunction`。
  pub fn set_add_histogram_sample_function(&self, f: AddHistogramSampleCallback) {
    *self.add_histogram_sample_function.lock() = Some(f);
  }

  /// 对应 `HasCounterFunction`。
  pub fn has_counter_function(&self) -> bool {
    self.lookup_function.lock().is_some()
  }

  /// 对应 `FindLocation`。
  pub fn find_location(&self, name: &str) -> Option<Arc<AtomicI32>> {
    // The return value must not be cached and reused across threads
    // (C++ 注释保留;Rust 以 Arc 提供线程共享的原子槽位)。
    let f = *self.lookup_function.lock();
    f.and_then(|f| f(name))
  }

  /// 对应 `CreateHistogram`。
  pub fn create_histogram(
    &self,
    name: &str,
    min: i32,
    max: i32,
    buckets: usize,
  ) -> HistogramHandle {
    let f = *self.create_histogram_function.lock();
    f.and_then(|f| f(name, min, max, buckets)).unwrap_or(0)
  }

  /// 对应 `AddHistogramSample`。
  pub fn add_histogram_sample(&self, histogram: HistogramHandle, sample: i32) {
    let f = *self.add_histogram_sample_function.lock();
    if let Some(f) = f {
      f(histogram, sample);
    }
  }
}

impl Default for StatsTable {
  fn default() -> Self {
    Self::new()
  }
}

/// 对应 counters.cc 匿名空间的 `unused_counter_dump`。
static UNUSED_COUNTER_DUMP: AtomicI32 = AtomicI32::new(0);

#[derive(Clone)]
enum CounterPtr {
  External(Arc<AtomicI32>),
  /// 对应 C++ 缓存 `&unused_counter_dump`。
  Unused,
}

/// 对应 `class StatsCounter`(32 位命名计数器,槽位来自 StatsTable)。
pub struct StatsCounter {
  counters: Arc<StatsTable>,
  name: String,
  /// 对应 `std::atomic<std::atomic<int>*> ptr_`(缓存的槽位)。
  ptr: Mutex<Option<CounterPtr>>,
}

impl StatsCounter {
  fn new(counters: Arc<StatsTable>, name: String) -> Self {
    // Counter names always start with "c:V8.".
    debug_assert!(
      name.starts_with("c:V8."),
      "counter name must start with c:V8.: {name}"
    );
    Self {
      counters,
      name,
      ptr: Mutex::new(None),
    }
  }

  /// 对应 `name()`。
  #[inline]
  pub fn name(&self) -> &str {
    &self.name
  }

  /// 对应 `SetupPtrFromStatsTable()` 并返回缓存槽位(`GetPtr()`)。
  fn get_ptr(&self) -> CounterPtr {
    {
      let guard = self.ptr.lock();
      if let Some(ptr) = guard.as_ref() {
        return ptr.clone();
      }
    }
    let location = self.counters.find_location(&self.name);
    let ptr = match location {
      Some(arc) => CounterPtr::External(arc),
      None => CounterPtr::Unused,
    };
    *self.ptr.lock() = Some(ptr.clone());
    ptr
  }

  /// 对应 `Enabled()`:lookup 提供过非空槽位。
  pub fn enabled(&self) -> bool {
    matches!(self.get_ptr(), CounterPtr::External(_))
  }

  /// 对应 `Set(int value)`。
  pub fn set(&self, value: i32) {
    match self.get_ptr() {
      CounterPtr::External(arc) => arc.store(value, Ordering::Relaxed),
      CounterPtr::Unused => UNUSED_COUNTER_DUMP.store(value, Ordering::Relaxed),
    }
  }

  /// 对应 `Get()`(C++ `load()` 无序参, 默认 seq_cst)。
  pub fn get(&self) -> i32 {
    match self.get_ptr() {
      CounterPtr::External(arc) => arc.load(Ordering::SeqCst),
      CounterPtr::Unused => UNUSED_COUNTER_DUMP.load(Ordering::SeqCst),
    }
  }

  /// 对应 `Increment(int value = 1)`。
  pub fn increment(&self, value: i32) {
    match self.get_ptr() {
      CounterPtr::External(arc) => {
        arc.fetch_add(value, Ordering::Relaxed);
      }
      CounterPtr::Unused => {
        UNUSED_COUNTER_DUMP.fetch_add(value, Ordering::Relaxed);
      }
    }
  }

  /// 对应 `Decrement(int value = 1)`。
  pub fn decrement(&self, value: i32) {
    self.increment(-value);
  }

  /// 对应 `GetInternalPointer()`:代码生成路径推迟,先暴露 Arc 句柄;
  /// 未启用(无 lookup)返回 `None`(C++ 为 `&unused_counter_dump`)。
  pub fn get_internal_pointer(&self) -> Option<Arc<AtomicI32>> {
    match self.get_ptr() {
      CounterPtr::External(arc) => Some(arc),
      CounterPtr::Unused => None,
    }
  }

  /// 对应 `Reset()`(重置缓存内部指针)。
  fn reset(&self) {
    *self.ptr.lock() = None;
  }
}

/// 对应 `class Histogram`:StatsTable 中动态创建的直方图。
pub struct Histogram {
  counters: Arc<StatsTable>,
  name: String,
  min: i32,
  max: i32,
  num_buckets: i32,
  /// 对应 `std::atomic<void*> histogram_`(0 = nullptr)。
  handle: AtomicU64,
  mutex: Mutex<()>,
}

impl Histogram {
  fn new(counters: Arc<StatsTable>, name: &str, min: i32, max: i32, num_buckets: i32) -> Self {
    Self {
      counters,
      name: name.to_owned(),
      min,
      max,
      num_buckets,
      handle: AtomicU64::new(0),
      mutex: Mutex::new(()),
    }
  }

  /// 对应 `AddSample(int sample)`。
  pub fn add_sample(&self, sample: i32) {
    if self.enabled() {
      self
        .counters
        .add_histogram_sample(self.handle.load(Ordering::Relaxed), sample);
    }
  }

  /// 对应 `Enabled()`。
  #[inline]
  pub fn enabled(&self) -> bool {
    self.handle.load(Ordering::Acquire) != 0
  }

  #[inline]
  pub fn name(&self) -> &str {
    &self.name
  }
  #[inline]
  pub fn min(&self) -> i32 {
    self.min
  }
  #[inline]
  pub fn max(&self) -> i32 {
    self.max
  }
  #[inline]
  pub fn num_buckets(&self) -> i32 {
    self.num_buckets
  }

  /// 对应 `Reset()`:惰性重建。
  fn reset(&self) {
    self.handle.store(0, Ordering::Relaxed);
  }

  /// 对应 `CreateHistogram() const`。
  fn create_histogram(&self) -> HistogramHandle {
    self
      .counters
      .create_histogram(&self.name, self.min, self.max, self.num_buckets as usize)
  }

  /// 对应 `EnsureCreated(bool create_new = true)`:双检锁。
  pub fn ensure_created(&self, create_new: bool) {
    if create_new && self.handle.load(Ordering::Acquire) == 0 {
      let _guard = self.mutex.lock();
      if self.handle.load(Ordering::Relaxed) == 0 {
        self
          .handle
          .store(self.create_histogram(), Ordering::Release);
      }
    }
  }
}

/// 对应 `class PercentageHistogram : public Histogram`(空子类,仅访问性区分)。
pub struct PercentageHistogram {
  inner: Histogram,
}

impl PercentageHistogram {
  fn new(counters: Arc<StatsTable>, caption: &str) -> Self {
    // 对应 CountersInitializer::Visit(PercentageHistogram*): (0, 101, 100)。
    Self {
      inner: Histogram::new(counters, caption, 0, 101, 100),
    }
  }
  #[inline]
  pub fn histogram(&self) -> &Histogram {
    &self.inner
  }
  #[inline]
  pub fn name(&self) -> &str {
    self.inner.name()
  }
  /// 对应 `EnsureCreated()` + 父类直通。
  pub fn ensure_created(&self) {
    self.inner.ensure_created(true);
  }
}

/// 对应 `class LegacyMemoryHistogram : public Histogram`。
pub struct LegacyMemoryHistogram {
  inner: Histogram,
}

impl LegacyMemoryHistogram {
  fn new(counters: Arc<StatsTable>, caption: &str) -> Self {
    // 对应 CountersInitializer::Visit(LegacyMemoryHistogram*): (1000, 500000, 50)。
    Self {
      inner: Histogram::new(counters, caption, 1000, 500_000, 50),
    }
  }
  #[inline]
  pub fn histogram(&self) -> &Histogram {
    &self.inner
  }
  #[inline]
  pub fn name(&self) -> &str {
    self.inner.name()
  }
  pub fn ensure_created(&self) {
    self.inner.ensure_created(true);
  }
}

/// 对应 `class TimedHistogram : public Histogram`(线程安全计时直方图)。
pub struct TimedHistogram {
  inner: Histogram,
  resolution: TimedHistogramResolution,
}

impl TimedHistogram {
  fn new(
    counters: Arc<StatsTable>,
    caption: &str,
    min: i32,
    max: i32,
    resolution: TimedHistogramResolution,
    num_buckets: i32,
  ) -> Self {
    Self {
      inner: Histogram::new(counters, caption, min, max, num_buckets),
      resolution,
    }
  }

  /// 对应 `AddTimedSample(base::TimeDelta sample)`。
  pub fn add_timed_sample(&self, sample: TimeDelta) {
    if self.inner.enabled() {
      let sample_int = match self.resolution {
        TimedHistogramResolution::Microsecond => sample.in_microseconds(),
        TimedHistogramResolution::Millisecond => sample.in_milliseconds(),
      };
      // C++ static_cast<int>:低 32 位截断。
      self.inner.add_sample(sample_int as i32);
    }
  }

  /// 对应 `RecordAbandon(base::ElapsedTimer* timer, Isolate* isolate)`:
  /// isolate 非空的 `V8FileLogger::CallEventLogger` 分支推迟(见 readme)。
  pub fn record_abandon(&self, timer: &mut ElapsedTimer) {
    if self.inner.enabled() {
      debug_assert!(timer.is_started());
      timer.stop();
      let sample = match self.resolution {
        TimedHistogramResolution::Microsecond => TimeDelta::max().in_microseconds(),
        TimedHistogramResolution::Millisecond => TimeDelta::max().in_milliseconds(),
      };
      self.inner.add_sample(sample as i32);
    }
  }

  #[inline]
  pub fn histogram(&self) -> &Histogram {
    &self.inner
  }
  #[inline]
  pub fn name(&self) -> &str {
    self.inner.name()
  }
  #[inline]
  pub fn resolution(&self) -> TimedHistogramResolution {
    self.resolution
  }
  #[inline]
  pub fn enabled(&self) -> bool {
    self.inner.enabled()
  }
  pub fn ensure_created(&self, create_new: bool) {
    self.inner.ensure_created(create_new);
  }
  fn reset(&self) {
    self.inner.reset();
  }
  // 注: 原版 protected `Stop(base::ElapsedTimer*)`(counters.h:253, counters.cc:59-63)
  // 在现源码树无任何调用方(V8 自身死代码, `BaseTimedHistogramScope::StopInternal`
  // 已内联该逻辑), 本移植不予转写; 详见 readme/zh.md 删除登记。
}

/// 对应 `class NestedTimedHistogram : public TimedHistogram`。
///
/// C++ 用裸指针链表串起当前作用域栈;Rust 等价改为 `Mutex<Vec<ElapsedTimer>>`
/// (登记差异;语义:进入暂停上层、离开停止本层并把时间计入直方图、恢复上层)。
pub struct NestedTimedHistogram {
  inner: TimedHistogram,
  scopes: Mutex<Vec<ElapsedTimer>>,
}

impl NestedTimedHistogram {
  fn new(
    counters: Arc<StatsTable>,
    caption: &str,
    max: i32,
    resolution: TimedHistogramResolution,
    num_buckets: i32,
  ) -> Self {
    Self {
      inner: TimedHistogram::new(counters, caption, 0, max, resolution, num_buckets),
      scopes: Mutex::new(Vec::new()),
    }
  }

  /// 对应 `Enter(next)` + `NestedTimedHistogramScope::StartInteral`。
  fn enter(&self) {
    let now = TimeTicks::now();
    let mut stack = self.scopes.lock();
    if let Some(top) = stack.last_mut() {
      top.pause(now);
    }
    let mut timer = ElapsedTimer::new();
    timer.start_at(now);
    stack.push(timer);
  }

  /// 对应 `Leave(previous)` + `NestedTimedHistogramScope::StopInternal`。
  fn leave(&self) {
    let now = TimeTicks::now();
    let mut stack = self.scopes.lock();
    if let Some(mut timer) = stack.pop() {
      timer.pause(now);
      let elapsed = timer.elapsed_at(now);
      self.inner.add_timed_sample(elapsed);
      // RecordLongTaskTime 分支依赖 Isolate,推迟(见 readme)。
      if let Some(top) = stack.last_mut() {
        top.resume(now);
      }
    }
  }

  /// 对应 `PauseNestedTimedHistogramScope`:临时弹出(离开 V8 的外部回调)。
  fn pause_top(&self) -> Option<ElapsedTimer> {
    let now = TimeTicks::now();
    let mut stack = self.scopes.lock();
    let mut timer = stack.pop()?;
    timer.pause(now);
    Some(timer)
  }

  fn resume_top(&self, timer: Option<ElapsedTimer>) {
    let Some(mut timer) = timer else { return };
    let now = TimeTicks::now();
    timer.resume(now);
    self.scopes.lock().push(timer);
  }

  #[inline]
  pub fn histogram(&self) -> &Histogram {
    self.inner.histogram()
  }
  #[inline]
  pub fn name(&self) -> &str {
    self.inner.name()
  }
  #[inline]
  pub fn enabled(&self) -> bool {
    self.inner.enabled()
  }
  pub fn ensure_created(&self, create_new: bool) {
    self.inner.ensure_created(create_new);
  }
  fn reset(&self) {
    self.inner.reset();
  }
}

/// 对应 `class AggregatableHistogramTimer : public Histogram`(外层聚合)。
pub struct AggregatableHistogramTimer {
  inner: Histogram,
  /// 对应 `base::TimeDelta time_`。
  time: Cell<TimeDelta>,
}

impl AggregatableHistogramTimer {
  fn new(counters: Arc<StatsTable>, caption: &str) -> Self {
    // 对应 CountersInitializer::Visit(AggregatableHistogramTimer*):
    // Initialize(caption, 0, 10000000, DefaultTimedHistogramNumBuckets)。
    Self {
      inner: Histogram::new(
        counters,
        caption,
        0,
        10_000_000,
        DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS,
      ),
      time: Cell::new(TimeDelta::default()),
    }
  }

  /// 对应 `Start()`。
  pub fn start(&self) {
    self.time.set(TimeDelta::default());
  }

  /// 对应 `Stop()`(仅非零样本计入)。
  pub fn stop(&self) {
    let time = self.time.get();
    if !time.is_zero() {
      // Only add non-zero samples, since zero samples represent situations
      // where there were no aggregated samples added.
      self.inner.add_sample(time.in_microseconds() as i32);
    }
  }

  /// 对应 `Add(base::TimeDelta other)`。
  pub fn add(&self, other: TimeDelta) {
    self.time.set(self.time.get() + other);
  }

  #[inline]
  pub fn histogram(&self) -> &Histogram {
    &self.inner
  }
  #[inline]
  pub fn name(&self) -> &str {
    self.inner.name()
  }
  pub fn ensure_created(&self) {
    self.inner.ensure_created(true);
  }
  fn reset(&self) {
    self.inner.reset();
  }
}

/// 对应 `AggregatingHistogramTimerScope`(外层作用域)。
pub struct AggregatingHistogramTimerScope<'a> {
  histogram: &'a AggregatableHistogramTimer,
}

impl<'a> AggregatingHistogramTimerScope<'a> {
  pub fn new(histogram: &'a AggregatableHistogramTimer) -> Self {
    histogram.start();
    Self { histogram }
  }
}

impl Drop for AggregatingHistogramTimerScope<'_> {
  fn drop(&mut self) {
    self.histogram.stop();
  }
}

/// 对应 `AggregatedHistogramTimerScope`(内层作用域)。
pub struct AggregatedHistogramTimerScope<'a> {
  histogram: &'a AggregatableHistogramTimer,
  timer: ElapsedTimer,
}

impl<'a> AggregatedHistogramTimerScope<'a> {
  pub fn new(histogram: &'a AggregatableHistogramTimer) -> Self {
    let mut timer = ElapsedTimer::new();
    timer.start();
    Self { histogram, timer }
  }
}

impl Drop for AggregatedHistogramTimerScope<'_> {
  fn drop(&mut self) {
    self.histogram.add(self.timer.elapsed());
  }
}

// ============================================================================
// AggregatedMemoryHistogram(1:1 线性插值算法,counters.h)
// ============================================================================

/// 后备直方图的采样出口(C++ 中 `MockHistogram::AddSample(int)` 覆写
/// `Histogram::AddSample`;Rust 以 trait 注入,便于单测)。
pub trait SampleSink {
  fn add_sample(&self, sample: i32);
}

impl SampleSink for Histogram {
  fn add_sample(&self, sample: i32) {
    Histogram::add_sample(self, sample);
  }
}

/// 对应 `template <typename Histogram> class AggregatedMemoryHistogram`:
/// 收集 (time, value) 采样对,按 `v8_flags.histogram_interval` 生成
/// 均匀时间轴样本给后备直方图。
pub struct AggregatedMemoryHistogram<'a, S: SampleSink> {
  is_initialized: bool,
  start_ms: f64,
  last_ms: f64,
  aggregate_value: f64,
  last_value: f64,
  backing_histogram: &'a S,
}

impl<'a, S: SampleSink> AggregatedMemoryHistogram<'a, S> {
  /// 对应 `explicit AggregatedMemoryHistogram(Histogram* backing_histogram)`。
  pub fn new(backing_histogram: &'a S) -> Self {
    Self {
      is_initialized: false,
      start_ms: 0.0,
      last_ms: 0.0,
      aggregate_value: 0.0,
      last_value: 0.0,
      backing_histogram,
    }
  }

  /// 对应 `Aggregate(current_ms, current_value)`:加权平均。
  fn aggregate(&self, current_ms: f64, current_value: f64) -> f64 {
    let interval_ms = current_ms - self.start_ms;
    let value = (current_value + self.last_value) / 2.0;
    // The aggregate_value_ is the average for [start_ms_; last_ms_].
    // The value is the average for [last_ms_; current_ms].
    // Return the weighted average of the aggregate_value_ and the value.
    self.aggregate_value * ((self.last_ms - self.start_ms) / interval_ms)
      + value * ((current_ms - self.last_ms) / interval_ms)
  }

  /// 对应 `AddSample(double current_ms, double current_value)`。
  pub fn add_sample(&mut self, current_ms: f64, current_value: f64) {
    if !self.is_initialized {
      self.aggregate_value = current_value;
      self.start_ms = current_ms;
      self.last_value = current_value;
      self.last_ms = current_ms;
      self.is_initialized = true;
    } else {
      const K_EPSILON: f64 = 1e-6;
      const K_MAX_SAMPLES: i32 = 1000;
      if current_ms < self.last_ms + K_EPSILON {
        // Two samples have the same time, remember the last one.
        self.last_value = current_value;
      } else {
        // 1:1 对齐 counters.h:451-452: 原版不检查 interval 合法性,
        // interval <= 0 时循环由 kMaxSamples(1000) 兜底收敛, 不钳制。
        let sample_interval_ms = flags_lite::histogram_interval();
        let mut end_ms = self.start_ms + sample_interval_ms;
        if end_ms <= current_ms + K_EPSILON {
          // Linearly interpolate between the last_ms_ and the current_ms.
          let slope = (current_value - self.last_value) / (current_ms - self.last_ms);
          let mut i = 0;
          // Send aggregated samples to the backing histogram from the start_ms
          // to the current_ms.
          while i < K_MAX_SAMPLES && end_ms <= current_ms + K_EPSILON {
            let end_value = self.last_value + (end_ms - self.last_ms) * slope;

            let sample_value = if i == 0 {
              // Take aggregate_value_ into account.
              self.aggregate(end_ms, end_value)
            } else {
              // There is no aggregate_value_ for i > 0.
              (self.last_value + end_value) / 2.0
            };
            self
              .backing_histogram
              .add_sample((sample_value + 0.5) as i64 as i32);
            self.last_value = end_value;
            self.last_ms = end_ms;
            end_ms += sample_interval_ms;
            i += 1;
          }
          if i == K_MAX_SAMPLES {
            // We hit the sample limit, ignore the remaining samples.
            self.aggregate_value = current_value;
            self.start_ms = current_ms;
          } else {
            self.aggregate_value = self.last_value;
            self.start_ms = self.last_ms;
          }
        }
        self.aggregate_value = if current_ms > self.start_ms + K_EPSILON {
          self.aggregate(current_ms, current_value)
        } else {
          self.aggregate_value
        };
        self.last_value = current_value;
        self.last_ms = current_ms;
      }
    }
  }
}

// ============================================================================
// counters-scopes.h 移植
// ============================================================================

/// 对应 `enum class OptionalTimedHistogramScopeMode`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionalTimedHistogramScopeMode {
  TakeTime,
  DontTakeTime,
}

/// 对应 `class TimedHistogramScope : public BaseTimedHistogramScope`。
///
/// `LogStart/LogEnd`(V8FileLogger isolate 挂钩)与 `ToggleRunningState`
/// 的 DEBUG 嵌套检查推迟(见 readme);`result_in_microseconds` 出参
/// 改为 `stop()` 返回值。
pub struct TimedHistogramScope<'a> {
  histogram: &'a TimedHistogram,
  timer: Option<ElapsedTimer>,
}

impl<'a> TimedHistogramScope<'a> {
  pub fn new(histogram: &'a TimedHistogram) -> Self {
    let mut timer = None;
    if histogram.enabled() {
      // StartInternal: DCHECK(ToggleRunningState(true)) 推迟。
      let mut t = ElapsedTimer::new();
      t.start();
      timer = Some(t);
    }
    Self { histogram, timer }
  }

  /// 对应 `Stop()`:未启用时返回 -1 微秒(kTimeNotMeasured 语义)。
  pub fn stop(&mut self) -> TimeDelta {
    match self.timer.take() {
      Some(mut t) => {
        // StopInternal: elapsed → AddTimedSample → timer.Stop()。
        let elapsed = t.elapsed();
        self.histogram.add_timed_sample(elapsed);
        t.stop();
        elapsed
      }
      None => TimeDelta::from_microseconds(-1),
    }
  }
}

impl Drop for TimedHistogramScope<'_> {
  fn drop(&mut self) {
    // ~TimedHistogramScope: Stop() 后 isolate/结果出参分支已并入返回值/推迟。
    let _ = self.stop();
  }
}

/// 对应 `class OptionalTimedHistogramScope`(`DONT_TAKE_TIME` 不计时)。
pub struct OptionalTimedHistogramScope<'a> {
  inner: TimedHistogramScope<'a>,
}

impl<'a> OptionalTimedHistogramScope<'a> {
  pub fn new(histogram: &'a TimedHistogram, mode: OptionalTimedHistogramScopeMode) -> Self {
    let inner = if mode == OptionalTimedHistogramScopeMode::TakeTime {
      TimedHistogramScope::new(histogram)
    } else {
      // 模式不计时:持句柄但不启动 timer(等价 C++ 提前 return)。
      Self::idle_scope(histogram)
    };
    Self { inner }
  }

  fn idle_scope(histogram: &'a TimedHistogram) -> TimedHistogramScope<'a> {
    TimedHistogramScope {
      histogram,
      timer: None,
    }
  }
}

impl Drop for OptionalTimedHistogramScope<'_> {
  fn drop(&mut self) {
    // 对应 C++ 析构 `Stop()`(counters-scopes.h:103-107; TAKE_TIME 才计时,
    // DONT_TAKE_TIME 下 timer 未启动, `stop()` 返回 -1us 即 no-op)。
    let _ = self.inner.stop();
  }
}

/// 对应 `class LazyTimedHistogramScope`:直方图在停止时刻才选定。
pub struct LazyTimedHistogramScope<'a> {
  timer: ElapsedTimer,
  histogram: Option<&'a TimedHistogram>,
}

impl<'a> LazyTimedHistogramScope<'a> {
  pub fn new() -> Self {
    let mut timer = ElapsedTimer::new();
    timer.start();
    Self {
      timer,
      histogram: None,
    }
  }

  /// 对应 `set_histogram(TimedHistogram*)`。
  pub fn set_histogram(&mut self, histogram: &'a TimedHistogram) {
    // DCHECK_IMPLIES(histogram->Enabled(), ToggleRunningState(true)) 推迟。
    self.histogram = Some(histogram);
  }
}

impl Drop for LazyTimedHistogramScope<'_> {
  fn drop(&mut self) {
    // Stop(): histogram_ && Enabled() 才计入;否则 -1us(此处直接丢弃)。
    if let Some(h) = self.histogram.take()
      && h.enabled()
    {
      let elapsed = self.timer.elapsed();
      h.add_timed_sample(elapsed);
      self.timer.stop();
    }
  }
}

impl Default for LazyTimedHistogramScope<'_> {
  fn default() -> Self {
    Self::new()
  }
}

/// 对应 `class NestedTimedHistogramScope`。
pub struct NestedTimedHistogramScope<'a> {
  histogram: &'a NestedTimedHistogram,
  active: bool,
}

impl<'a> NestedTimedHistogramScope<'a> {
  pub fn new(histogram: &'a NestedTimedHistogram) -> Self {
    let active = histogram.enabled();
    if active {
      // StartInteral(原文拼写): Enter + 暂停上层 + Start(now)。
      histogram.enter();
    }
    // LogStart(counters()->isolate()) 推迟。
    Self { histogram, active }
  }
}

impl Drop for NestedTimedHistogramScope<'_> {
  fn drop(&mut self) {
    if self.active {
      // StopInternal: Leave(内部完成计时+恢复上层)。
      self.histogram.leave();
    }
    // LogEnd 推迟。
  }
}

/// 对应 `class PauseNestedTimedHistogramScope`(离开 V8 的外部回调时暂停)。
pub struct PauseNestedTimedHistogramScope<'a> {
  histogram: &'a NestedTimedHistogram,
  saved: Option<ElapsedTimer>,
}

impl<'a> PauseNestedTimedHistogramScope<'a> {
  pub fn new(histogram: &'a NestedTimedHistogram) -> Self {
    // C++: Enter(nullptr) 压入空哨兵;isEnabled() = previous && Enabled。
    let saved = if histogram.enabled() {
      histogram.pause_top()
    } else {
      None
    };
    Self { histogram, saved }
  }
}

impl Drop for PauseNestedTimedHistogramScope<'_> {
  fn drop(&mut self) {
    // ~: Leave(previous) 对哨兵是 no-op;Resume 恢复栈顶。
    self.histogram.resume_top(self.saved.take());
  }
}

// ============================================================================
// Counters(counters.h 尾段;X-macro 清单机械展开)
// ============================================================================

/// `counters-definitions.h` 五张 X-macro 清单的机械展开登记。
/// 分组即原型的五个 Visit 重载(禁止理解宏,仅照抄清单)。
macro_rules! define_counters {
  (
    $(histogram $hname:ident [$hcap:expr, $hmin:expr, $hmax:expr, $hbuckets:expr];)*
    $(slow_histogram $shname:ident [$shcap:expr, $shmin:expr, $shmax:expr, $shbuckets:expr];)*
    $(nested_timed $nname:ident [$ncap:expr, $nmax:expr, $nres:expr];)*
    $(slow_nested_timed $snname:ident [$sncap:expr, $snmax:expr, $snres:expr];)*
    $(timed $tname:ident [$tcap:expr, $tmax:expr, $tres:expr];)*
    $(aggregatable_timer $aname:ident [$acap:expr];)*
    $(percentage $pname:ident [$pcap:expr];)*
    $(legacy_memory $mname:ident [$mcap:expr];)*
    $(stats_counter $cname:ident [$ccap:expr];)*
  ) => {
    /// 对应 `class Counters`:V8 全部计数器的聚合体。
    pub struct Counters {
      $($hname: Histogram,)*
      $($shname: Histogram,)*
      $($nname: NestedTimedHistogram,)*
      $($snname: NestedTimedHistogram,)*
      $($tname: TimedHistogram,)*
      $($aname: AggregatableHistogramTimer,)*
      $($pname: PercentageHistogram,)*
      $($mname: LegacyMemoryHistogram,)*
      $($cname: StatsCounter,)*
      /// 对应 `RuntimeCallStats runtime_call_stats_`(V8_RUNTIME_CALL_STATS 默认开)。
      runtime_call_stats: RuntimeCallStats,
      /// 对应 `WorkerThreadRuntimeCallStats worker_thread_runtime_call_stats_`。
      worker_thread_runtime_call_stats:
        WorkerThreadRuntimeCallStats,
      stats_table: Arc<StatsTable>,
    }

    impl Counters {
      /// 对应 `Counters(Isolate*)` + `CountersInitializer init(this); init.Start();`
      /// (isolate 依赖推迟;C++ Start() 先 stats-counters 后 histograms,但各
      /// Initialize 互不依赖, Rust 按字段声明序初始化, 顺序无语义影响)。
      pub fn new(stats_table: Arc<StatsTable>) -> Self {
        let clone = || stats_table.clone();
        Self {
          $($hname: Histogram::new(clone(), $hcap, $hmin, $hmax, $hbuckets),)*
          $($shname: Histogram::new(clone(), $shcap, $shmin, $shmax, $shbuckets),)*
          $($nname: NestedTimedHistogram::new(clone(), $ncap, $nmax, $nres,
              DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS),)*
          $($snname: NestedTimedHistogram::new(clone(), $sncap, $snmax, $snres,
              DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS),)*
          $($tname: TimedHistogram::new(clone(), $tcap, 0, $tmax, $tres,
              DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS),)*
          $($aname: AggregatableHistogramTimer::new(clone(), $acap),)*
          $($pname: PercentageHistogram::new(clone(), $pcap),)*
          $($mname: LegacyMemoryHistogram::new(clone(), $mcap),)*
          $($cname: StatsCounter::new(clone(), $ccap.to_owned()),)*
          runtime_call_stats: RuntimeCallStats::new(
            ThreadType::MainIsolateThread,
          ),
          worker_thread_runtime_call_stats:
            WorkerThreadRuntimeCallStats::new(),
          stats_table,
        }
      }

      /// 对应 `StatsTable* Counters::stats_table()`(C++ 为私有 friend 通道)。
      pub fn stats_table(&self) -> &StatsTable {
        &self.stats_table
      }

      /// 对应 `RuntimeCallStats* runtime_call_stats()`。
      pub fn runtime_call_stats(&self) -> &RuntimeCallStats {
        &self.runtime_call_stats
      }

      /// 对应 `WorkerThreadRuntimeCallStats* worker_thread_runtime_call_stats()`。
      pub fn worker_thread_runtime_call_stats(&self) -> &WorkerThreadRuntimeCallStats {
        &self.worker_thread_runtime_call_stats
      }

      /// 对应 `void ResetCounterFunction(CounterLookupCallback f)`
      /// (+ `StatsCounterResetter` 遍历)。
      pub fn reset_counter_function(&self, f: CounterLookupCallback) {
        self.stats_table.set_counter_function(f);
        $(self.$cname.reset();)*
      }

      /// 对应 `void ResetCreateHistogramFunction(CreateHistogramCallback f)`
      /// (+ `HistogramResetter` 遍历; C++ 经继承/重载分派到各类 `Reset()`,
      /// 此处调各类型自身的 `reset` 包装)。
      pub fn reset_create_histogram_function(&self, f: CreateHistogramCallback) {
        self.stats_table.set_create_histogram_function(f);
        $(self.$hname.reset();)*
        $(self.$shname.reset();)*
        $(self.$nname.reset();)*
        $(self.$snname.reset();)*
        $(self.$tname.reset();)*
        $(self.$aname.reset();)*
        $(self.$pname.histogram().reset();)*
        $(self.$mname.histogram().reset();)*
      }

      /// 对应 `void SetAddHistogramSampleFunction(AddHistogramSampleCallback f)`。
      pub fn set_add_histogram_sample_function(&self, f: AddHistogramSampleCallback) {
        self.stats_table.set_add_histogram_sample_function(f);
      }

      // --- 访问器:X-macro 展开为同名方法(EnsureCreated 语义照抄) ---
      $(
        #[doc = concat!("对应 `Histogram* ", stringify!($hname), "()`。")]
        pub fn $hname(&self) -> &Histogram {
          self.$hname.ensure_created(true);
          &self.$hname
        }
      )*
      $(
        #[doc = concat!("对应 SLOW `Histogram* ", stringify!($shname),
          "()`(EnsureCreated(v8_flags.slow_histograms))。")]
        pub fn $shname(&self) -> &Histogram {
          self.$shname.ensure_created(flags_lite::slow_histograms());
          &self.$shname
        }
      )*
      $(
        #[doc = concat!("对应 `NestedTimedHistogram* ", stringify!($nname), "()`。")]
        pub fn $nname(&self) -> &NestedTimedHistogram {
          self.$nname.ensure_created(true);
          &self.$nname
        }
      )*
      $(
        #[doc = concat!("对应 SLOW `NestedTimedHistogram* ", stringify!($snname),
          "()`(EnsureCreated(v8_flags.slow_histograms))。")]
        pub fn $snname(&self) -> &NestedTimedHistogram {
          self.$snname.ensure_created(flags_lite::slow_histograms());
          &self.$snname
        }
      )*
      $(
        #[doc = concat!("对应 `TimedHistogram* ", stringify!($tname), "()`。")]
        pub fn $tname(&self) -> &TimedHistogram {
          self.$tname.ensure_created(true);
          &self.$tname
        }
      )*
      $(
        #[doc = concat!("对应 `AggregatableHistogramTimer* ", stringify!($aname), "()`。")]
        pub fn $aname(&self) -> &AggregatableHistogramTimer {
          self.$aname.ensure_created();
          &self.$aname
        }
      )*
      $(
        #[doc = concat!("对应 `PercentageHistogram* ", stringify!($pname), "()`。")]
        pub fn $pname(&self) -> &PercentageHistogram {
          self.$pname.ensure_created();
          &self.$pname
        }
      )*
      $(
        #[doc = concat!("对应 `LegacyMemoryHistogram* ", stringify!($mname), "()`。")]
        pub fn $mname(&self) -> &LegacyMemoryHistogram {
          self.$mname.ensure_created();
          &self.$mname
        }
      )*
      $(
        #[doc = concat!("对应 `StatsCounter* ", stringify!($cname), "()`。")]
        pub fn $cname(&self) -> &StatsCounter {
          &self.$cname
        }
      )*
    }

    // --- 机械展开自省表(测试用):顺序即 counters-definitions.h 清单顺序,
    //     由宏自身生成,禁手写计数(10.3 确定性红线)。---
    impl Counters {
      /// `HISTOGRAM_RANGE_LIST` 各 caption(注册顺序)。
      pub const HISTOGRAM_CAPTIONS: &'static [&'static str] = &[$($hcap,)*];
      /// `HISTOGRAM_RANGE_LIST` 中 SLOW 项 caption。
      pub const SLOW_HISTOGRAM_CAPTIONS: &'static [&'static str] = &[$($shcap,)*];
      /// `TIMED_NESTED_HISTOGRAM_MS_US_LIST` 各 caption。
      pub const NESTED_TIMED_CAPTIONS: &'static [&'static str] = &[$($ncap,)*];
      /// 同上清单中 SLOW 项 caption。
      pub const SLOW_NESTED_TIMED_CAPTIONS: &'static [&'static str] = &[$($sncap,)*];
      /// `TIMED_HISTOGRAM_MS_US_LIST` 各 caption。
      pub const TIMED_CAPTIONS: &'static [&'static str] = &[$($tcap,)*];
      /// `AGGREGATABLE_HISTOGRAM_TIMER_LIST` 各 caption。
      pub const AGGREGATABLE_TIMER_CAPTIONS: &'static [&'static str] = &[$($acap,)*];
      /// `HISTOGRAM_PERCENTAGE_LIST` 各 caption。
      pub const PERCENTAGE_CAPTIONS: &'static [&'static str] = &[$($pcap,)*];
      /// `HISTOGRAM_LEGACY_MEMORY_LIST` 各 caption。
      pub const LEGACY_MEMORY_CAPTIONS: &'static [&'static str] = &[$($mcap,)*];
      /// `STATS_COUNTER_*_LIST` 各 caption(含 `c:` 前缀)。
      pub const STATS_COUNTER_CAPTIONS: &'static [&'static str] = &[$($ccap,)*];

      /// 全部清单条目总数(对应 counters-definitions.h 全展开计数)。
      pub const TOTAL_COUNT: usize = Self::HISTOGRAM_CAPTIONS.len()
        + Self::SLOW_HISTOGRAM_CAPTIONS.len()
        + Self::NESTED_TIMED_CAPTIONS.len()
        + Self::SLOW_NESTED_TIMED_CAPTIONS.len()
        + Self::TIMED_CAPTIONS.len()
        + Self::AGGREGATABLE_TIMER_CAPTIONS.len()
        + Self::PERCENTAGE_CAPTIONS.len()
        + Self::LEGACY_MEMORY_CAPTIONS.len()
        + Self::STATS_COUNTER_CAPTIONS.len();
    }
  };
}

// 以下为 counters-definitions.h 五清单的逐项机械转写(caption 字符串含
// 原始 "V8." 前缀;SC 加 "c:" 前缀,同 CountersVisitor::VisitStatsCounters)。
// HISTOGRAM_RANGE_LIST(49 项):
define_counters! {
  histogram code_cache_reject_reason ["V8.CodeCacheRejectReason", 1, 9, 9];
  histogram incremental_marking_reason ["V8.GCIncrementalMarkingReason", 0,
    K_GARBAGE_COLLECTION_REASON_MAX_VALUE, K_GARBAGE_COLLECTION_REASON_MAX_VALUE + 1];
  histogram incremental_marking_sum ["V8.GCIncrementalMarkingSum", 0, 10000, 101];
  histogram mark_compact_reason ["V8.GCMarkCompactReason", 0,
    K_GARBAGE_COLLECTION_REASON_MAX_VALUE, K_GARBAGE_COLLECTION_REASON_MAX_VALUE + 1];
  histogram gc_finalize_clear ["V8.GCFinalizeMC.Clear", 0, 10000, 101];
  histogram gc_finalize_epilogue ["V8.GCFinalizeMC.Epilogue", 0, 10000, 101];
  histogram gc_finalize_evacuate ["V8.GCFinalizeMC.Evacuate", 0, 10000, 101];
  histogram gc_finalize_finish ["V8.GCFinalizeMC.Finish", 0, 10000, 101];
  histogram gc_finalize_mark ["V8.GCFinalizeMC.Mark", 0, 10000, 101];
  histogram gc_finalize_prologue ["V8.GCFinalizeMC.Prologue", 0, 10000, 101];
  histogram gc_finalize_sweep ["V8.GCFinalizeMC.Sweep", 0, 10000, 101];
  histogram gc_scavenger_scavenge_main ["V8.GCScavenger.ScavengeMain", 0, 10000, 101];
  histogram gc_scavenger_scavenge_roots ["V8.GCScavenger.ScavengeRoots", 0, 10000, 101];
  // Asm/Wasm.
  histogram wasm_functions_per_asm_module ["V8.WasmFunctionsPerModule.asm", 1, 1000000, 51];
  histogram wasm_functions_per_wasm_module ["V8.WasmFunctionsPerModule.wasm", 1, 1000000, 51];
  histogram array_buffer_big_allocations ["V8.ArrayBufferLargeAllocations", 0, 4096, 13];
  histogram array_buffer_new_size_failures ["V8.ArrayBufferNewSizeFailures", 0, 4096, 13];
  histogram wasm_wasm_huge_function_size_bytes ["V8.WasmHugeFunctionSizeBytes.wasm",
    100 * KB, GB, 51];
  histogram wasm_wasm_module_size_bytes ["V8.WasmModuleSizeBytes.wasm", 1, GB, 51];
  histogram wasm_compile_huge_function_peak_memory_bytes
    ["V8.WasmCompileHugeFunctionPeakMemoryBytes", 1, GB, 51];
  histogram asm_module_size_bytes ["V8.AsmModuleSizeBytes", 1, GB, 51];
  histogram compile_script_cache_behaviour ["V8.CompileScript.CacheBehaviour", 0, 22, 23];
  histogram wasm_memory_allocation_result ["V8.WasmMemoryAllocationResult", 0, 3, 4];
  // Committed code size per module, collected on GC. Older histogram, in MiB.
  histogram wasm_module_code_size_mb ["V8.WasmModuleCodeSizeMiB", 0, 1024, 64];
  // Newer histogram, in KiB (0..100MB).
  histogram wasm_module_code_size_kb ["V8.WasmModuleCodeSizeKiB", 0, 1024 * 100, 101];
  histogram wasm_module_metadata_size_kb ["V8.WasmModuleMetadataSizeKiB", 0, 1024 * 100, 101];
  histogram wasm_engine_metadata_size_kb ["V8.WasmEngineMetadataSizeKiB", 0, 1024 * 100, 101];
  histogram wasm_module_freed_code_size_percent ["V8.WasmModuleCodeSizePercentFreed", 0, 100, 32];
  histogram wasm_module_num_triggered_code_gcs ["V8.WasmModuleNumberOfCodeGCsTriggered", 1, 128, 20];
  histogram wasm_module_num_code_spaces ["V8.WasmModuleNumberOfCodeSpaces", 1, 128, 20];
  histogram wasm_deopts_executed ["V8.WasmDeoptsExecutedCount", 0, 10000, 51];
  histogram wasm_deopts_per_function ["V8.WasmDeoptsPerFunction", 0, 500, 21];
  histogram wasm_revec_conversion_percent ["V8.WasmRevecConversionPercent", 0, 101, 32];
  histogram wasm_modules_per_isolate ["V8.WasmModulesPerIsolate", 1, 1024, 30];
  histogram wasm_modules_per_engine ["V8.WasmModulesPerEngine", 1, 1024, 30];
  histogram liftoff_bailout_reasons ["V8.LiftoffBailoutReasons", 0, 20, 21];
  histogram wasm_memory_protection_keys_support ["V8.WasmMemoryProtectionKeysSupport", 0, 1, 2];
  histogram wasm_avx2_support ["V8.WasmAvx2Support", 0, 1, 2];
  histogram turbofan_ticks ["V8.TurboFan1KTicks", 0, 100000, 200];
  histogram regexp_backtracks ["V8.RegExpBacktracks", 1, 100000000, 50];
  histogram wasm_cache_count ["V8.WasmCacheCount", 0, 100, 101];
  histogram external_pointers_count ["V8.SandboxedExternalPointersCount", 0,
    K_MAX_EXTERNAL_POINTERS, 101];
  histogram trusted_pointers_count ["V8.SandboxedTrustedPointersCount", 0,
    K_MAX_TRUSTED_POINTERS, 101];
  histogram cppheap_pointers_count ["V8.SandboxedCppHeapPointersCount", 0,
    K_MAX_CPP_HEAP_POINTERS, 101];
  histogram js_dispatch_table_entries_count ["V8.JSDispatchTableEntriesCount", 0,
    K_MAX_JS_DISPATCH_ENTRIES, 101];
  histogram external_pointer_table_compaction_outcome
    ["V8.ExternalPointerTableCompactionOutcome", 0, 2, 3];
  histogram wasm_compilation_method ["V8.WasmCompilationMethod", 0, 4, 5];
  histogram esm_modules_per_page ["V8.ESMModulesPerPage", 1, 1024, 30];
  histogram esm_import_graph_depth ["V8.ESMImportGraphDepth", 1, 128, 20];

  // HISTOGRAM_RANGE_LIST_SLOW(V8_ENABLE_DRUMBRAKE,4 项;字段常驻,
  // EnsureCreated 由 v8_flags.slow_histograms 门控):
  slow_histogram wasm_jit_execution_ratio ["V8.JitWasmExecutionPercentage", 0, 100000, 101];
  slow_histogram wasm_jit_execution_too_slow ["V8.JitWasmExecutionTooSlow", 0, 100000, 101];
  slow_histogram wasm_jitless_execution_ratio ["V8.JitlessWasmExecutionPercentage", 0, 100000, 101];
  slow_histogram wasm_jitless_execution_too_slow ["V8.JitlessWasmExecutionTooSlow", 0, 100000, 101];

  // NESTED_TIMED_HISTOGRAM_LIST(HT name, caption, max, unit;24 项):
  nested_timed gc_incremental_marking ["V8.GCIncrementalMarking", 10000, MILLISECOND];
  nested_timed gc_incremental_marking_start ["V8.GCIncrementalMarkingStart", 10000, MILLISECOND];
  nested_timed gc_minor_incremental_marking_start
    ["V8.GCMinorIncrementalMarkingStart", 10000, MILLISECOND];
  nested_timed gc_low_memory_notification ["V8.GCLowMemoryNotification", 10000, MILLISECOND];
  nested_timed collect_source_positions ["V8.CollectSourcePositions", 1000000, MICROSECOND];
  nested_timed compile ["V8.CompileMicroSeconds", 1000000, MICROSECOND];
  nested_timed compile_eval ["V8.CompileEvalMicroSeconds", 1000000, MICROSECOND];
  nested_timed compile_serialize ["V8.CompileSerializeMicroSeconds", 100000, MICROSECOND];
  nested_timed compile_deserialize ["V8.CompileDeserializeMicroSeconds", 1000000, MICROSECOND];
  nested_timed compile_deserialize_module
    ["V8.CompileDeserializeMicroSeconds.Module", 1000000, MICROSECOND];
  nested_timed compile_deserialize_classic
    ["V8.CompileDeserializeMicroSeconds.Classic", 1000000, MICROSECOND];
  nested_timed compile_deserialize_main_thread
    ["V8.CompileDeserializeMicroSeconds.MainThread", 1000000, MICROSECOND];
  nested_timed compile_deserialize_main_thread_module
    ["V8.CompileDeserializeMicroSeconds.MainThread.Module", 1000000, MICROSECOND];
  nested_timed compile_deserialize_main_thread_classic
    ["V8.CompileDeserializeMicroSeconds.MainThread.Classic", 1000000, MICROSECOND];
  nested_timed snapshot_decompress ["V8.SnapshotDecompressMicroSeconds", 1000000, MICROSECOND];
  nested_timed snapshot_deserialize_rospace
    ["V8.SnapshotDeserializeRoSpaceMicroSeconds", 1000000, MICROSECOND];
  nested_timed snapshot_deserialize_isolate
    ["V8.SnapshotDeserializeIsolateMicroSeconds", 1000000, MICROSECOND];
  nested_timed snapshot_deserialize_context
    ["V8.SnapshotDeserializeContextMicroSeconds", 1000000, MICROSECOND];
  nested_timed compile_script ["V8.CompileScriptMicroSeconds", 1000000, MICROSECOND];
  nested_timed compile_script_module
    ["V8.CompileScriptMicroSeconds.Module", 1000000, MICROSECOND];
  nested_timed compile_script_classic
    ["V8.CompileScriptMicroSeconds.Classic", 1000000, MICROSECOND];
  nested_timed compile_script_main_thread
    ["V8.CompileScriptMicroSeconds.MainThread", 1000000, MICROSECOND];
  nested_timed compile_script_main_thread_module
    ["V8.CompileScriptMicroSeconds.MainThread.Module", 1000000, MICROSECOND];
  nested_timed compile_script_main_thread_classic
    ["V8.CompileScriptMicroSeconds.MainThread.Classic", 1000000, MICROSECOND];

  // NESTED_TIMED_HISTOGRAM_LIST_SLOW(1 项):
  slow_nested_timed execute ["V8.ExecuteMicroSeconds", 1000000, MICROSECOND];

  // TIMED_HISTOGRAM_LIST(67 项):
  timed gc_finalize_incremental_regular
    ["V8.GC.Event.MainThread.Full.Finalize.Incremental.Regular", 10000, MILLISECOND];
  timed gc_finalize_incremental_regular_foreground
    ["V8.GC.Event.MainThread.Full.Finalize.Incremental.Regular.Foreground", 10000, MILLISECOND];
  timed gc_finalize_incremental_regular_background
    ["V8.GC.Event.MainThread.Full.Finalize.Incremental.Regular.Background", 10000, MILLISECOND];
  timed gc_finalize_incremental_memory_reducing
    ["V8.GC.Event.MainThread.Full.Finalize.Incremental.ReduceMemory", 10000, MILLISECOND];
  timed gc_finalize_incremental_memory_reducing_foreground
    ["V8.GC.Event.MainThread.Full.Finalize.Incremental.ReduceMemory.Foreground", 10000, MILLISECOND];
  timed gc_finalize_incremental_memory_reducing_background
    ["V8.GC.Event.MainThread.Full.Finalize.Incremental.ReduceMemory.Background", 10000, MILLISECOND];
  timed gc_finalize_incremental_memory_measure
    ["V8.GC.Event.MainThread.Full.Finalize.Incremental.MeasureMemory", 10000, MILLISECOND];
  timed gc_finalize_incremental_memory_measure_foreground
    ["V8.GC.Event.MainThread.Full.Finalize.Incremental.MeasureMemory.Foreground", 10000, MILLISECOND];
  timed gc_finalize_incremental_memory_measure_background
    ["V8.GC.Event.MainThread.Full.Finalize.Incremental.MeasureMemory.Background", 10000, MILLISECOND];
  timed gc_finalize_non_incremental_regular
    ["V8.GC.Event.MainThread.Full.Finalize.NonIncremental.Regular", 10000, MILLISECOND];
  timed gc_finalize_non_incremental_regular_foreground
    ["V8.GC.Event.MainThread.Full.Finalize.NonIncremental.Regular.Foreground", 10000, MILLISECOND];
  timed gc_finalize_non_incremental_regular_background
    ["V8.GC.Event.MainThread.Full.Finalize.NonIncremental.Regular.Background", 10000, MILLISECOND];
  timed gc_finalize_non_incremental_memory_reducing
    ["V8.GC.Event.MainThread.Full.Finalize.NonIncremental.ReduceMemory", 10000, MILLISECOND];
  timed gc_finalize_non_incremental_memory_reducing_foreground
    ["V8.GC.Event.MainThread.Full.Finalize.NonIncremental.ReduceMemory.Foreground", 10000, MILLISECOND];
  timed gc_finalize_non_incremental_memory_reducing_background
    ["V8.GC.Event.MainThread.Full.Finalize.NonIncremental.ReduceMemory.Background", 10000, MILLISECOND];
  timed gc_finalize_non_incremental_memory_measure
    ["V8.GC.Event.MainThread.Full.Finalize.NonIncremental.MeasureMemory", 10000, MILLISECOND];
  timed gc_finalize_non_incremental_memory_measure_foreground
    ["V8.GC.Event.MainThread.Full.Finalize.NonIncremental.MeasureMemory.Foreground", 10000, MILLISECOND];
  timed gc_finalize_non_incremental_memory_measure_background
    ["V8.GC.Event.MainThread.Full.Finalize.NonIncremental.MeasureMemory.Background", 10000, MILLISECOND];
  timed measure_memory_delay_ms ["V8.MeasureMemoryDelayMilliseconds", 100000, MILLISECOND];
  timed gc_time_to_global_safepoint ["V8.GC.TimeToGlobalSafepoint", 10000000, MICROSECOND];
  timed gc_time_to_safepoint ["V8.GC.TimeToSafepoint", 10000000, MICROSECOND];
  timed gc_time_to_collection_on_background ["V8.GC.TimeToCollectionOnBackground",
    10000000, MICROSECOND];
  // Maglev timers.
  timed maglev_optimize_prepare ["V8.MaglevOptimizePrepare", 100000, MICROSECOND];
  timed maglev_optimize_execute ["V8.MaglevOptimizeExecute", 100000, MICROSECOND];
  timed maglev_optimize_finalize ["V8.MaglevOptimizeFinalize", 100000, MICROSECOND];
  timed maglev_optimize_total_time ["V8.MaglevOptimizeTotalTime", 1000000, MICROSECOND];
  // TurboFan timers.
  timed turbofan_optimize_prepare ["V8.TurboFanOptimizePrepare", 1000000, MICROSECOND];
  timed turbofan_optimize_execute ["V8.TurboFanOptimizeExecute", 1000000, MICROSECOND];
  timed turbofan_optimize_finalize ["V8.TurboFanOptimizeFinalize", 1000000, MICROSECOND];
  timed turbofan_optimize_total_foreground ["V8.TurboFanOptimizeTotalForeground",
    10000000, MICROSECOND];
  timed turbofan_optimize_total_background ["V8.TurboFanOptimizeTotalBackground",
    10000000, MICROSECOND];
  timed turbofan_optimize_total_time ["V8.TurboFanOptimizeTotalTime", 10000000, MICROSECOND];
  timed turbofan_optimize_non_concurrent_total_time
    ["V8.TurboFanOptimizeNonConcurrentTotalTime", 10000000, MICROSECOND];
  timed turbofan_optimize_concurrent_total_time
    ["V8.TurboFanOptimizeConcurrentTotalTime", 10000000, MICROSECOND];
  timed turbofan_osr_prepare ["V8.TurboFanOptimizeForOnStackReplacementPrepare",
    1000000, MICROSECOND];
  timed turbofan_osr_execute ["V8.TurboFanOptimizeForOnStackReplacementExecute",
    1000000, MICROSECOND];
  timed turbofan_osr_finalize ["V8.TurboFanOptimizeForOnStackReplacementFinalize",
    1000000, MICROSECOND];
  timed turbofan_osr_total_time ["V8.TurboFanOptimizeForOnStackReplacementTotalTime",
    10000000, MICROSECOND];
  // Wasm timers.
  timed wasm_compile_asm_module_time ["V8.WasmCompileModuleMicroSeconds.asm", 10000000, MICROSECOND];
  timed wasm_compile_wasm_module_time ["V8.WasmCompileModuleMicroSeconds.wasm", 10000000, MICROSECOND];
  timed wasm_async_compile_wasm_module_time ["V8.WasmCompileModuleAsyncMicroSeconds",
    100000000, MICROSECOND];
  timed wasm_streaming_compile_wasm_module_time ["V8.WasmCompileModuleStreamingMicroSeconds",
    100000000, MICROSECOND];
  timed wasm_streaming_finish_wasm_module_time ["V8.WasmFinishModuleStreamingMicroSeconds",
    100000000, MICROSECOND];
  timed wasm_deserialization_time ["V8.WasmDeserializationTimeMilliSeconds", 10000, MILLISECOND];
  timed wasm_compile_asm_function_time ["V8.WasmCompileFunctionMicroSeconds.asm", 1000000, MICROSECOND];
  timed wasm_compile_wasm_function_time ["V8.WasmCompileFunctionMicroSeconds.wasm", 1000000, MICROSECOND];
  timed wasm_compile_huge_function_time ["V8.WasmCompileHugeFunctionMilliSeconds", 100000, MILLISECOND];
  timed wasm_instantiate_wasm_module_time ["V8.WasmInstantiateModuleMicroSeconds.wasm",
    10000000, MICROSECOND];
  timed wasm_instantiate_asm_module_time ["V8.WasmInstantiateModuleMicroSeconds.asm",
    10000000, MICROSECOND];
  timed wasm_lazy_compile_time ["V8.WasmLazyCompileTimeMicroSeconds", 100000000, MICROSECOND];
  timed wasm_compile_after_deserialize ["V8.WasmCompileAfterDeserializeMilliSeconds",
    1000000, MILLISECOND];
  // Total compilation time incl. caching/parsing for various cache states.
  timed compile_script_with_produce_cache ["V8.CompileScriptMicroSeconds.ProduceCache",
    1000000, MICROSECOND];
  timed compile_script_with_isolate_cache_hit ["V8.CompileScriptMicroSeconds.IsolateCacheHit",
    1000000, MICROSECOND];
  timed compile_script_with_consume_cache ["V8.CompileScriptMicroSeconds.ConsumeCache",
    1000000, MICROSECOND];
  timed compile_script_consume_failed ["V8.CompileScriptMicroSeconds.ConsumeCache.Failed",
    1000000, MICROSECOND];
  timed compile_script_no_cache_other ["V8.CompileScriptMicroSeconds.NoCache.Other",
    1000000, MICROSECOND];
  timed compile_script_no_cache_because_inline_script
    ["V8.CompileScriptMicroSeconds.NoCache.InlineScript", 1000000, MICROSECOND];
  timed compile_script_no_cache_because_script_too_small
    ["V8.CompileScriptMicroSeconds.NoCache.ScriptTooSmall", 1000000, MICROSECOND];
  timed compile_script_no_cache_because_cache_too_cold
    ["V8.CompileScriptMicroSeconds.NoCache.CacheTooCold", 1000000, MICROSECOND];
  timed compile_script_streaming_finalization
    ["V8.CompileScriptMicroSeconds.StreamingFinalization", 1000000, MICROSECOND];
  timed compile_script_on_background ["V8.CompileScriptMicroSeconds.BackgroundThread",
    1000000, MICROSECOND];
  timed compile_script_on_background_module
    ["V8.CompileScriptMicroSeconds.BackgroundThread.Module", 1000000, MICROSECOND];
  timed compile_script_on_background_classic
    ["V8.CompileScriptMicroSeconds.BackgroundThread.Classic", 1000000, MICROSECOND];
  timed compile_function_on_background ["V8.CompileFunctionMicroSeconds.BackgroundThread",
    1000000, MICROSECOND];
  timed deserialize_script_on_background
    ["V8.CompileScriptMicroSeconds.ConsumeCache.BackgroundThread", 1000000, MICROSECOND];
  timed deserialize_script_on_background_module
    ["V8.CompileScriptMicroSeconds.ConsumeCache.BackgroundThread.Module", 1000000, MICROSECOND];
  timed deserialize_script_on_background_classic
    ["V8.CompileScriptMicroSeconds.ConsumeCache.BackgroundThread.Classic", 1000000, MICROSECOND];

  // AGGREGATABLE_HISTOGRAM_TIMER_LIST(1 项):
  aggregatable_timer compile_lazy ["V8.CompileLazyMicroSeconds"];

  // HISTOGRAM_PERCENTAGE_LIST(1 项):
  percentage external_fragmentation_total ["V8.MemoryExternalFragmentationTotal"];

  // HISTOGRAM_LEGACY_MEMORY_LIST(2 项;min=1000, max=500000, buckets=50):
  legacy_memory heap_sample_total_committed ["V8.MemoryHeapSampleTotalCommitted"];
  legacy_memory heap_sample_total_used ["V8.MemoryHeapSampleTotalUsed"];

  // STATS_COUNTER_LIST(38 项)+ STATS_COUNTER_NATIVE_CODE_LIST(8 项),
  // caption 加 "c:" 前缀:
  stats_counter global_handles ["c:V8.GlobalHandles"];
  stats_counter alive_after_last_gc ["c:V8.AliveAfterLastGC"];
  stats_counter compilation_cache_hits ["c:V8.CompilationCacheHits"];
  stats_counter compilation_cache_misses ["c:V8.CompilationCacheMisses"];
  stats_counter compilation_cache_partial_hits ["c:V8.CompilationCachePartialHits"];
  stats_counter deopts ["c:V8.Deopts"];
  stats_counter utility_deopts ["c:V8.UtilityDeopts"];
  stats_counter objs_since_last_young ["c:V8.ObjsSinceLastYoung"];
  stats_counter objs_since_last_full ["c:V8.ObjsSinceLastFull"];
  stats_counter gc_compactor_caused_by_request ["c:V8.GCCompactorCausedByRequest"];
  stats_counter gc_compactor_caused_by_promoted_data ["c:V8.GCCompactorCausedByPromotedData"];
  stats_counter gc_compactor_caused_by_oldspace_exhaustion
    ["c:V8.GCCompactorCausedByOldspaceExhaustion"];
  stats_counter enum_cache_hits ["c:V8.EnumCacheHits"];
  stats_counter enum_cache_misses ["c:V8.EnumCacheMisses"];
  stats_counter maps_created ["c:V8.MapsCreated"];
  stats_counter megamorphic_stub_cache_updates ["c:V8.MegamorphicStubCacheUpdates"];
  stats_counter regexp_entry_runtime ["c:V8.RegExpEntryRuntime"];
  stats_counter stack_interrupts ["c:V8.StackInterrupts"];
  stats_counter new_space_bytes_available ["c:V8.MemoryNewSpaceBytesAvailable"];
  stats_counter new_space_bytes_committed ["c:V8.MemoryNewSpaceBytesCommitted"];
  stats_counter new_space_bytes_used ["c:V8.MemoryNewSpaceBytesUsed"];
  stats_counter old_space_bytes_available ["c:V8.MemoryOldSpaceBytesAvailable"];
  stats_counter old_space_bytes_committed ["c:V8.MemoryOldSpaceBytesCommitted"];
  stats_counter old_space_bytes_used ["c:V8.MemoryOldSpaceBytesUsed"];
  stats_counter code_space_bytes_available ["c:V8.MemoryCodeSpaceBytesAvailable"];
  stats_counter code_space_bytes_committed ["c:V8.MemoryCodeSpaceBytesCommitted"];
  stats_counter code_space_bytes_used ["c:V8.MemoryCodeSpaceBytesUsed"];
  stats_counter map_space_bytes_available ["c:V8.MemoryMapSpaceBytesAvailable"];
  stats_counter map_space_bytes_committed ["c:V8.MemoryMapSpaceBytesCommitted"];
  stats_counter map_space_bytes_used ["c:V8.MemoryMapSpaceBytesUsed"];
  stats_counter lo_space_bytes_available ["c:V8.MemoryLoSpaceBytesAvailable"];
  stats_counter lo_space_bytes_committed ["c:V8.MemoryLoSpaceBytesCommitted"];
  stats_counter lo_space_bytes_used ["c:V8.MemoryLoSpaceBytesUsed"];
  stats_counter wasm_generated_code_size ["c:V8.WasmGeneratedCodeBytes"];
  stats_counter wasm_reloc_size ["c:V8.WasmRelocBytes"];
  stats_counter wasm_deopt_data_size ["c:V8.WasmDeoptDataBytes"];
  stats_counter wasm_lazily_compiled_functions ["c:V8.WasmLazilyCompiledFunctions"];
  stats_counter wasm_compiled_export_wrapper ["c:V8.WasmCompiledExportWrappers"];
  // 生成代码可自增的计数器(独立清单以便重定位):
  stats_counter write_barriers ["c:V8.WriteBarriers"];
  stats_counter regexp_entry_native ["c:V8.RegExpEntryNative"];
  stats_counter megamorphic_stub_cache_probes ["c:V8.MegamorphicStubCacheProbes"];
  stats_counter megamorphic_stub_cache_misses ["c:V8.MegamorphicStubCacheMisses"];
  stats_counter number_string_cache_smi_probes ["c:V8.NumberStringCacheSmiProbes"];
  stats_counter number_string_cache_smi_misses ["c:V8.NumberStringCacheSmiMisses"];
  stats_counter number_string_cache_double_probes ["c:V8.NumberStringCacheDoubleProbes"];
  stats_counter number_string_cache_double_misses ["c:V8.NumberStringCacheDoubleMisses"];
}

// ============================================================================
// DelayedCounterUpdates(counters.h 尾段)
// ============================================================================

/// 对应 `using GetHistogramFn = Histogram* (Counters::*)()`(成员函数指针
/// 在 Rust 以 `fn(&Counters) -> &Histogram` 的访问器函数指针承载)。
pub type GetHistogramFn = fn(&Counters) -> &Histogram;
/// 对应 `using GetTimedHistogramFn = TimedHistogram* (Counters::*)()`。
pub type GetTimedHistogramFn = fn(&Counters) -> &TimedHistogram;
/// 对应 `using GetStatsCounterFn = StatsCounter* (Counters::*)()`。
pub type GetStatsCounterFn = fn(&Counters) -> &StatsCounter;

/// 对应 `using AnyUpdate = std::variant<HistogramUpdate, TimedHistogramUpdate,
/// StatsCounterUpdate>`。
#[derive(Clone, Copy)]
pub enum AnyUpdate {
  /// HistogramUpdate{fn, sample}。
  Histogram(GetHistogramFn, i32),
  /// TimedHistogramUpdate{fn, base::TimeDelta}。
  Timed(GetTimedHistogramFn, TimeDelta),
  /// StatsCounterUpdate{fn, increment}。
  Stats(GetStatsCounterFn, i32),
}

/// 对应 `class DelayedCounterUpdates`:暂存直方图样本/计数增量,稍后统一
/// 发布到 `Counters`(C++ 参数 `Isolate*` 改为 `&Counters`,isolate 桥接推迟)。
pub struct DelayedCounterUpdates {
  has_updates: AtomicBool,
  outstanding_updates: Mutex<Vec<AnyUpdate>>,
}

impl DelayedCounterUpdates {
  pub fn new() -> Self {
    Self {
      has_updates: AtomicBool::new(false),
      outstanding_updates: Mutex::new(Vec::new()),
    }
  }

  /// 对应 `Publish(Isolate*)`(快路径:无暂存更新直接返回)。
  pub fn publish(&self, counters: &Counters) {
    if !self.has_updates.load(Ordering::Relaxed) {
      return;
    }
    self.publish_impl(counters);
  }

  /// 对应 `V8_NOINLINE PublishImpl(Isolate*)`。
  fn publish_impl(&self, counters: &Counters) {
    let updates: Vec<AnyUpdate> = {
      let mut guard = self.outstanding_updates.lock();
      let mut local = Vec::new();
      swap(&mut *guard, &mut local);
      self.has_updates.store(false, Ordering::Relaxed);
      local
    };
    for update in updates {
      match update {
        AnyUpdate::Histogram(fn_, sample) => fn_(counters).add_sample(sample),
        AnyUpdate::Timed(fn_, sample) => fn_(counters).add_timed_sample(sample),
        AnyUpdate::Stats(fn_, increment) => fn_(counters).increment(increment),
      }
    }
  }

  /// 对应 `AddSample(GetHistogramFn, int)`。
  pub fn add_sample(&self, fn_: GetHistogramFn, sample: i32) {
    let mut guard = self.outstanding_updates.lock();
    guard.push(AnyUpdate::Histogram(fn_, sample));
    self.has_updates.store(true, Ordering::Relaxed);
  }

  /// 对应 `AddTimedSample(GetTimedHistogramFn, base::TimeDelta)`。
  pub fn add_timed_sample(&self, fn_: GetTimedHistogramFn, sample: TimeDelta) {
    let mut guard = self.outstanding_updates.lock();
    guard.push(AnyUpdate::Timed(fn_, sample));
    self.has_updates.store(true, Ordering::Relaxed);
  }

  /// 对应 `AddIncrement(GetStatsCounterFn, int)`。
  pub fn add_increment(&self, fn_: GetStatsCounterFn, increment: i32) {
    let mut guard = self.outstanding_updates.lock();
    guard.push(AnyUpdate::Stats(fn_, increment));
    self.has_updates.store(true, Ordering::Relaxed);
  }

  /// 对应 `AddAll(DelayedCounterUpdates&&)`(rvalue → `&mut` 借用取走)。
  pub fn add_all(&self, other: &mut DelayedCounterUpdates) {
    // No need to take `other`'s mutex, as it's an r-value ref, so unshared.
    if other.outstanding_updates.lock().is_empty() {
      return;
    }
    let mut guard = self.outstanding_updates.lock();
    let mut other_guard = other.outstanding_updates.lock();
    if guard.is_empty() {
      swap(&mut *guard, &mut *other_guard);
    } else {
      guard.append(&mut *other_guard);
    }
    self.has_updates.store(true, Ordering::Relaxed);
  }

  /// 对应 `EstimateCurrentMemoryConsumption()`。
  pub fn estimate_current_memory_consumption(&self) -> usize {
    let guard = self.outstanding_updates.lock();
    size_of::<Self>() + guard.capacity() * size_of::<AnyUpdate>()
  }

  /// 对应 `NumOutstandingUpdates()`(debug 用)。
  pub fn num_outstanding_updates(&self) -> usize {
    self.outstanding_updates.lock().len()
  }
}

impl Default for DelayedCounterUpdates {
  fn default() -> Self {
    Self::new()
  }
}
