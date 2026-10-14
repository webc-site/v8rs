#!/usr/bin/env bun
// tool/src/heap/base/basic-slot-set.h.js — basic_slot_set.rs 初版生成
// 组→文件映射 (本脚本管辖):
//   v8/src/heap/base/basic-slot-set.h → v8-heap-base/src/basic_slot_set.rs
// 规则:
//   - 位标志常量表机械提取并交叉验算 (kBitsPerBucket = kCellsPerBucket * kBitsPerCell 等)
//   - 枚举三组 (SlotCallbackResult/AccessMode/EmptyBucketMode) 机械翻译
//   - 原版 CHECK (槽偏移越界) → Result<_, Error::SlotBucketOutOfRange> (禁 panic)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/heap/base/basic-slot-set.h");

// ---- 常量提取 + 交叉验算 ----
const int = (name) => {
  const m = h.match(new RegExp(`(static constexpr int ${name})\\s*=\\s*([^;]+);`));
  if (!m) throw new Error(`basic-slot-set.h: 常量 ${name} 提取失败`);
  const expr = m[2].replace(/k(\w+)/g, (_, k) => `k${k}`);
  return { name, expr: m[2] };
};
const consts = {};
for (const n of ["kCellsPerBucket", "kCellsPerBucketLog2", "kCellSizeBytesLog2", "kCellSizeBytes", "kBitsPerCell", "kBitsPerCellLog2"]) {
  const m = h.match(new RegExp(`static constexpr int ${n} = ([^;]+);`));
  if (!m) throw new Error(`常量 ${n} 提取失败`);
  consts[n] = m[1].trim();
}
{
  const m = h.match(/static constexpr int kBitsPerBucket = ([^;]+);/);
  if (!m) throw new Error("kBitsPerBucket 提取失败");
  if (m[1].trim() !== "kCellsPerBucket * kBitsPerCell") throw new Error(`kBitsPerBucket 表达式不符: ${m[1]}`);
  consts.kBitsPerBucket = "kCellsPerBucket * kBitsPerCell";
  const m2 = h.match(/static constexpr int kBitsPerBucketLog2 =([\s\S]*?);/);
  if (!m2 || !m2[1].includes("kCellsPerBucketLog2 + kBitsPerCellLog2")) throw new Error("kBitsPerBucketLog2 表达式不符");
  consts.kBitsPerBucketLog2 = "kCellsPerBucketLog2 + kBitsPerCellLog2";
}
// 数值验算: 项形如 "数字"、"1 << 常量"、"常量", 以 + 连接
const evalConst = (e) => {
  const sub = (s) => s
    .replace(/kCellsPerBucketLog2/g, "5").replace(/kCellsPerBucket/g, "32")
    .replace(/kCellSizeBytesLog2/g, "2")
    .replace(/kBitsPerCellLog2/g, "5").replace(/kBitsPerCell/g, "32")
    .replace(/kBitsPerBucketLog2/g, "10");
  const term = (t) => {
    const s = sub(t.trim());
    if (!/^[\d\s|<>*]+$/.test(s)) return Number(s);
    const sh = s.match(/^(\d+)\s*(?:<<|\*)\s*(\d+)$/);
    // 纯数字直接返回; 仅数字与 <</*/ 的表达式安全求值
    if (/^\d+$/.test(s)) return Number(s);
    if (!sh) throw new Error(`非法项: ${s}`);
    return s.includes("<<") ? Number(sh[1]) << Number(sh[2]) : Number(sh[1]) * Number(sh[2]);
  };
  return e.split("+").reduce((a, s) => a + term(s), 0);
};
const expectVals = { kCellsPerBucket: 32, kCellsPerBucketLog2: 5, kCellSizeBytesLog2: 2, kCellSizeBytes: 4, kBitsPerCell: 32, kBitsPerCellLog2: 5, kBitsPerBucket: 1024, kBitsPerBucketLog2: 10 };
for (const [k, v] of Object.entries(expectVals)) {
  const got = evalConst(consts[k]);
  if (got !== v) throw new Error(`常量 ${k} 验算失败: 期望 ${v}, 实得 ${got}`);
}

