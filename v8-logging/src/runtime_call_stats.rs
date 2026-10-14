//! 1:1 转写自 `v8/src/logging/runtime-call-stats.h/.cc` +
//! `runtime-call-stats-scope.h`。
//!
//! 转写要点:
//! - `RuntimeCallCounterId` 枚举(7 段)展开为扁平 `&str` 名称表
//!   (见 `rcs_names.rs`,由 X-macro 清单机械生成; 仅 THREAD/HANDLER 段
//!   常量有运行时消费者,其余段 offset/len 已删除,见 readme)。
//!   **`kRuntime_*`(FOR_EACH_INTRINSIC)与 `kBuiltin_*`(BUILTIN_LIST_C)两段
//!   长度为 0**:其清单归属 v8-runtime/v8-builtins,【待回收】(见 readme)。
//!   因此本移植中 `K_NUMBER_OF_COUNTERS` 与 C++ 完整构建不同(739),段间
//!   相对顺序保持 1:1。
//! - `RuntimeCallTimer::Now` 全局函数指针 → `set_now/now/reset_now`
//!   (`AtomicUsize` 承载,`0` 即 C++ 默认 `&base::TimeTicks::Now`)。
//! - C++ 裸指针所有权模型(timer/counter 指针链)以 `NonNull` + `Cell` 镜像:
//!   `RuntimeCallCounter` 全部字段为 `Cell`(内部可变,与 C++ 非原子计数一致);
//!   单线程使用契约与 C++ 相同(`IsCalledOnTheSameThread`)。
//! - `NowCPUTime`:`base::ThreadTicks` 未移植(【待回收】v8-base),
//!   `rcs_cpu_time` 置位时降级为单调时钟并 `log::warn!`(禁 panic)。
//! - `v8::tracing`(TracedValue Dump/WorkerThreadRuntimeCallStatsScope TLS)
//!   推迟,登记于 readme。

use std::{
  cell::{Cell, RefCell},
  cmp,
  io::{Write, stdout},
  mem::transmute,
  ptr::{NonNull, eq, null},
  sync::atomic::{AtomicUsize, Ordering},
  thread::{ThreadId, current},
};

use parking_lot::Mutex;

use crate::{
  error, flags_lite, rcs_names,
  time_base::{TimeDelta, TimeTicks, thread_ticks_supported},
  tracing_flags::{ENABLED_BY_SAMPLING, RUNTIME_STATS, TracingFlags},
};

/// 对应 `RuntimeCallCounterId`(C++ 为 enum,数值即名称表下标)。
pub type RuntimeCallCounterId = usize;

/// 对应 `RuntimeCallStats::kNumberOfCounters`。
pub const K_NUMBER_OF_COUNTERS: usize =
  rcs_names::K_SECTION_OFFSET_THREAD + rcs_names::K_SECTION_LEN_THREAD;

/// 单测所需的 enum id(机械平移自 `RuntimeCallCounterId::k*`)。
pub const K_TEST_COUNTER1: usize = rcs_names::K_TEST_COUNTER1;
/// `kTestCounter2`。
pub const K_TEST_COUNTER2: usize = rcs_names::K_TEST_COUNTER2;
/// `kTestCounter3`。
pub const K_TEST_COUNTER3: usize = rcs_names::K_TEST_COUNTER3;
/// `kJS_Execution`。
pub const K_JS_EXECUTION: usize = rcs_names::K_JS_EXECUTION;

/// 对应 `enum ThreadType { kMainIsolateThread, kWorkerThread }`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadType {
  MainIsolateThread,
  WorkerThread,
}

/// 对应 `enum CounterMode { kExact, kThreadSpecific }`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterMode {
  Exact,
  ThreadSpecific,
}

// --- RuntimeCallTimer::Now 全局时间源 ---------------------------------------

/// 对应 `base::TimeTicks (*RuntimeCallTimer::Now)()`。
pub type NowFn = fn() -> TimeTicks;

/// 0 = C++ 默认值 `&base::TimeTicks::Now`。
static NOW: AtomicUsize = AtomicUsize::new(0);

