//! 内存均衡器 (MemoryBalancer)。
//! 对应原版 v8/src/heap/memory-balancer.h (105 行) + memory-balancer.cc (115 行)。
//!
//! 收编说明 (W10HCS, skel-F 第二批挂载, R3 整改):
//! - 前置 `TimeDelta`/`TimeTicks` 已落地 (v8-base-platform time.rs:51/:607, lib.rs:54 再导出,
//!   in_milliseconds_f time.rs:175), 非「依赖未落」; UpdateAllocationRate (cc:74-84) 与
//!   UpdateGCSpeed (cc:63-72) 方法体零 `heap_` 引用系纯 EWMA 函数 → 本波 1:1 填体。
//! - 推迟面 (readme skel-F 台账): HeartbeatUpdate (cc:86-100) 与 RecomputeLimits (cc:17-25)
//!   真依赖 Heap 运行态 (`Heap::OldGenerationSizeOfObjects` h:59 消费 + `Heap::limits()`
//!   cc:44/:46/:59 + `GetForegroundTaskRunner` cc:105), RefreshLimit (cc:27-61) /
//!   PostHeartbeatTask (cc:102-107) / HeartbeatTask (h:87-97 `CancelableTask` 任务面 +
//!   cc:109-112) 不入本票签名面 → 归 heap-gc / Heap 实体票接入。

use v8_base_platform::{TimeDelta, TimeTicks};

use crate::memory_allocator::Heap;

// ══════════════════════════════════════════════════════════════════════════
// SmoothedBytesAndDuration (h:36-50 private 嵌套类, 全 pub 承接)
// ══════════════════════════════════════════════════════════════════════════

/// EWMA 字节/时长平滑对 (h:36 `class SmoothedBytesAndDuration`)。
/// x' = x * decay + new * (1 - decay) (h:40-44); rate() = bytes / duration (bytes/ms, h:46)。
#[derive(Debug, Clone, Copy)]
pub struct SmoothedBytesAndDuration {
  /// h:48 `double bytes_`。
  bytes: f64,
  /// h:49 `double duration_`。
  duration: f64,
}

impl SmoothedBytesAndDuration {
  /// h:38 构造: bytes_ = static_cast<double>(bytes)。
  #[must_use]
  pub fn new(bytes: usize, duration: f64) -> Self {
    Self {
      bytes: bytes as f64,
      duration,
    }
  }

  /// h:40-44 `Update(bytes, duration, decay_rate)` — 双量同率 EWMA。
  pub fn update(&mut self, bytes: usize, duration: f64, decay_rate: f64) {
    self.bytes = self.bytes * decay_rate + (bytes as f64) * (1.0 - decay_rate);
    self.duration = self.duration * decay_rate + duration * (1.0 - decay_rate);
  }

  /// h:46 `rate()` — bytes/ms。
  #[must_use]
  pub fn rate(&self) -> f64 {
    self.bytes / self.duration
  }
}

// ══════════════════════════════════════════════════════════════════════════
// MemoryBalancer (h:23-85 + cc:14-107)
// ══════════════════════════════════════════════════════════════════════════

/// 内存均衡器 (h:23 `class MemoryBalancer`) — 监听分配/GC 事件, EWMA 平滑分配率与
/// GC 速度, 心跳测差分并重算旧生代限额。
pub struct MemoryBalancer {
  /// h:59 `Heap* heap_` — opaque 承接位 (实体 memory_allocator.rs:61), 仅推迟体消费。
  pub heap: *mut Heap,
  /// h:63 `size_t live_memory_after_gc_ = 0` — 上次 major GC 的存活内存估计。
  pub live_memory_after_gc: usize,
  /// h:74 `size_t embedder_allocation_limit_ = 0` — embedder 限额 (RefreshLimit 拼合面)。
  pub embedder_allocation_limit: usize,
  /// h:77 `std::optional<SmoothedBytesAndDuration> major_allocation_rate_`。
  pub major_allocation_rate: Option<SmoothedBytesAndDuration>,
  /// h:78 `std::optional<SmoothedBytesAndDuration> major_gc_speed_`。
  pub major_gc_speed: Option<SmoothedBytesAndDuration>,
  /// h:82 `size_t last_measured_memory_ = 0` — 心跳差分基准。
  pub last_measured_memory: usize,
  /// h:83 `base::TimeTicks last_measured_at_` — 心跳差分时刻基准。
  pub last_measured_at: TimeTicks,
  /// h:84 `bool heartbeat_task_started_ = false`。
  pub heartbeat_task_started: bool,
}

impl MemoryBalancer {
  /// h:53 `static constexpr double kMajorAllocationDecayRate = 0.95`。
  pub const K_MAJOR_ALLOCATION_DECAY_RATE: f64 = 0.95;
  /// h:54 `static constexpr double kMajorGCDecayRate = 0.5`。
  pub const K_MAJOR_GC_DECAY_RATE: f64 = 0.5;

