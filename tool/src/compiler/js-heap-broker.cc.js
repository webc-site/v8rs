#!/usr/bin/env bun
// tool/src/compiler/js-heap-broker.cc.js
// 从 v8/src/compiler/js-heap-broker.h + js-heap-broker.cc
// 差集视角: broker 本体已由 v8-compiler-broker 冻结, 本脚本只产前端消费的
// 反馈访问薄面 (HasFeedback/GetFeedback/GetFeedbackSlotKind/FeedbackIsInsufficient)
// 生成 v8-compiler-frontend/src/js_heap_broker.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-heap-broker.h");
const cc = readV8("src/compiler/js-heap-broker.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class V8_EXPORT_PRIVATE JSHeapBroker {", "broker 主类"],
  ["bool HasFeedback(FeedbackSource const& source) const;", "HasFeedback 声明"],
  ["FeedbackSlotKind GetFeedbackSlotKind(FeedbackSource const& source) const;", "GetFeedbackSlotKind 声明"],
  ["ProcessedFeedback const& GetFeedback(FeedbackSource const& source) const;", "GetFeedback 声明"],
  ["if (HasFeedback(source)) return GetFeedback(source).slot_kind();", "slot kind 双路径"],
  ["return FeedbackNexus(source.vector, source.slot, feedback_nexus_config())", "FeedbackNexus 回退路径"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit) && !cc.includes(lit)) {
    throw new Error(`js-heap-broker 锚点缺失: ${name}`);
  }
}

// —— 2. cc 定义面计数 (对数基准) ——
const defs = [...new Set(
  [...cc.matchAll(/^\w[\w:<>*&, ]*\bJSHeapBroker::(\w+)\(/gm)].map((m) => m[1]),
)];

const body = `${banner("src/compiler/js-heap-broker.cc.js", "src/compiler/js-heap-broker.h + js-heap-broker.cc", "broker 薄镜像: 前端反馈访问面 (本体在 v8-compiler-broker)")}

/// 前端反馈访问面 — 委托冻结 broker crate, FeedbackNexus 回退路径推迟
/// v8: src/compiler/js-heap-broker.h JSHeapBroker (差集)
pub struct BrokerFeedbackAccess;

impl BrokerFeedbackAccess {
  /// HasFeedback — processed feedback 快照存在性
  /// v8: src/compiler/js-heap-broker.cc JSHeapBroker::HasFeedback
  pub fn has_feedback<'z>(
    broker: &v8_compiler_broker::JSHeapBroker<'z, '_>,
    source: &crate::feedback_source::FeedbackSource,
  ) -> bool {
    broker.get_feedback(source).is_some()
  }

  /// GetFeedback — processed 反馈快照
  /// v8: src/compiler/js-heap-broker.cc JSHeapBroker::GetFeedback
  pub fn get_feedback<'z>(
    broker: &v8_compiler_broker::JSHeapBroker<'z, '_>,
    source: &crate::feedback_source::FeedbackSource,
  ) -> Option<&'z v8_compiler_broker::ProcessedFeedbackSnapshot<'z>> {
    broker.get_feedback(source)
  }

  /// GetFeedbackSlotKind — 有快照走快照, 否则 FeedbackNexus 实读
  /// v8: src/compiler/js-heap-broker.cc JSHeapBroker::GetFeedbackSlotKind
  pub fn get_feedback_slot_kind<'z>(
    _broker: &v8_compiler_broker::JSHeapBroker<'z, '_>,
    _source: &crate::feedback_source::FeedbackSource,
  ) -> crate::Result<v8_objects_rest::feedback_vector::FeedbackSlotKind> {
    todo!() // 推迟台账: FeedbackNexus (v8-objects-rest) 实读面
  }

  /// FeedbackIsInsufficient — 快照 IsInsufficient 或 nexus IsUninitialized
  /// v8: src/compiler/js-heap-broker.cc JSHeapBroker::FeedbackIsInsufficient
  pub fn feedback_is_insufficient<'z>(
    _broker: &v8_compiler_broker::JSHeapBroker<'z, '_>,
    _source: &crate::feedback_source::FeedbackSource,
  ) -> bool {
    todo!() // 推迟台账: FeedbackNexus::IsUninitialized
  }
}

// —— 对数基准 ——
// cc 定义面 ${defs.length} 项 (本体归 v8-compiler-broker, 前端取用 ${anchors.length / 2 | 0} 条访问面)
`;

const p = writeSrc("v8-compiler-frontend", "js_heap_broker.rs.skel", body);
console.log(`OK ${p} defs=${defs.length}`);