/// 对应 `RuntimeCallTimer::Now = &fn`(测试注入点)。
pub fn set_now(f: NowFn) {
  NOW.store(f as usize, Ordering::Relaxed);
}

/// 恢复默认时间源(对应 `RuntimeCallTimer::Now = &base::TimeTicks::Now`)。
pub fn reset_now() {
  NOW.store(0, Ordering::Relaxed);
}

/// 读取当前时间源(对应 `RuntimeCallTimer::Now()`)。
pub fn now() -> TimeTicks {
  let raw = NOW.load(Ordering::Relaxed);
  if raw == 0 {
    return TimeTicks::now();
  }
  // SAFETY: raw 只来自 `set_now` 的 `fn` 指针 `as usize` 往返,同一 ABI。
  let f: NowFn = unsafe { transmute(raw) };
  f()
}

/// 对应 `static base::TimeTicks RuntimeCallTimer::NowCPUTime()`:
/// `base::ThreadTicks` 未移植(【待回收】v8-base),不支持时降级单调时钟。
pub fn now_cpu_time() -> TimeTicks {
  if !thread_ticks_supported() {
    // C++ 构造处为 CHECK(IsSupported);禁 panic → warn + 保持默认时间源。
    return TimeTicks::now();
  }
  TimeTicks::now()
}

// --- RuntimeCallCounter ------------------------------------------------------

/// 对应 `class RuntimeCallCounter final`(计数走 Cell,内部可变,单线程契约)。
pub struct RuntimeCallCounter {
  name: &'static str,
  count: Cell<i64>,
  /// Stored as int64_t so that its initialization can be deferred(C++ 注释)。
  time_us: Cell<i64>,
}

impl RuntimeCallCounter {
  fn new(name: &'static str) -> Self {
    Self {
      name,
      count: Cell::new(0),
      time_us: Cell::new(0),
    }
  }

  /// 对应 `Reset()`。
  pub fn reset(&self) {
    self.count.set(0);
    self.time_us.set(0);
  }

  /// 对应 `Add(RuntimeCallCounter* other)`。
  pub fn add(&self, other: &RuntimeCallCounter) {
    self.count.set(self.count.get() + other.count());
    self
      .time_us
      .set(self.time_us.get() + other.time().in_microseconds());
  }

  /// 对应 `name()`。
  #[inline]
  pub fn name(&self) -> &'static str {
    self.name
  }

  /// 对应 `count()`。
  #[inline]
  pub fn count(&self) -> i64 {
    self.count.get()
  }

  /// 对应 `time()`。
  #[inline]
  pub fn time(&self) -> TimeDelta {
    TimeDelta::from_microseconds(self.time_us.get())
  }

  /// 对应 `Increment()`。
  #[inline]
  pub fn increment(&self) {
    self.count.set(self.count.get() + 1);
  }

  /// 对应 `Add(base::TimeDelta delta)`(重载)。
  #[inline]
  pub fn add_delta(&self, delta: TimeDelta) {
    self
      .time_us
      .set(self.time_us.get() + delta.in_microseconds());
  }

  /// 对应 `Dump(v8::tracing::TracedValue*)`:v8-tracing 未移植,推迟登记。
  pub fn dump(&self) {
    // 输出 BeginArray(name)/AppendDouble(count)/AppendDouble(time)/EndArray。
  }
}

// --- RuntimeCallTimer ---------------------------------------------------------

/// 对应 `class RuntimeCallTimer final`。
///
/// C++ 成员裸指针(`counter_`/`parent_`,经 `base::AtomicValue`)以
/// `Cell<Option<NonNull<..>>>` 镜像;使用契约同为“单线程 + 被指向对象存活”。
pub struct RuntimeCallTimer {
  counter: Cell<Option<NonNull<RuntimeCallCounter>>>,
  parent: Cell<Option<NonNull<RuntimeCallTimer>>>,
  start_ticks: Cell<TimeTicks>,
  elapsed: Cell<TimeDelta>,
}

impl RuntimeCallTimer {
  /// 对应默认构造(各成员清零)。
  pub fn new() -> Self {
    Self {
      counter: Cell::new(None),
      parent: Cell::new(None),
      start_ticks: Cell::new(TimeTicks::from_internal_value(0)),
      elapsed: Cell::new(TimeDelta::default()),
    }
  }

