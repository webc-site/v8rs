// 由 tool/src/heap/base/bytes.h.js 从 v8/src/heap/base/bytes.h + src/base/ring-buffer.h 生成; 修订就地改, 复跑覆盖
// 吞吐统计: 字节/时长事件环形缓冲 + 平均速度 + 平滑吞吐

//! bytes.h: GC 吞吐统计基元。
//! 附带内嵌 RingBuffer (原 v8/src/base/ring-buffer.h, v8-base 未落地前就地承载)。

use std::array::from_fn;

use v8_base_platform::TimeDelta;

/// 原 v8::base::RingBuffer (ring-buffer.h): 定长环形缓冲, 满后覆盖最旧。
/// Reduce 从最新到最旧归约 (对数核对通过)。
pub struct RingBuffer<T, const SIZE: usize = 10> {
  elements: [Option<T>; SIZE],
  pos: usize,
  is_full: bool,
}

impl<T, const SIZE: usize> RingBuffer<T, SIZE> {
  pub const K_SIZE: usize = SIZE;

  pub fn new() -> Self {
    Self {
      elements: from_fn(|_| None),
      pos: 0,
      is_full: false,
    }
  }

  /// 原 Push (ring-buffer.h:23-28): 写入后指针回绕, 满标志置位。
  pub fn push(&mut self, value: T) {
    self.elements[self.pos] = Some(value);
    self.pos += 1;
    if self.pos == SIZE {
      self.pos = 0;
      self.is_full = true;
    }
  }

  /// 原 Size (ring-buffer.h:30): is_full ? kSize : pos。
  pub fn size(&self) -> usize {
    if self.is_full { SIZE } else { self.pos }
  }

  /// 原 Empty (ring-buffer.h:32)。
  pub fn empty(&self) -> bool {
    self.size() == 0
  }

  /// 原 Clear (ring-buffer.h:34-37): 只重置指针不析构元素
  /// (Rust 侧旧值保留在不可达槽位, 新写入时覆盖 — 与原版"不调析构"等价)。
  pub fn clear(&mut self) {
    self.pos = 0;
    self.is_full = false;
  }

  /// 原 Reduce (ring-buffer.h:39-50): 先 elements[pos-1..0] (最新→旧),
  /// 满则再 elements[SIZE-1..pos]。
  pub fn reduce(&self, mut callback: impl FnMut(T, &T) -> T, initial: T) -> T {
    let mut result = initial;
    for i in (0..self.pos).rev() {
      if let Some(e) = &self.elements[i] {
        result = callback(result, e);
      }
    }
    if self.is_full {
      for i in (self.pos..SIZE).rev() {
        if let Some(e) = &self.elements[i] {
          result = callback(result, e);
        }
      }
    }
    result
  }
}

impl<T, const SIZE: usize> Default for RingBuffer<T, SIZE> {
  fn default() -> Self {
    Self::new()
  }
}

/// 原 BytesAndDuration (bytes.h:19-26)。
#[derive(Clone, Copy)]
pub struct BytesAndDuration {
  pub bytes: usize,
  pub duration: TimeDelta,
}

impl BytesAndDuration {
  /// 原 constexpr 构造 (bytes.h:21-22)。
  pub const fn new(bytes: usize, duration: TimeDelta) -> Self {
    Self { bytes, duration }
  }
}

impl Default for BytesAndDuration {
  /// 原 constexpr 默认构造: bytes=0, duration 零时长。
  fn default() -> Self {
    Self {
      bytes: 0,
      duration: TimeDelta::new(),
    }
  }
}

/// 原 BytesAndDurationBuffer (bytes.h:28)。
pub type BytesAndDurationBuffer = RingBuffer<BytesAndDuration>;

