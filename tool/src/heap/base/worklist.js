#!/usr/bin/env bun
// tool/src/heap/base/worklist.js — worklist.rs 初版生成
// 组→文件映射 (本脚本管辖):
//   v8/src/heap/base/worklist.h + worklist.cc → v8-heap-base/src/worklist.rs
// 规则:
//   - 方法清单锚点断言 (对数核对: 原版 Worklist/Segment/Local/WorklistBase 全方法)
//   - 常量/内存序语义机械提取登记进注释
//   - 直出签名骨架 + 常量表; 语义 (段链/发布/窃取) 就地修订
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/heap/base/worklist.h");
const cc = readV8("src/heap/base/worklist.cc");

// ---- 锚点断言: 方法清单对数 ----
const expect = {
  Worklist: ["IsEmpty", "Size", "Merge", "Clear", "Update", "Iterate", "Push", "Pop"],
  Segment: ["Create", "Delete", "Push", "Pop", "Update", "Iterate", "next", "set_next", "Size", "Capacity", "IsEmpty", "IsFull", "Clear"],
  Local: ["Push", "Pop", "IsLocalAndGlobalEmpty", "IsLocalEmpty", "IsGlobalEmpty", "PushSegmentSize", "Publish", "Merge", "Clear", "PublishPushSegment", "PublishPopSegment", "StealPopSegment", "NewSegment", "DeleteSegment"],
  WorklistBase: ["EnforcePredictableOrder", "PredictableOrder"],
};
for (const [cls, fns] of Object.entries(expect)) {
  for (const fn of fns) {
    const re = new RegExp(`\\b${fn}\\s*\\(`);
    if (!re.test(h)) throw new Error(`worklist.h: ${cls}::${fn} 缺失`);
  }
}
// worklist.cc: 全局开关 + 哨兵段
if (!/predictable_order_ = false/.test(cc)) throw new Error("worklist.cc: predictable_order_ 初值缺失");
if (!/GetSentinelSegmentAddress/.test(cc)) throw new Error("worklist.cc: GetSentinelSegmentAddress 缺失");

// ---- 常量提取: 内存序 (fetch_add/fetch_sub/store/exchange 均 relaxed) ----
const orders = [...new Set([...h.matchAll(/std::memory_order_(\w+)/g)].map((m) => m[1]))];
if (JSON.stringify(orders) !== JSON.stringify(["relaxed"]))
  throw new Error(`worklist.h 内存序应为全 relaxed, 实得 ${orders}`);