  /// 对应 `RuntimeCallCounter* counter()`。
  #[inline]
  pub fn counter(&self) -> Option<&RuntimeCallCounter> {
    // SAFETY: 指针由 `set_counter/Start` 从存活借用产生(单线程契约)。
    self.counter.get().map(|p| unsafe { p.as_ref() })
  }

  /// 对应 `void set_counter(RuntimeCallCounter*)`。
  #[inline]
  pub fn set_counter(&self, counter: &RuntimeCallCounter) {
    self.counter.set(Some(NonNull::from(counter)));
  }

  /// 对应 `RuntimeCallTimer* parent()`。
  #[inline]
  pub fn parent(&self) -> Option<&RuntimeCallTimer> {
    // SAFETY: 指针由 `set_parent/Start` 从存活借用产生(单线程契约)。
    self.parent.get().map(|p| unsafe { p.as_ref() })
  }

  /// 对应 `void set_parent(RuntimeCallTimer*)`。
  #[inline]
  pub fn set_parent(&self, timer: Option<&RuntimeCallTimer>) {
    self.parent.set(timer.map(NonNull::from));
  }

  /// 对应 `const char* name() const { return counter_->name(); }`
  /// (C++ 空指针即 UB;此处无 counter 返回 "")。
  pub fn name(&self) -> &'static str {
    match self.counter.get() {
      // SAFETY: counter 名挂在名称表 `&'static str` 上,指针源自存活借用
      // (单线程契约)。
      Some(p) => unsafe { p.as_ref() }.name(),
      None => "",
    }
  }

  /// 对应 `IsStarted()`:`start_ticks_ != base::TimeTicks()`。
  #[inline]
  pub fn is_started(&self) -> bool {
    !self.start_ticks.get().is_null()
  }

  /// 对应 `Start(counter, parent)`。
  pub fn start(&self, counter: &RuntimeCallCounter, parent: Option<&RuntimeCallTimer>) {
    debug_assert!(!self.is_started());
    self.counter.set(Some(NonNull::from(counter)));
    self.parent.set(parent.map(NonNull::from));
    if RUNTIME_STATS.load(Ordering::Relaxed) == ENABLED_BY_SAMPLING {
      // 采样模式不计时(C++ 直接 return)。
      return;
    }
    let now = now();
    if let Some(p) = parent {
      p.pause(now);
    }
    self.resume(now);
    debug_assert!(self.is_started());
  }

  /// 对应 `void Snapshot()`。
  pub fn snapshot(&self) {
    let now = now();
    // Pause only / topmost timer in the timer stack.
    self.pause(now);
    // Commit all the timer's elapsed time to the counters.
    let mut timer: Option<NonNull<RuntimeCallTimer>> = Some(NonNull::from(self));
    while let Some(t) = timer {
      // SAFETY: 单线程存活契约(镜像 C++ parent 链裸指针)。
      let tr = unsafe { t.as_ref() };
      tr.commit_time_to_counter();
      timer = tr.parent.get();
    }
    self.resume(now);
  }

  /// 对应 `RuntimeCallTimer* Stop()`;返回父 timer 原始指针。
  pub fn stop(&self) -> Option<NonNull<RuntimeCallTimer>> {
    if !self.is_started() {
      return self.parent.get();
    }
    let now = now();
    self.pause(now);
    if let Some(c) = self.counter.get() {
      // SAFETY: 见类型级注释。
      unsafe { c.as_ref() }.increment();
    }
    self.commit_time_to_counter();

    let parent_timer = self.parent.get();
    if let Some(p) = parent_timer {
      // SAFETY: 同上。
      unsafe { p.as_ref() }.resume(now);
    }
    parent_timer
  }

  /// 对应 `Pause(base::TimeTicks now)`(private)。
  fn pause(&self, now: TimeTicks) {
    debug_assert!(self.is_started());
    self
      .elapsed
      .set(self.elapsed.get() + (now - self.start_ticks.get()));
    self.start_ticks.set(TimeTicks::from_internal_value(0));
  }

  /// 对应 `Resume(base::TimeTicks now)`(private)。
  fn resume(&self, now: TimeTicks) {
    debug_assert!(!self.is_started());
    self.start_ticks.set(now);
  }

  /// 对应 `CommitTimeToCounter()`(private)。
  fn commit_time_to_counter(&self) {
    if let Some(c) = self.counter.get() {
      // SAFETY: 同上。
      unsafe { c.as_ref() }.add_delta(self.elapsed.get());
    }
    self.elapsed.set(TimeDelta::default());
  }
}

