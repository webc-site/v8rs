#!/usr/bin/env bun
// tool/src/heap/base/bytes.h.js — bytes.rs 初版生成
// 组→文件映射 (本脚本管辖):
//   v8/src/heap/base/bytes.h → v8-heap-base/src/bytes.rs
//   v8/src/base/ring-buffer.h → 同文件内嵌 RingBuffer (v8-base 未落地, 待回收, 见 README)
// 规则:
//   - RingBuffer 容量/Reduce 顺序机械提取并断言
//   - AverageSpeed / SmoothedBytesAndDuration 签名与公式骨架
import { readV8, writeSrc, banner } from "../../../lib.js";

const bytesH = readV8("src/heap/base/bytes.h");
const ringH = readV8("src/base/ring-buffer.h");

// ---- 锚点断言 ----
for (const sym of ["BytesAndDuration", "AverageSpeed", "SmoothedBytesAndDuration", "ByteSize", "StrongAlias"]) {
  if (!bytesH.includes(sym)) throw new Error(`bytes.h: ${sym} 缺失`);
}
// RingBuffer 默认容量 10
const kSize = ringH.match(/template <typename T, uint8_t _SIZE = (\d+)>/);
if (!kSize || kSize[1] !== "10") throw new Error(`ring-buffer.h 默认容量应为 10, 实得 ${kSize}`);
// Reduce 双段顺序: 先 [pos-1, 0], 后 [kSize-1, pos] (最新→最旧)
if (!/for \(uint8_t i = pos_; i > 0; --i\)/.test(ringH)) throw new Error("ring-buffer.h: Reduce 前段顺序不符");
if (!/for \(uint8_t i = kSize; i > pos_; --i\)/.test(ringH)) throw new Error("ring-buffer.h: Reduce 后段顺序不符");

const body = `${banner("src/heap/base/bytes.h.js", "src/heap/base/bytes.h + src/base/ring-buffer.h", "吞吐统计: 字节/时长事件环形缓冲 + 平均速度 + 平滑吞吐")}
//! bytes.h: GC 吞吐统计基元。
//! 附带内嵌 RingBuffer (原 v8/src/base/ring-buffer.h, v8-base 未落地前就地承载)。

use v8_base_platform::TimeDelta;

/// 原 v8::base::RingBuffer (ring-buffer.h): 定长环形缓冲, 满后覆盖最旧。
/// Reduce 从最新到最旧归约 (对数核对通过)。
pub struct RingBuffer<T, const SIZE: usize = 10> {
  elements: [Option<T>; SIZE],
  pos: u8,
  is_full: bool,
}

impl<T, const SIZE: usize> RingBuffer<T, SIZE> {
  pub const K_SIZE: usize = SIZE;
  pub fn new() -> Self { todo!("就地修订: elements None 填充") }
  /// 原 Push: 写入后指针回绕, 满标志置位。
  pub fn push(&mut self, value: T) { todo!("就地修订") }
  /// 原 Size: is_full ? kSize : pos。
  pub fn size(&self) -> usize { todo!("就地修订") }
  pub fn empty(&self) -> bool { todo!("就地修订") }
  /// 原 Clear: 只重置指针不析构元素 (Rust 侧同步清 Option)。
  pub fn clear(&mut self) { todo!("就地修订") }
  /// 原 Reduce: 先 elements[pos-1..0], 满则再 elements[SIZE-1..pos]。
  pub fn reduce(&self, callback: impl FnMut(T, &T) -> T, initial: T) -> T { todo!("就地修订") }
}

/// 原 BytesAndDuration (bytes.h:19-26)。
pub struct BytesAndDuration {
  pub bytes: usize,
  pub duration: TimeDelta,
}

/// 原 BytesAndDurationBuffer (bytes.h:28)。
pub type BytesAndDurationBuffer = RingBuffer<BytesAndDuration>;

/// 原 AverageSpeed (bytes.h:35-58): 含 initial 事件的平均速度 (Bytes/ms);
/// selected_duration 限定纳入的事件时长窗口; 结果夹在 [min_non_empty_speed, max_speed]。
pub fn average_speed(
  buffer: &BytesAndDurationBuffer,
  initial: &BytesAndDuration,
  selected_duration: Option<TimeDelta>,
  min_non_empty_speed: usize,
  max_speed: usize,
) -> Option<f64> {
  todo!("就地修订: sum=reduce 夹逼聚合; duration 为零返回 None")
}

/// 原 SmoothedBytesAndDuration (bytes.h:60-102): 半衰平滑吞吐。
pub struct SmoothedBytesAndDuration {
  throughput: f64,
  increase_decay: TimeDelta,
  decrease_decay: TimeDelta,
}

impl SmoothedBytesAndDuration {
  pub fn new(increase_decay: TimeDelta, decrease_decay: TimeDelta) -> Self { todo!("就地修订") }
  /// 原 Update: 时长为零忽略; 吞吐升降分别取 increase/decrease 衰减率。
  pub fn update(&mut self, bytes_and_duration: BytesAndDuration) { todo!("就地修订") }
  /// 原 GetThroughput: bytes/ms。
  pub fn throughput(&self) -> f64 { todo!("就地修订") }
  /// 原 GetThroughputForTesting: 以 0 吞吐假设检验 decrease_decay 衰减。
  pub fn throughput_for_testing(&self, delay: TimeDelta) -> f64 { todo!("就地修订") }
  /// 原 Decay: throughput * exp2(-delay_ms / decay_ms)。
  fn decay(&self, throughput: f64, delay: TimeDelta, decay: TimeDelta) -> f64 { todo!("就地修订") }
}

/// 原 ByteSize (bytes.h:104): StrongAlias<ByteSizeTag, size_t> → newtype。
pub struct ByteSize(pub usize);
`;

const p = writeSrc("v8-heap-base", "bytes.rs", body);
console.log(`bytes.h.js: ${p} (锚点 5 符号 + RingBuffer 默认容量 ${kSize[1]} + Reduce 双段顺序核对通过)`);