const body = `${banner("src/heap/base/worklist.js", "src/heap/base/worklist.h + worklist.cc", "并发工作队列: 段链 + 线程本地视图; 原版 v8::base::Mutex/哨兵段 → Rust Mutex/Option<Box<Segment>>")}
//! Worklist: 段(segment)式全局工作队列, 线程本地生产/消费 + 全局窃取。
//! 原版内存序: size_ 计数全程 memory_order_relaxed (对数核对通过: ${orders.join(",")});
//! 段链表拓扑由 lock_ 全量互斥保护, 故 relaxed 计数只作近似读。

use std::{marker::PhantomData, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};

use v8_base_platform::Mutex;

/// 原 WorklistBase (worklist.cc:10-13): 全局可预测序开关 (单线程模式 push/pop 顺序可预期)。
pub struct WorklistBase;

static PREDICTABLE_ORDER: AtomicBool = AtomicBool::new(false);

impl WorklistBase {
  /// 原 EnforcePredictableOrder (worklist.cc:13)。
  pub fn enforce_predictable_order() {
    PREDICTABLE_ORDER.store(true, Ordering::Relaxed);
  }
  /// 原 PredictableOrder。
  pub fn predictable_order() -> bool {
    PREDICTABLE_ORDER.load(Ordering::Relaxed)
  }
}

/// 段: 有界条目栈 + 侵入式 next 链。
pub struct Segment<T> {
  next: Option<Box<Segment<T>>>,
  entries: Vec<T>,
}

impl<T> Segment<T> {
  /// 原 Segment::Create (worklist.h:218-233)。
  /// 偏差: 原版 AllocateAtLeast 超配容量优化省略, 恒按 min_segment_size 精确分配。
  pub fn create(min_segment_size: usize) -> Box<Self> {
    let mut entries = Vec::with_capacity(min_segment_size);
    entries.shrink_to_fit();
    Box::new(Self { next: None, entries })
  }
  pub fn size(&self) -> usize { self.entries.len() }
  pub fn capacity(&self) -> usize { self.entries.capacity() }
  pub fn is_empty(&self) -> bool { self.entries.is_empty() }
  pub fn is_full(&self) -> bool { self.entries.len() == self.entries.capacity() }
  /// 原 Clear: index_ = 0。
  pub fn clear(&mut self) { self.entries.clear() }
  pub fn push(&mut self, entry: T) { debug_assert!(!self.is_full()); self.entries.push(entry); }
  pub fn pop(&mut self) -> Option<T> { debug_assert!(!self.is_empty()); self.entries.pop() }
  /// 原 Update (worklist.h:283-291): 回调 (源条目, 目的槽) -> 是否保留; 槽内压缩。
  pub fn update(&mut self, callback: impl FnMut(&mut T, &mut T) -> bool) { todo!("就地修订: split_at_mut 压缩") }
  /// 原 Iterate (worklist.h:294-300)。
  pub fn iterate(&self, mut callback: impl FnMut(&T)) { for e in &self.entries { callback(e); } }
}

/// 全局工作队列: 只统计已发布段。
pub struct Worklist<T, const MIN_SEGMENT_SIZE: usize> {
  lock: Mutex,
  top: Option<Box<Segment<T>>>,
  size: AtomicUsize,
  _pd: PhantomData<()>,
}

impl<T, const MIN_SEGMENT_SIZE: usize> Worklist<T, MIN_SEGMENT_SIZE> {
  pub const K_MIN_SEGMENT_SIZE: usize = MIN_SEGMENT_SIZE;

  pub fn new() -> Self { todo!("就地修订") }
  /// 原 Push (worklist.h:106-112): 头插 + relaxed 计数。
  fn push_segment(&mut self, segment: Box<Segment<T>>) { todo!("就地修订") }
  /// 原 Pop (worklist.h:114-123)。
  fn pop_segment(&mut self) -> Option<Box<Segment<T>>> { todo!("就地修订") }
  /// 原 IsEmpty (worklist.h:126-128)。
  pub fn is_empty(&self) -> bool { todo!("就地修订") }
  /// 原 Size (worklist.h:130-136): 无锁近似读。
  pub fn size(&self) -> usize { todo!("就地修订") }
  /// 原 Merge (worklist.h:188-212): other 全部段搬入 this, other 置空。
  pub fn merge(&mut self, other: &mut Self) { todo!("就地修订") }
  /// 原 Clear (worklist.h:139-149)。
  pub fn clear(&mut self) { todo!("就地修订") }
  /// 原 Update (worklist.h:152-177): 全段压缩, 空段释放。
  pub fn update(&mut self, callback: impl FnMut(&mut T, &mut T) -> bool) { todo!("就地修订") }
  /// 原 Iterate (worklist.h:179-186)。
  pub fn iterate(&self, callback: impl FnMut(&T)) { todo!("就地修订") }
}

/// 线程本地视图: 未发布的 push/pop 段对外不可见。
pub struct Local<'a, T, const MIN_SEGMENT_SIZE: usize> {
  worklist: &'a Worklist<T, MIN_SEGMENT_SIZE>,
  push_segment: Option<Box<Segment<T>>>,
  pop_segment: Option<Box<Segment<T>>>,
}

impl<'a, T, const MIN_SEGMENT_SIZE: usize> Local<'a, T, MIN_SEGMENT_SIZE> {
  pub type ItemType = T; // 占位, 修订时改 associated type 用法
  /// 原 Local (worklist.h:377-382)。
  pub fn new(worklist: &'a Worklist<T, MIN_SEGMENT_SIZE>) -> Self { todo!("就地修订") }
  /// 原 Push (worklist.h:392-399): push 段满则先发布再换新段。
  pub fn push(&mut self, entry: T) { todo!("就地修订") }
  /// 原 Pop (worklist.h:401-413): 本地段优先, 空则窃取全局段。
  pub fn pop(&mut self) -> Option<T> { todo!("就地修订") }
  pub fn is_local_and_global_empty(&self) -> bool { todo!("就地修订") }
  pub fn is_local_empty(&self) -> bool { todo!("就地修订") }
  pub fn is_global_empty(&self) -> bool { todo!("就地修订") }
  /// 原 PushSegmentSize (worklist.h:330)。
  pub fn push_segment_size(&self) -> usize { todo!("就地修订") }
  /// 原 Publish (worklist.h:429-439): 非空段全部发布, 视图回哨兵态。
  pub fn publish(&mut self) { todo!("就地修订") }
  /// 原 Merge (worklist.h:441-445): 合并的是两个全局池。
  pub fn merge(&mut self, other: &mut Self) { todo!("就地修订") }
  /// 原 Clear (worklist.h:473-477): 清空本地段条目。
  pub fn clear(&mut self) { todo!("就地修订") }
  fn publish_push_segment(&mut self) { todo!("就地修订") }
  fn publish_pop_segment(&mut self) { todo!("就地修订") }
  /// 原 StealPopSegment (worklist.h:461-471)。
  fn steal_pop_segment(&mut self) -> bool { todo!("就地修订") }
}
`;

const p = writeSrc("v8-heap-base", "worklist.rs", body);
console.log(`worklist.js: ${p} (方法锚点 ${Object.values(expect).flat().length} 项核对通过, 内存序 ${orders.join("/")})`);
