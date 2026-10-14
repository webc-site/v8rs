//! broker 前端薄镜像 — 反馈访问面
//!
//! JSHeapBroker 本体冻结于 v8-compiler-broker; 本模块只 1:1 承接
//! v8/src/compiler/js-heap-broker.cc 中前端消费的反馈查询函数。
//! 依赖 FeedbackNexus 的实读回退路径推迟 (登记台账)。

use v8_compiler_broker::{JSHeapBroker, ProcessedFeedbackSnapshot};
use v8_objects_rest::feedback_vector::FeedbackSlotKind;

use crate::feedback_source::FeedbackSource;

/// 前端反馈访问面
/// v8: src/compiler/js-heap-broker.h JSHeapBroker (差集)
pub struct BrokerFeedbackAccess;

impl BrokerFeedbackAccess {
  /// HasFeedback — processed 快照存在性
  /// v8: src/compiler/js-heap-broker.cc JSHeapBroker::HasFeedback
  pub fn has_feedback<'z>(broker: &JSHeapBroker<'z, '_>, source: &FeedbackSource) -> bool {
    broker.has_feedback(&(*source).into())
  }

  /// GetFeedback — 取 processed 反馈快照 (原版 CHECK(HasFeedback))
  /// v8: src/compiler/js-heap-broker.cc JSHeapBroker::GetFeedback
  pub fn get_feedback<'z>(
    broker: &'z JSHeapBroker<'z, '_>,
    source: &FeedbackSource,
  ) -> Option<&'z ProcessedFeedbackSnapshot<'z>> {
    broker.get_feedback(&(*source).into())
  }

  /// GetFeedbackSlotKind — 有快照走快照, 否则 FeedbackNexus 实读
  /// v8: src/compiler/js-heap-broker.cc JSHeapBroker::GetFeedbackSlotKind
  pub fn get_feedback_slot_kind<'z>(
    _broker: &JSHeapBroker<'z, '_>,
    _source: &FeedbackSource,
  ) -> crate::Result<FeedbackSlotKind> {
    todo!() // 推迟台账: ProcessedFeedback::slot_kind + FeedbackNexus 实读面
  }

  /// FeedbackIsInsufficient — 快照 IsInsufficient 或 nexus IsUninitialized
  /// v8: src/compiler/js-heap-broker.cc JSHeapBroker::FeedbackIsInsufficient
  pub fn feedback_is_insufficient<'z>(
    _broker: &JSHeapBroker<'z, '_>,
    _source: &FeedbackSource,
  ) -> bool {
    todo!() // 推迟台账: ProcessedFeedback::IsInsufficient + FeedbackNexus
  }

  /// NewInsufficientFeedback — zone 上构造 InsufficientFeedback
  /// v8: src/compiler/js-heap-broker.cc JSHeapBroker::NewInsufficientFeedback
  pub fn new_insufficient_feedback<'z>(
    _broker: &JSHeapBroker<'z, '_>,
    _kind: FeedbackSlotKind,
  ) -> crate::Result<&'z ProcessedFeedbackSnapshot<'z>> {
    todo!() // 推迟台账: InsufficientFeedback 构造面
  }
}