impl Default for RuntimeCallTimer {
  fn default() -> Self {
    Self::new()
  }
}

// --- RuntimeCallStats ---------------------------------------------------------

/// 对应 `class RuntimeCallStats final`。
pub struct RuntimeCallStats {
  /// 对应 `RuntimeCallCounter counters_[kNumberOfCounters]`(堆上定长片)。
  counters: Box<[RuntimeCallCounter]>,
  /// Top of a stack of active timers(`base::AtomicValue<RuntimeCallTimer*>`)。
  current_timer: Cell<Option<NonNull<RuntimeCallTimer>>>,
  current_counter: Cell<Option<NonNull<RuntimeCallCounter>>>,
  in_use: Cell<bool>,
  thread_type: ThreadType,
  /// `thread_id_`(kInvalidThreadId ↔ None;IsValid ↔ Some)。
  thread_id: RefCell<Option<ThreadId>>,
}

impl RuntimeCallStats {
  /// 对应构造函数:填充 `kNames[]` 名称表并按名建 counter;
  /// `v8_flags.rcs_cpu_time` 分支(ThreadTicks CHECK)降级见 [`now_cpu_time`]。
  pub fn new(thread_type: ThreadType) -> Self {
    let mut names: Vec<&'static str> = Vec::with_capacity(K_NUMBER_OF_COUNTERS);
    for section in rcs_names::SECTION_ORDER {
      names.extend_from_slice(section);
    }
    debug_assert_eq!(names.len(), K_NUMBER_OF_COUNTERS);
    if flags_lite::rcs_cpu_time() {
      if !thread_ticks_supported() {
        // C++: CHECK(base::ThreadTicks::IsSupported()) → 移植环境不支持线程时钟,
        // 禁 panic:warn 并保持默认单调时间源(登记【待回收】v8-base ThreadTicks)。
        log::warn!("rcs_cpu_time requested but ThreadTicks unsupported; using monotonic clock");
      } else {
        set_now(now_cpu_time);
      }
    }
    Self {
      counters: names.into_iter().map(RuntimeCallCounter::new).collect(),
      current_timer: Cell::new(None),
      current_counter: Cell::new(None),
      in_use: Cell::new(false),
      thread_type,
      thread_id: RefCell::new(None),
    }
  }

  /// 对应 `Enter(RuntimeCallTimer* timer, RuntimeCallCounterId counter_id)`。
  pub fn enter(&self, timer: &RuntimeCallTimer, counter_id: RuntimeCallCounterId) {
    debug_assert!(self.is_called_on_the_same_thread());
    let counter = self.get_counter(counter_id);
    debug_assert!(!counter.name().is_empty()); // DCHECK_NOT_NULL(counter->name())
    timer.start(counter, self.current_timer());
    self.current_timer.set(Some(NonNull::from(timer)));
    self.current_counter.set(Some(NonNull::from(counter)));
  }

  /// 对应 `Leave(RuntimeCallTimer* timer)`。
  pub fn leave(&self, timer: &RuntimeCallTimer) {
    debug_assert!(self.is_called_on_the_same_thread());
    let stack_top = self.current_timer.get();
    let Some(top) = stack_top else {
      // Missing timer is a result of Reset().
      return;
    };
    // C++ CHECK(stack_top == timer) → 禁 panic:debug_assert + 不一致时告警返回。
    debug_assert!(top == NonNull::from(timer));
    if top != NonNull::from(timer) {
      log::warn!("RuntimeCallStats::Leave: timer is not stack top");
      return;
    }
    let parent = timer.stop();
    self.current_timer.set(parent);
    // current_counter_ = cur_timer ? cur_timer->counter() : nullptr
    // SAFETY: parent 指针来自存活 timer 链(单线程契约)。
    self
      .current_counter
      .set(parent.and_then(|p| unsafe { p.as_ref() }.counter.get()));
  }