  /// 原版 h:25 声明 + cc:14-15 `MemoryBalancer(Heap*, TimeTicks)` —
  /// `heap_(heap), last_measured_at_(startup_time)` (零解引用, 其余字段按 h 缺省初值)。
  /// v8: src/heap/memory-balancer.cc:14 MemoryBalancer::MemoryBalancer
  #[must_use]
  pub fn new(heap: *mut Heap, startup_time: TimeTicks) -> Self {
    Self {
      heap,
      live_memory_after_gc: 0,
      embedder_allocation_limit: 0,
      major_allocation_rate: None,
      major_gc_speed: None,
      last_measured_memory: 0,
      last_measured_at: startup_time,
      heartbeat_task_started: false,
    }
  }

  /// 原版 h:27-28 `UpdateAllocationRate(bytes, duration)` — 首次直构, 否则 EWMA
  /// (cc:76-82; 方法体零 `heap_` 引用, 纯函数面)。
  /// v8: src/heap/memory-balancer.cc:74 MemoryBalancer::UpdateAllocationRate
  pub fn update_allocation_rate(
    &mut self,
    major_allocation_bytes: usize,
    major_allocation_duration: TimeDelta,
  ) {
    if self.major_allocation_rate.is_none() {
      // cc:76-78: 首次 = SmoothedBytesAndDuration{bytes, InMillisecondsF()}。
      self.major_allocation_rate = Some(SmoothedBytesAndDuration::new(
        major_allocation_bytes,
        major_allocation_duration.in_milliseconds_f(),
      ));
    } else if let Some(rate) = &mut self.major_allocation_rate {
      // cc:79-82: 否则 Update(bytes, InMillisecondsF(), kMajorAllocationDecayRate)。
      rate.update(
        major_allocation_bytes,
        major_allocation_duration.in_milliseconds_f(),
        Self::K_MAJOR_ALLOCATION_DECAY_RATE,
      );
    }
  }

  /// 原版 h:29 `UpdateGCSpeed(bytes, duration)` — 首次直构, 否则 EWMA
  /// (cc:65-71; 方法体零 `heap_` 引用, 纯函数面)。
  /// v8: src/heap/memory-balancer.cc:63 MemoryBalancer::UpdateGCSpeed
  pub fn update_gc_speed(&mut self, major_gc_bytes: usize, major_gc_duration: TimeDelta) {
    if self.major_gc_speed.is_none() {
      // cc:65-67: 首次 = SmoothedBytesAndDuration{bytes, InMillisecondsF()}。
      self.major_gc_speed = Some(SmoothedBytesAndDuration::new(
        major_gc_bytes,
        major_gc_duration.in_milliseconds_f(),
      ));
    } else if let Some(speed) = &mut self.major_gc_speed {
      // cc:68-71: 否则 Update(bytes, InMillisecondsF(), kMajorGCDecayRate)。
      speed.update(
        major_gc_bytes,
        major_gc_duration.in_milliseconds_f(),
        Self::K_MAJOR_GC_DECAY_RATE,
      );
    }
  }

  /// 原版 h:31 `HeartbeatUpdate()` — 心跳差分推分配率 (置 started/Now/差分/Refresh/Post)。
  /// v8: src/heap/memory-balancer.cc:86 MemoryBalancer::HeartbeatUpdate
  pub fn heartbeat_update(&mut self) {
    // 台账 (readme skel-F): cc:89 heap_->OldGenerationSizeOfObjects + cc:88
    // base::TimeTicks::Now + cc:98/:99 RefreshLimit/PostHeartbeatTask 系 Heap 运行态。
    todo!(
      "就地修订: cc:86-100 (Heap::OldGenerationSizeOfObjects/limits + TaskRunner 运行态, heap-gc 票)"
    )
  }

  /// 原版 h:33 `RecomputeLimits(embedder_allocation_limit, time)` — 重测基准并重算限额。
  /// v8: src/heap/memory-balancer.cc:17 MemoryBalancer::RecomputeLimits
  pub fn recompute_limits(&mut self, embedder_allocation_limit: usize, time: TimeTicks) {
    // 台账 (readme skel-F): cc:21 heap_->OldGenerationSizeOfObjects + cc:23/:24
    // RefreshLimit (cc:27-61 heap_->limits() 三处)/PostHeartbeatTask 系 Heap 运行态;
    // embedder_allocation_limit/time 承接 cc:19-22 存储位, 填体票消费。
    let _ = (embedder_allocation_limit, time);
    todo!("就地修订: cc:17-25 (Heap::OldGenerationSizeOfObjects/limits 运行态, heap-gc 票)")
  }

  /// 承接 h:59 存储位的读出口 (原版无独立 getter, 推迟体经 this->heap_ 直读)。
  #[must_use]
  pub fn heap(&self) -> *mut Heap {
    self.heap
  }
}