// ---- 锚点断言: 方法清单 ----
for (const fn of ["Allocate", "Delete", "BucketsForSize", "BucketForSlot", "OffsetForBucket", "Insert", "Contains", "Remove", "RemoveRange", "Lookup", "Iterate", "SlotToIndices", "LoadBucket", "StoreBucket", "SwapInNewBucket", "ReleaseBucket", "ClearBucket", "FreeBucketIfEmpty"]) {
  if (!new RegExp(`\\b${fn}\\s*\\(`).test(h)) throw new Error(`basic-slot-set.h: ${fn} 缺失`);
}
for (const fn of ["LoadCell", "SetCellBits", "ClearCellBits", "StoreCell", "IsEmpty"]) {
  if (!new RegExp(`\\b${fn}\\s*\\(`).test(h)) throw new Error(`basic-slot-set.h: Bucket::${fn} 缺失`);
}

// ---- 枚举提取 ----
if (!/enum SlotCallbackResult \{ KEEP_SLOT, REMOVE_SLOT \}/.test(h)) throw new Error("SlotCallbackResult 枚举不符");
if (!/FREE_EMPTY_BUCKETS/.test(h) || !/KEEP_EMPTY_BUCKETS/.test(h)) throw new Error("EmptyBucketMode 枚举不符");