/// 聚合谓词: selected_duration 窗口外事件跳过 (原 AverageSpeed lambda, bytes.h:40-48)。
fn accumulate(
  a: BytesAndDuration,
  b: &BytesAndDuration,
  selected_duration: Option<TimeDelta>,
) -> BytesAndDuration {
  if selected_duration.is_some_and(|sel| a.duration >= sel) {
    return a;
  }
  BytesAndDuration {
    bytes: a.bytes + b.bytes,
    duration: a.duration + b.duration,
  }
}

/// 原 AverageSpeed (bytes.h:35-58) 的无界形态: min=0 / max=usize::MAX,
/// 含 initial 事件的平均速度 (Bytes/ms); 时长为零返回 None。
pub fn average_speed_unbounded(
  buffer: &BytesAndDurationBuffer,
  initial: &BytesAndDuration,
  selected_duration: Option<TimeDelta>,
) -> Option<f64> {
  let sum = buffer.reduce(|a, b| accumulate(a, b, selected_duration), *initial);
  if sum.duration.is_zero() {
    return None;
  }
  Some(sum.bytes as f64 / sum.duration.in_milliseconds_f())
}

/// 原 AverageSpeed (bytes.h:35-58): 含 initial 事件的平均速度 (Bytes/ms);
/// selected_duration 限定纳入的事件时长窗口; 结果夹在 [min_non_empty_speed, max_speed]。
pub fn average_speed(
  buffer: &BytesAndDurationBuffer,
  initial: &BytesAndDuration,
  selected_duration: Option<TimeDelta>,
  min_non_empty_speed: usize,
  max_speed: usize,
) -> Option<f64> {
  let speed = average_speed_unbounded(buffer, initial, selected_duration)?;
  // 原 std::max(std::min(speed, max), min) — min > max 时取 min, 无 panic。
  Some(speed.min(max_speed as f64).max(min_non_empty_speed as f64))
}

/// 原 SmoothedBytesAndDuration (bytes.h:60-102): 半衰平滑吞吐。
pub struct SmoothedBytesAndDuration {
  throughput: f64,
  increase_decay: TimeDelta,
  decrease_decay: TimeDelta,
}

impl SmoothedBytesAndDuration {
  /// 原构造 (bytes.h:62-64)。
  pub fn new(increase_decay: TimeDelta, decrease_decay: TimeDelta) -> Self {
    Self {
      throughput: 0.0,
      increase_decay,
      decrease_decay,
    }
  }

  /// 原 Update (bytes.h:66-78): 时长为零忽略; 吞吐升降分别取 increase/decrease 衰减率。
  pub fn update(&mut self, bytes_and_duration: BytesAndDuration) {
    if bytes_and_duration.duration.is_zero() {
      return;
    }
    let new_throughput =
      bytes_and_duration.bytes as f64 / bytes_and_duration.duration.in_milliseconds_f();
    let decay = if new_throughput >= self.throughput {
      self.increase_decay
    } else {
      self.decrease_decay
    };
    self.throughput = new_throughput
      + self.decay(
        self.throughput - new_throughput,
        bytes_and_duration.duration,
        decay,
      );
  }

  /// 原 GetThroughput (bytes.h:79): bytes/ms。
  pub fn throughput(&self) -> f64 {
    self.throughput
  }

  /// 原 GetThroughputForTesting (bytes.h:84-86): 以 0 吞吐假设检验 decrease_decay 衰减。
  pub fn throughput_for_testing(&self, delay: TimeDelta) -> f64 {
    self.decay(self.throughput, delay, self.decrease_decay)
  }

  /// 原 Decay (bytes.h:89-93): throughput * exp2(-delay_ms / decay_ms)。
  fn decay(&self, throughput: f64, delay: TimeDelta, decay: TimeDelta) -> f64 {
    throughput * (-delay.in_milliseconds_f() / decay.in_milliseconds_f()).exp2()
  }
}

/// 原 ByteSize (bytes.h:104): StrongAlias<ByteSizeTag, size_t> → newtype。
pub struct ByteSize(pub usize);