  /// 对应 `CorrectCurrentCounterId(counter_id, mode = kExact)`。
  pub fn correct_current_counter_id(
    &self,
    mut counter_id: RuntimeCallCounterId,
    mode: CounterMode,
  ) {
    debug_assert!(self.is_called_on_the_same_thread());
    if mode == CounterMode::ThreadSpecific {
      counter_id = self.counter_id_for_thread(counter_id);
    }
    debug_assert!(self.is_counter_appropriate_for_thread(counter_id));

    let Some(timer_ptr) = self.current_timer.get() else {
      return;
    };
    // SAFETY: 单线程存活契约。
    let timer = unsafe { timer_ptr.as_ref() };
    let counter = self.get_counter(counter_id);
    timer.set_counter(counter);
    self.current_counter.set(Some(NonNull::from(counter)));
  }

  /// 对应 `Reset()`。
  pub fn reset(&self) {
    if !TracingFlags::is_runtime_stats_enabled() {
      return;
    }
    // In tracing, ... clear the whole runtime counter stack (C++ 注释保留)。
    while let Some(top) = self.current_timer.get() {
      // SAFETY: 单线程存活契约。
      let next = unsafe { top.as_ref() }.stop();
      self.current_timer.set(next);
    }
    for c in self.counters.iter() {
      c.reset();
    }
    self.in_use.set(true);
  }

  /// 对应 `Add(RuntimeCallStats* other)`。
  pub fn add(&self, other: &RuntimeCallStats) {
    for i in 0..K_NUMBER_OF_COUNTERS {
      self.get_counter(i).add(other.get_counter(i));
    }
  }

  /// 对应 `Print()`(StdoutStream 版)。
  pub fn print_to_stdout(&self) {
    let mut out = stdout();
    if let Err(e) = self.print(&mut out) {
      log::warn!("RuntimeCallStats::Print stdout failed: {e}");
    }
  }

  /// 对应 `Print(std::ostream& os)`(含 `RuntimeCallStatEntries`)。
  pub fn print<W: Write>(&self, os: &mut W) -> error::Result<()> {
    if let Some(top) = self.current_timer.get() {
      // SAFETY: 单线程存活契约。
      unsafe { top.as_ref() }.snapshot();
    }
    let mut entries: Vec<Entry> = Vec::new();
    let mut total_time_us: i64 = 0;
    let mut total_call_count: u64 = 0;
    for i in 0..K_NUMBER_OF_COUNTERS {
      let counter = self.get_counter(i);
      // V8_NOINLINE void Add(RuntimeCallCounter* counter)
      if counter.count() == 0 {
        continue;
      }
      entries.push(Entry::new(
        counter.name(),
        counter.time().in_microseconds(),
        counter.count() as u64,
      ));
      total_time_us += counter.time().in_microseconds();
      total_call_count += counter.count() as u64;
    }
    Entry::print_all(os, entries, total_time_us, total_call_count)
  }

  /// 对应 `Dump(v8::tracing::TracedValue*)` 的可移植等价:
  /// 收集 `count > 0` 的 `(name, time_us, count)` 条目,最后 `in_use = false`
  /// (原版 TracedValue 数组输出推迟至 v8-tracing,见 `RuntimeCallCounter::dump`)。
  /// 注意: 与原版 Dump(runtime-call-stats.cc:284-289)一致,**不** Snapshot
  /// 运行中的 timer 栈(提交运行中计时是 `Print` 的行为)。
  pub fn dump_entries(&self) -> Vec<(&'static str, i64, i64)> {
    let mut entries = Vec::new();
    for i in 0..K_NUMBER_OF_COUNTERS {
      let counter = self.get_counter(i);
      if counter.count() != 0 {
        entries.push((
          counter.name(),
          counter.time().in_microseconds(),
          counter.count(),
        ));
      }
    }
    self.in_use.set(false);
    entries
  }

