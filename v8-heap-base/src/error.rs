// 错误集中定义 (原版 CHECK/DCHECK 失败语义 → Result, 禁 panic)。

use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  /// 原 BasicSlotSet::SlotToIndices CHECK(bucket < num_buckets)
  /// (basic-slot-set.h:464)。
  #[error("槽偏移越界: bucket {bucket} >= 桶数 {buckets}")]
  SlotBucketOutOfRange { bucket: usize, buckets: usize },
  /// 原 RemoveRange CHECK_LE(end_offset, buckets * kBitsPerBucket * SlotGranularity)
  /// (basic-slot-set.h:166)。
  #[error("区间越界: end {end} 超出 {buckets} 桶容量")]
  RemoveRangeOutOfBounds { end: usize, buckets: usize },
  /// 原 IteratePointersInStack CHECK 族 (stack.cc:129-138):
  /// top/start 空、start < top、未按 sizeof(void*) 对齐。
  #[error("栈段无效: {what}")]
  InvalidStackSegment { what: &'static str },
}

pub type Result<T> = result::Result<T, Error>;