const body = `${banner("src/heap/base/basic-slot-set.h.js", "src/heap/base/basic-slot-set.h", "两级位图槽集合: bucket(cell(bitmap32)) — GC 记录集核心")}
//! BasicSlotSet: 槽集合, 假设槽按 SlotGranularity 对齐, 偏移域切分为 bucket,
//! 每 bucket 32 cell × 32 bit 位图。
//! 原版分配在指针前藏 num_buckets 头 (写屏障快路径); Rust 侧 buckets.len() 承载。
//! 原版 AsAtomic32 load+store 竞态写 → Rust fetch_or/fetch_and RMW (语义增强, 消丢更新)。

use std::{ptr, sync::atomic::{AtomicPtr, AtomicU32, Ordering}};

use crate::error::{Error, Result};

/// 原 SlotCallbackResult (basic-slot-set.h:22)。
pub enum SlotCallbackResult {
  KeepSlot,
  RemoveSlot,
}

/// 原 AccessMode (basic-slot-set.h:37-40): bucket 级是否允许并发访问。
pub enum AccessMode {
  Atomic,
  NonAtomic,
}

/// 原 EmptyBucketMode (basic-slot-set.h:42-45)。
pub enum EmptyBucketMode {
  /// 空桶立即释放。
  FreeEmptyBuckets,
  /// 空桶保留。
  KeepEmptyBuckets,
}

/// 位标志常量表 (机械提取 + 交叉验算通过):
/// kCellsPerBucket=${consts.kCellsPerBucket} kCellsPerBucketLog2=${consts.kCellsPerBucketLog2}
/// kCellSizeBytesLog2=${consts.kCellSizeBytesLog2} kCellSizeBytes=${consts.kCellSizeBytes}
/// kBitsPerCell=${consts.kBitsPerCell} kBitsPerCellLog2=${consts.kBitsPerCellLog2}
/// kBitsPerBucket=${consts.kBitsPerBucket}=${expectVals.kBitsPerBucket} kBitsPerBucketLog2=${consts.kBitsPerBucketLog2}=${expectVals.kBitsPerBucketLog2}
pub const K_CELLS_PER_BUCKET: usize = ${expectVals.kCellsPerBucket};
pub const K_CELLS_PER_BUCKET_LOG2: usize = ${expectVals.kCellsPerBucketLog2};
pub const K_BITS_PER_CELL: usize = ${expectVals.kBitsPerCell};
pub const K_BITS_PER_CELL_LOG2: usize = ${expectVals.kBitsPerCellLog2};
pub const K_BITS_PER_BUCKET: usize = ${expectVals.kBitsPerBucket};
pub const K_BITS_PER_BUCKET_LOG2: usize = ${expectVals.kBitsPerBucketLog2};

/// 原 Bucket (basic-slot-set.h:287-338): 32 × u32 原子 cell。
struct Bucket {
  cells: [AtomicU32; K_CELLS_PER_BUCKET],
}

impl Bucket {
  /// 原 LoadCell<ATOMIC>: Acquire。
  fn load_cell(&self, cell_index: usize, mode: AccessMode) -> u32 { todo!("就地修订") }
  /// 原 SetCellBits: ATOMIC → fetch_or; NON_ATOMIC → 普通置位。
  fn set_cell_bits(&self, cell_index: usize, mask: u32, mode: AccessMode) { todo!("就地修订") }
  /// 原 ClearCellBits: ATOMIC → fetch_and !mask。
  fn clear_cell_bits(&self, cell_index: usize, mask: u32, mode: AccessMode) { todo!("就地修订") }
  /// 原 StoreCell: Release 整字写。
  fn store_cell(&self, cell_index: usize, value: u32) { todo!("就地修订") }
  /// 原 IsEmpty: 32 cell 全零。
  fn is_empty(&self) -> bool { todo!("就地修订") }
}

/// 原 BasicSlotSet<SlotGranularity>: 槽粒度对齐的两级位图。
pub struct BasicSlotSet<const SLOT_GRANULARITY: usize> {
  /// 桶指针数组 (Box::into_raw 挂载; null = 桶缺省)。
  buckets: Vec<AtomicPtr<Bucket>>,
}

impl<const SLOT_GRANULARITY: usize> BasicSlotSet<SLOT_GRANULARITY> {
  /// 原 Allocate (basic-slot-set.h:49-75): buckets 个桶全 null。
  pub fn allocate(buckets: usize) -> Box<Self> { todo!("就地修订") }
  /// 原 BucketsForSize (basic-slot-set.h:89-92): 上取整。
  pub const fn buckets_for_size(size: usize) -> usize { todo!("就地修订") }
  /// 原 BucketForSlot (basic-slot-set.h:95-98)。
  pub const fn bucket_for_slot(slot_offset: usize) -> usize { todo!("就地修订") }
  /// 原 OffsetForBucket (basic-slot-set.h:101-103)。
  pub const fn offset_for_bucket(bucket_index: usize) -> usize { todo!("就地修订") }
  /// 原 Insert (basic-slot-set.h:108-134): 缺桶 CAS 补挂, 置位。
  pub fn insert(&self, mode: AccessMode, slot_offset: usize) -> Result<()> { todo!("就地修订") }
  /// 原 Contains (basic-slot-set.h:138-145)。
  pub fn contains(&self, slot_offset: usize) -> Result<bool> { todo!("就地修订") }
  /// 原 Remove (basic-slot-set.h:148-160)。
  pub fn remove(&self, slot_offset: usize) -> Result<()> { todo!("就地修订") }
  /// 原 RemoveRange (basic-slot-set.h:164-238): 区间清位, 越界 CHECK → Err。
  pub fn remove_range(&self, start_offset: usize, end_offset: usize, buckets: usize, mode: EmptyBucketMode) -> Result<()> { todo!("就地修订") }
  /// 原 Lookup (basic-slot-set.h:241-248)。
  pub fn lookup(&self, slot_offset: usize) -> Result<bool> { todo!("就地修订") }
  /// 原 Iterate (basic-slot-set.h:265-275 + 345-385): 回调 KEEP_SLOT 保留 / REMOVE_SLOT 清位;
  /// 返回保留槽总数; FREE_EMPTY_BUCKETS 释放空桶。
  pub fn iterate(&self, chunk_start: usize, start_bucket: usize, end_bucket: usize, callback: impl FnMut(usize) -> SlotCallbackResult, mode: EmptyBucketMode) -> usize { todo!("就地修订") }
  /// 原 SlotToIndices (basic-slot-set.h:458-468): 桶越界 CHECK → Err。
  fn slot_to_indices(&self, slot_offset: usize) -> Result<(usize, usize, usize)> { todo!("就地修订") }
  /// 原 ReleaseBucket (basic-slot-set.h:411-415)。
  fn release_bucket(&self, bucket_index: usize) { todo!("就地修订") }
  /// 原 LoadBucket / StoreBucket (basic-slot-set.h:418-442): ATOMIC → Acquire/Release。
  fn load_bucket(&self, bucket_index: usize, mode: AccessMode) -> Option<&Bucket> { todo!("就地修订") }
  /// 原 SwapInNewBucket (basic-slot-set.h:445-455): CAS(null→value)。
  fn swap_in_new_bucket(&self, bucket_index: usize, value: &Bucket) -> bool { todo!("就地修订") }
  /// 原 ClearBucket (basic-slot-set.h:400-408): 自 start_cell 清到桶尾
  /// (原版 end_cell 参数被循环忽略, 调用点恒传 kCellsPerBucket, 予以省并)。
  fn clear_bucket(bucket: &Bucket, from_cell: usize) { todo!("就地修订") }
}
`;

const p = writeSrc("v8-heap-base", "basic_slot_set.rs", body);
console.log(`basic-slot-set.h.js: ${p} (常量 8 项验算 + 方法 23 项锚点核对通过)`);