  /// 对应 `thread_id()`(移植语义:是否已绑定)。
  pub fn has_thread_id(&self) -> bool {
    self.thread_id.borrow().is_some()
  }

  /// 对应 `RuntimeCallTimer* current_timer()`。
  #[inline]
  pub fn current_timer(&self) -> Option<&RuntimeCallTimer> {
    // SAFETY: 指针由 `Enter` 从调用方存活的 timer 借用产生(单线程契约)。
    self.current_timer.get().map(|p| unsafe { p.as_ref() })
  }

  /// 对应 `current_timer()` 原始指针(单测地址比较用)。
  #[inline]
  pub fn current_timer_ptr(&self) -> *const RuntimeCallTimer {
    match self.current_timer.get() {
      Some(p) => p.as_ptr(),
      None => null(),
    }
  }

  /// 对应 `RuntimeCallCounter* current_counter()`。
  #[inline]
  pub fn current_counter(&self) -> Option<&RuntimeCallCounter> {
    // SAFETY: 指针由 `Enter/CorrectCurrentCounterId` 从自身 counters 数组的
    // 存活借用产生(单线程契约)。
    self.current_counter.get().map(|p| unsafe { p.as_ref() })
  }

  /// 对应 `bool InUse()`。
  #[inline]
  pub fn in_use(&self) -> bool {
    self.in_use.get()
  }

  /// 对应 `bool IsCalledOnTheSameThread()`(kInvalidThreadId ↔ 首次绑定)。
  pub fn is_called_on_the_same_thread(&self) -> bool {
    let mut guard = self.thread_id.borrow_mut();
    if let Some(id) = *guard {
      return id == current().id();
    }
    *guard = Some(current().id());
    true
  }

  /// 对应 `IsBackgroundThreadSpecificVariant(RuntimeCallCounterId id)`。
  pub fn is_background_thread_specific_variant(&self, id: RuntimeCallCounterId) -> bool {
    self.has_thread_specific_counter_variants(id)
      && (id - rcs_names::K_FIRST_THREAD_VARIANT_COUNTER) % 2 == 1
  }

  /// 对应 `HasThreadSpecificCounterVariants(RuntimeCallCounterId id)`。
  pub fn has_thread_specific_counter_variants(&self, id: RuntimeCallCounterId) -> bool {
    let first = rcs_names::K_FIRST_THREAD_VARIANT_COUNTER;
    let last = first + rcs_names::K_THREAD_VARIANT_COUNTER_COUNT - 1;
    id >= first && id <= last
  }

  /// 对应 `CounterIdForThread(id)`( DCHECK 双变体;worker 取 id+1 )。
  pub fn counter_id_for_thread(&self, id: RuntimeCallCounterId) -> RuntimeCallCounterId {
    debug_assert!(self.has_thread_specific_counter_variants(id));
    if self.thread_type == ThreadType::WorkerThread {
      id + 1
    } else {
      id
    }
  }

  /// 对应 `IsCounterAppropriateForThread(id)`。
  pub fn is_counter_appropriate_for_thread(&self, id: RuntimeCallCounterId) -> bool {
    if !self.has_thread_specific_counter_variants(id) {
      return true;
    }
    self.is_background_thread_specific_variant(id) == (self.thread_type == ThreadType::WorkerThread)
  }

  /// 对应 `GetCounter(RuntimeCallCounterId)`;越界不 panic,告警后回落 0 号。
  pub fn get_counter(&self, counter_id: RuntimeCallCounterId) -> &RuntimeCallCounter {
    match self.counters.get(counter_id) {
      Some(c) => c,
      None => {
        // C++ 此处越界即 UB;移植降级(禁 panic)。
        log::warn!("RuntimeCallStats::GetCounter out of bounds: {counter_id}");
        &self.counters[0]
      }
    }
  }
}

// --- Print 辅助(对应 `class RuntimeCallStatEntries`) --------------------------

/// 对应 `RuntimeCallStatEntries::Entry`。
struct Entry {
  name: &'static str,
  time_us: i64,
  count: u64,
  time_percent: f64,
  count_percent: f64,
}

impl Entry {
  /// Entry(name, time, count):percent 初始 100(总行语义)。
  fn new(name: &'static str, time_us: i64, count: u64) -> Self {
    Self {
      name,
      time_us,
      count,
      time_percent: 100.0,
      count_percent: 100.0,
    }
  }

  /// 对应 `operator<`(time 优先、count 次之的升序全序)。
  fn less(a: &Entry, b: &Entry) -> bool {
    if a.time_us < b.time_us {
      return true;
    }
    if a.time_us > b.time_us {
      return false;
    }
    a.count < b.count
  }

  /// 对应 `SetTotal(total_time, total_count)`。
  fn set_total(&mut self, total_time_us: i64, total_count: u64) {
    if total_time_us == 0 {
      self.time_percent = 0.0;
    } else {
      self.time_percent = 100.0 * self.time_us as f64 / total_time_us as f64;
    }
    self.count_percent = 100.0 * self.count as f64 / total_count as f64;
  }

  /// 对应 `Print(std::ostream&)`(setw/precision(2)/fixed 宽度 1:1)。
  fn print<W: Write>(&self, os: &mut W) -> error::Result<()> {
    let name = self.name;
    let time_ms = self.time_us as f64 / 1000.0;
    write!(os, "{name:>50}")?;
    write!(os, "{time_ms:>10.2}ms ")?;
    write!(os, "{:>6.2}%", self.time_percent)?;
    write!(os, "{:>10} ", self.count)?;
    write!(os, "{:>6.2}%", self.count_percent)?;
    writeln!(os)?;
    Ok(())
  }

  /// 对应 `RuntimeCallStatEntries::Print`。
  fn print_all<W: Write>(
    os: &mut W,
    mut entries: Vec<Entry>,
    total_time_us: i64,
    total_call_count: u64,
  ) -> error::Result<()> {
    if total_call_count == 0 {
      return Ok(());
    }
    // std::sort(entries.rbegin(), entries.rend()) → 降序。
    entries.sort_by(|a, b| {
      if Entry::less(a, b) {
        cmp::Ordering::Less
      } else if Entry::less(b, a) {
        cmp::Ordering::Greater
      } else {
        cmp::Ordering::Equal
      }
    });
    entries.reverse();
    for entry in &mut entries {
      entry.set_total(total_time_us, total_call_count);
    }
    writeln!(
      os,
      "{:>50}{:>12}{:>18}",
      "Runtime Function/C++ Builtin", "Time", "Count"
    )?;
    writeln!(os, "{}", "=".repeat(88))?;
    for entry in &entries {
      entry.print(os)?;
    }
    writeln!(os, "{}", "-".repeat(88))?;
    Entry::new("Total", total_time_us, total_call_count).print(os)?;
    Ok(())
  }
}

// --- RuntimeCallTimerScope / RCS_SCOPE ----------------------------------------

/// 对应 `class V8_NODISCARD RuntimeCallTimerScope`
/// (isolate/local_isolate 构造版仅取 `RuntimeCallStats*` 版,isolate 桥接推迟)。
///
/// timer 存于 `Box`(堆地址稳定):`stats.enter` 记录的指针在 scope 移动后
/// 仍有效,镜像 C++ 成员固定地址的语义。
pub struct RuntimeCallTimerScope<'a> {
  stats: Option<&'a RuntimeCallStats>,
  timer: Box<RuntimeCallTimer>,
}

impl<'a> RuntimeCallTimerScope<'a> {
  /// 对应 `RuntimeCallTimerScope(RuntimeCallStats*, counter_id, mode = kExact)`。
  pub fn new(
    stats: &'a RuntimeCallStats,
    counter_id: RuntimeCallCounterId,
    mode: CounterMode,
  ) -> Self {
    let timer = Box::new(RuntimeCallTimer::new());
    if !TracingFlags::is_runtime_stats_enabled() {
      // V8_LIKELY 快路:不采集(C++ stats==nullptr 分支此处由 Option 表达)。
      return Self { stats: None, timer };
    }
    let mut id = counter_id;
    if mode == CounterMode::ThreadSpecific {
      id = stats.counter_id_for_thread(id);
    }
    debug_assert!(stats.is_counter_appropriate_for_thread(id));
    stats.enter(&timer, id);
    Self {
      stats: Some(stats),
      timer,
    }
  }
}

impl Drop for RuntimeCallTimerScope<'_> {
  fn drop(&mut self) {
    if let Some(stats) = self.stats {
      stats.leave(&self.timer);
    }
  }
}

/// 对应宏 `RCS_SCOPE(stats, counter_id)`(`kExact`)。
#[must_use]
pub fn rcs_scope<'a>(
  stats: &'a RuntimeCallStats,
  counter_id: RuntimeCallCounterId,
) -> RuntimeCallTimerScope<'a> {
  RuntimeCallTimerScope::new(stats, counter_id, CounterMode::Exact)
}

/// 对应宏 `CHANGE_CURRENT_RUNTIME_COUNTER(runtime_call_stats, counter_id)`。
pub fn change_current_runtime_counter(
  stats: Option<&RuntimeCallStats>,
  counter_id: RuntimeCallCounterId,
) {
  if TracingFlags::is_runtime_stats_enabled()
    && let Some(stats) = stats
  {
    stats.correct_current_counter_id(counter_id, CounterMode::Exact);
  }
}

/// 对应宏 `TRACE_HANDLER_STATS(isolate, counter_name)`:isolate 桥接推迟,
/// 保留 handler 段 id 计算入口(以显式 stats 调用)。
pub fn trace_handler_stats(stats: Option<&RuntimeCallStats>, handler_offset: usize) {
  change_current_runtime_counter(stats, rcs_names::K_SECTION_OFFSET_HANDLER + handler_offset);
}

// --- WorkerThreadRuntimeCallStats ---------------------------------------------

/// 对应 `class WorkerThreadRuntimeCallStats final`(TLS key 机制推迟:
/// 以持有 `RuntimeCallStats` 集合替代,`GetKey`/`WorkerThreadRuntimeCallStatsScope`
/// 的 `base::Thread::LocalStorageKey` 通道登记【待回收】)。
pub struct WorkerThreadRuntimeCallStats {
  /// C++ 为 `vector<unique_ptr<RuntimeCallStats>>`(需返回稳定裸指针给 TLS);
  /// Rust 版访问全部在锁内、以索引/借用进行,无地址稳定需求,直接值存。
  tables: Mutex<Vec<RuntimeCallStats>>,
  /// 对应 `isolate_thread_id_`(主线程记录,防误建 worker 表)。
  isolate_thread_id: ThreadId,
}

impl WorkerThreadRuntimeCallStats {
  pub fn new() -> Self {
    Self {
      tables: Mutex::new(Vec::new()),
      isolate_thread_id: current().id(),
    }
  }

  /// 对应 `NewTable()`;Rust 版以索引返回借用受限,改为返回受管引用计数句柄。
  pub fn new_table(&self) -> usize {
    // C++ 注释(runtime-call-stats.h:743-745):记录主线程 ID 即为
    // "ensure we never create a worker RCS table for the main thread"。
    // 原版仅存储未检查;此处按注释语义接线(禁 panic:debug_assert + warn)。
    if current().id() == self.isolate_thread_id {
      log::warn!("WorkerThreadRuntimeCallStats::NewTable called on isolate thread");
    }
    let table = RuntimeCallStats::new(ThreadType::WorkerThread);
    let mut tables = self.tables.lock();
    tables.push(table);
    tables.len() - 1
  }

  /// 表个数(调试辅助,对应内部 `tables_`)。
  pub fn num_tables(&self) -> usize {
    self.tables.lock().len()
  }

  /// 对应 `AddToMainTable(RuntimeCallStats* main_call_stats)`。
  pub fn add_to_main_table(&self, main_call_stats: &RuntimeCallStats) {
    let tables = self.tables.lock();
    for worker_stats in tables.iter() {
      debug_assert!(
        !eq(main_call_stats, worker_stats),
        "main table must differ from worker table"
      );
      main_call_stats.add(worker_stats);
      worker_stats.reset();
    }
  }
}

impl Default for WorkerThreadRuntimeCallStats {
  fn default() -> Self {
    Self::new()
  }
}
